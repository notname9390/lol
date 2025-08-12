use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Output;
use std::sync::Arc;
use anyhow::{Context, Result};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use tokio::sync::Semaphore;
use crate::config::Config;
use crate::language_support::Language;
use crate::args::Args;
use std::process::Command;
use crate::language_support::LanguageSupport;

pub struct Compiler {
    max_jobs: usize,
}

#[derive(Debug)]
pub struct CompilationResult {
    pub language: Language,
    pub files: Vec<PathBuf>,
    pub status: CompilationStatus,
}

#[derive(Debug)]
pub enum CompilationStatus {
    Success { output: String },
    Failure { error: String },
}

impl Compiler {
    pub fn new(_config: Config, max_jobs: usize) -> Self {
        Self {
            max_jobs,
        }
    }

    pub async fn compile_all(
        &self,
        source_files: HashMap<Language, Vec<PathBuf>>,
        multi_progress: &MultiProgress,
        progress_style: &ProgressStyle,
        args: &Args,
    ) -> Result<Vec<CompilationResult>> {
        let semaphore = Arc::new(Semaphore::new(self.max_jobs));
        let mut results = Vec::new();

        // Create progress bars for each language
        let mut progress_bars: HashMap<Language, ProgressBar> = HashMap::new();
        
        for (language, files) in &source_files {
            let progress_bar = multi_progress.add(ProgressBar::new(files.len() as u64));
            progress_bar.set_style(progress_style.clone());
            progress_bar.set_message(format!("Compiling {} files...", language.name()));
            progress_bars.insert(language.clone(), progress_bar);
        }

        // Compile each language group
        for (language, files) in source_files {
            let progress_bar = progress_bars.get(&language).unwrap().clone();
            let semaphore = Arc::clone(&semaphore);
            let custom_flags = self.get_custom_flags(&language, args);
            
            let result = self.compile_language_group(
                language.clone(),
                files,
                &semaphore,
                &progress_bar,
                custom_flags,
            ).await;

            results.push(result);
        }

        // Wait for all progress bars to finish
        multi_progress.clear().unwrap();

        Ok(results)
    }

    async fn compile_language_group(
        &self,
        language: Language,
        files: Vec<PathBuf>,
        semaphore: &Arc<Semaphore>,
        progress_bar: &ProgressBar,
        custom_flags: Option<String>,
    ) -> CompilationResult {
        let mut successful_files = Vec::new();
        let mut failed_files = Vec::new();
        let mut compilation_output = String::new();
        let mut compilation_errors = String::new();

        // Process files in parallel with semaphore limiting concurrency
        let file_results: Vec<_> = files
            .par_iter()
            .map(|file| {
                let semaphore = Arc::clone(semaphore);
                let custom_flags = custom_flags.clone();
                let language_clone = language.clone();
                
                async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    self.compile_single_file(&language_clone, file, custom_flags.as_deref()).await
                }
            })
            .collect();

        // Wait for all compilations to complete
        for (file, result) in files.iter().zip(file_results) {
            let result = result.await;
            
            match result {
                Ok(output) => {
                    successful_files.push(file.clone());
                    if !output.is_empty() {
                        compilation_output.push_str(&format!("{}: {}\n", file.display(), output));
                    }
                }
                Err(error) => {
                    failed_files.push(file.clone());
                    compilation_errors.push_str(&format!("{}: {}\n", file.display(), error));
                }
            }
            
            progress_bar.inc(1);
        }

        progress_bar.finish_with_message(format!("Finished compiling {} files", language.name()));

        // Determine overall result
        let status = if failed_files.is_empty() {
            CompilationStatus::Success {
                output: compilation_output,
            }
        } else {
            CompilationStatus::Failure {
                error: compilation_errors,
            }
        };

        CompilationResult {
            language,
            files: successful_files,
            status,
        }
    }

    async fn compile_single_file(
        &self,
        language: &Language,
        file: &PathBuf,
        custom_flags: Option<&str>,
    ) -> Result<String> {
        let mut command = language
            .get_compilation_command(file, custom_flags)
            .context("Failed to create compilation command")?;

        // Execute compilation
        let output = command
            .output()
            .context("Failed to execute compilation command")?;

        if output.status.success() {
            Ok(self.format_output(&output))
        } else {
            Err(anyhow::anyhow!("Compilation failed: {}", self.format_error(&output)))
        }
    }

    fn get_custom_flags(&self, language: &Language, args: &Args) -> Option<String> {
        match language {
            Language::C => args.cflags.clone(),
            Language::Cpp => args.cxxflags.clone(),
            _ => None,
        }
    }

    fn format_output(&self, output: &Output) -> String {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        let mut result = String::new();
        if !stdout.is_empty() {
            result.push_str(&stdout);
        }
        if !stderr.is_empty() {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(&stderr);
        }
        result
    }

    fn format_error(&self, output: &Output) -> String {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        let mut result = String::new();
        if !stderr.is_empty() {
            result.push_str(&stderr);
        } else if !stdout.is_empty() {
            result.push_str(&stdout);
        } else {
            result.push_str("Unknown compilation error");
        }
        result
    }

    pub fn check_compilers_available(&self) -> HashMap<Language, bool> {
        let mut availability = HashMap::new();
        
        for language in LanguageSupport::new().get_available_languages() {
            availability.insert(language.clone(), language.check_compiler_available());
        }
        
        availability
    }

    pub fn get_compiler_info(&self) -> HashMap<Language, String> {
        let mut info = HashMap::new();
        
        for language in LanguageSupport::new().get_available_languages() {
            if language.needs_compiler_check() {
                let (compiler, args) = language.get_compiler_command();
                if let Ok(output) = Command::new(compiler).args(args).output() {
                    let version = String::from_utf8_lossy(&output.stdout);
                    info.insert(language.clone(), version.trim().to_string());
                } else {
                    info.insert(language.clone(), "Not available".to_string());
                }
            } else {
                info.insert(language.clone(), "Built-in".to_string());
            }
        }
        
        info
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_compiler_creation() {
        let config = Config::default();
        let compiler = Compiler::new(config, 4);
        
        assert_eq!(compiler.max_jobs, 4);
    }

    #[test]
    fn test_compiler_availability_check() {
        let config = Config::default();
        let compiler = Compiler::new(config, 1);
        let availability = compiler.check_compilers_available();
        
        // At least some compilers should be available on most systems
        assert!(!availability.is_empty());
    }

    #[test]
    fn test_compiler_info() {
        let config = Config::default();
        let compiler = Compiler::new(config, 1);
        let info = compiler.get_compiler_info();
        
        // Should have info for all supported languages
        assert!(!info.is_empty());
    }
} 