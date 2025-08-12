use std::collections::HashMap;
use std::path::{Path, PathBuf};
use anyhow::Result;
use walkdir::WalkDir;
use crate::language_support::{Language, LanguageSupport};
use crate::args::Args;

pub struct FileDetector {
    language_support: LanguageSupport,
}

impl FileDetector {
    pub fn new() -> Self {
        Self {
            language_support: LanguageSupport::new(),
        }
    }

    pub fn detect_files(
        &self,
        project_path: &Path,
        args: &Args,
        _config: &crate::config::Config,
    ) -> Result<HashMap<Language, Vec<PathBuf>>> {
        let mut language_files: HashMap<Language, Vec<PathBuf>> = HashMap::new();

        // Walk through the project directory recursively
        for entry in WalkDir::new(project_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            // Skip directories and hidden files
            if path.is_dir() || self.is_hidden_file(path) {
                continue;
            }

            // Get file extension
            if let Some(extension) = path.extension() {
                let ext_str = extension.to_string_lossy().to_lowercase();
                
                // Get language for this extension
                if let Some(language) = self.language_support.get_language_by_extension(&ext_str) {
                    // Check if this language should be compiled based on args
                    if self.should_compile_language(language, args) {
                        // Add file to the appropriate language group
                        language_files
                            .entry(language.clone())
                            .or_insert_with(Vec::new)
                            .push(path.canonicalize().unwrap_or_else(|_| path.to_path_buf()));
                    }
                }
            }
        }

        // Sort files within each language group
        for files in language_files.values_mut() {
            files.sort();
        }

        Ok(language_files)
    }

    fn is_hidden_file(&self, path: &Path) -> bool {
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.starts_with('.'))
            .unwrap_or(false)
    }

    fn should_compile_language(&self, language: &Language, args: &Args) -> bool {
        // If --all is specified, compile all languages
        if args.all {
            return true;
        }

        // Check if any specific language flags are set
        let has_specific_flags = args.c || args.cpp || args.python || args.java || args.rust || args.go || args.js || args.ts;

        // If no specific flags are set, compile all languages by default
        if !has_specific_flags {
            return true;
        }

        // Check specific language flags
        match language {
            Language::C => args.c,
            Language::Cpp => args.cpp,
            Language::Python => args.python,
            Language::Java => args.java,
            Language::Rust => args.rust,
            Language::Go => args.go,
            Language::JavaScript => args.js,
            Language::TypeScript => args.ts,
            // For other languages, compile them if no specific flags are set
            _ => !has_specific_flags,
        }
    }
}

impl Default for FileDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_detect_files() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();

        // Create test files
        fs::write(project_path.join("main.c"), "int main() { return 0; }").unwrap();
        fs::write(project_path.join("helper.cpp"), "#include <iostream>").unwrap();
        fs::write(project_path.join("script.py"), "print('Hello')").unwrap();
        fs::write(project_path.join(".hidden"), "hidden content").unwrap();

        let detector = FileDetector::new();
        let args = Args {
            project_path: project_path.to_path_buf(),
            c: false,
            cpp: false,
            python: false,
            java: false,
            rust: false,
            go: false,
            js: false,
            ts: false,
            all: true,
            verbose: false,
            jobs: 1,
            cflags: None,
            cxxflags: None,
            name: None,
        };

        let files = detector.detect_files(project_path, &args, &crate::config::Config::default()).unwrap();

        assert!(files.contains_key(&Language::C));
        assert!(files.contains_key(&Language::Cpp));
        assert!(files.contains_key(&Language::Python));
        assert!(!files.contains_key(&Language::Java)); // No Java files created

        // Check that hidden files are ignored
        assert!(!files.values().any(|file_list| {
            file_list.iter().any(|file| file.file_name().unwrap() == ".hidden")
        }));
    }

    #[test]
    fn test_is_hidden_file() {
        let detector = FileDetector::new();
        
        assert!(detector.is_hidden_file(Path::new(".gitignore")));
        assert!(detector.is_hidden_file(Path::new(".config")));
        assert!(!detector.is_hidden_file(Path::new("main.c")));
        assert!(!detector.is_hidden_file(Path::new("README.md")));
    }
} 