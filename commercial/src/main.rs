use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use indicatif::{MultiProgress, ProgressStyle};
use log::{info, warn, error};
use std::sync::Arc;
use tokio::sync::RwLock;

mod compiler;
mod config;
mod file_detector;
mod language_support;
mod args;
mod appimage;
mod enterprise;
mod watcher;
mod analytics;

use compiler::ProCompiler;
use config::ProConfig;
use file_detector::ProFileDetector;
use args::ProArgs;
use appimage::ProAppImageBuilder;
use enterprise::EnterpriseManager;
use watcher::FileWatcher;
use analytics::Analytics;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    let args = ProArgs::parse();
    
    // Initialize enterprise features
    let enterprise_manager = Arc::new(RwLock::new(EnterpriseManager::new()));
    
    // Load professional configuration
    let config = ProConfig::load().context("Failed to load professional configuration")?;
    
    // Validate project path
    if !args.project_path.exists() {
        anyhow::bail!("Project path does not exist: {:?}", args.project_path);
    }
    
    if !args.project_path.is_dir() {
        anyhow::bail!("Project path is not a directory: {:?}", args.project_path);
    }

    // Display professional branding
    display_professional_branding();
    println!("📁 Project: {:?}", args.project_path);
    
    // Check if we're creating an AppImage
    if let Some(app_name) = &args.name {
        println!("🎯 Creating Professional AppImage: {}", app_name.bold().green());
        return create_professional_appimage(&args, &config, app_name, enterprise_manager).await;
    }
    
    // Check for enterprise features
    if args.enterprise {
        println!("🏢 Enterprise Mode: {}", "ENABLED".bold().blue());
        enterprise_manager.write().await.activate_enterprise_features(&args)?;
    }
    
    println!("🔧 Parallel jobs: {}", args.jobs);
    if args.watch {
        println!("👀 Watch mode: {}", "ENABLED".bold().yellow());
    }
    println!();

    // Initialize analytics
    let analytics = Arc::new(Analytics::new());
    
    // Detect source files with professional detection
    let file_detector = ProFileDetector::new();
    let source_files = file_detector.detect_files(&args.project_path, &args, &config)?;

    if source_files.is_empty() {
        println!("{} No source files found to compile.", "⚠️".yellow());
        return Ok(());
    }

    // Display detected files with enhanced information
    display_enhanced_file_info(&source_files, &args);

    // Initialize progress bars with professional styling
    let multi_progress = MultiProgress::new();
    let progress_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("█▉▊▋▌▍▎▏");

    // Compile files with professional compiler
    let compiler = ProCompiler::new(config, args.jobs, enterprise_manager.clone());
    let results = compiler
        .compile_all(source_files, &multi_progress, &progress_style, &args)
        .await?;

    // Record analytics
    analytics.record_compilation_session(&results).await;

    // Display professional results
    display_professional_results(&results, args.verbose);

    // Start file watcher if requested
    if args.watch {
        println!("\n👀 Starting file watcher...");
        let watcher = FileWatcher::new(args.project_path.clone(), args.clone());
        watcher.start_watching().await?;
    }

    Ok(())
}

fn display_professional_branding() {
    println!("🚀 {} - Professional Multi-language Code Compiler", "LOL PRO".bold().blue());
    println!("🏢 Enterprise Edition v1.0.0");
    println!("{}", "=".repeat(60));
}

async fn create_professional_appimage(
    args: &ProArgs, 
    config: &ProConfig, 
    app_name: &str,
    enterprise_manager: Arc<RwLock<EnterpriseManager>>
) -> Result<()> {
    println!("🔍 Professional source code analysis...");
    
    // Detect source files with enhanced detection
    let file_detector = ProFileDetector::new();
    let source_files = file_detector.detect_files(&args.project_path, args, config)?;

    if source_files.is_empty() {
        println!("{} No source files found to include in AppImage.", "⚠️".yellow());
        return Ok(());
    }

    // Display professional file analysis
    println!("📋 Professional File Analysis:");
    for (lang, files) in &source_files {
        println!("  {}: {} files", lang.name().bold(), files.len());
        if args.verbose {
            for file in files {
                if let Ok(content) = fs::read_to_string(file) {
                    let lines = content.lines().count();
                    let size = content.len();
                    println!("    {} ({} lines, {} bytes)", file.display(), lines, size);
                }
            }
        }
    }
    println!();

    // Create professional AppImage
    println!("🏗️  Building Professional AppImage...");
    let appimage_builder = ProAppImageBuilder::new(
        app_name.to_string(), 
        source_files,
        enterprise_manager.clone()
    );
    
    // Show professional source summary
    if args.verbose {
        println!("{}", appimage_builder.get_professional_summary());
    }
    
    let appimage_path = appimage_builder.build_professional().await?;
    
    println!("✅ Professional AppImage created successfully!");
    println!("📦 Output: {}", appimage_path.display());
    println!("🔒 Enterprise Features: {}", "ACTIVATED".bold().green());
    println!("\n🚀 You can now run your Professional AppImage:");
    println!("   ./{}", appimage_path.file_name().unwrap().to_string_lossy());
    
    Ok(())
}

fn display_enhanced_file_info(source_files: &HashMap<Language, Vec<PathBuf>>, args: &ProArgs) {
    println!("📋 Enhanced Source File Detection:");
    let mut total_files = 0;
    let mut total_lines = 0;
    
    for (lang, files) in source_files {
        total_files += files.len();
        println!("  {}: {} files", lang.name().bold(), files.len());
        
        if args.verbose {
            for file in files {
                if let Ok(content) = fs::read_to_string(file) {
                    let lines = content.lines().count();
                    total_lines += lines;
                    let size = content.len();
                    println!("    {} ({} lines, {} bytes)", file.display(), lines, size);
                }
            }
        }
    }
    
    println!("📊 Summary: {} files, {} total lines", total_files, total_lines);
    println!();
}

fn display_professional_results(results: &[compiler::ProCompilationResult], verbose: bool) {
    println!("\n📊 Professional Compilation Results:");
    println!("{}", "=".repeat(60));

    let mut total_files = 0;
    let mut successful_compilations = 0;
    let mut failed_compilations = 0;
    let mut warnings = 0;

    for result in results {
        total_files += result.files.len();
        
        match &result.status {
            compiler::ProCompilationStatus::Success { output, warnings: result_warnings } => {
                successful_compilations += result.files.len();
                warnings += result_warnings;
                println!("✅ {}: {} files compiled successfully", 
                    result.language.name().bold().green(), 
                    result.files.len()
                );
                if verbose && !output.is_empty() {
                    println!("   Output: {}", output);
                }
                if *result_warnings > 0 {
                    println!("   ⚠️  {} warnings", result_warnings);
                }
            }
            compiler::ProCompilationStatus::Failure { error, suggestions } => {
                failed_compilations += result.files.len();
                println!("❌ {}: {} files failed to compile", 
                    result.language.name().bold().red(), 
                    result.files.len()
                );
                if verbose {
                    println!("   Error: {}", error);
                    if !suggestions.is_empty() {
                        println!("   💡 Suggestions:");
                        for suggestion in suggestions {
                            println!("     - {}", suggestion);
                        }
                    }
                }
            }
        }
    }

    println!("{}", "=".repeat(60));
    println!("📈 Professional Summary:");
    println!("  Total files: {}", total_files);
    println!("  Successful: {} {}", successful_compilations, "✅".green());
    println!("  Failed: {} {}", failed_compilations, "❌".red());
    println!("  Warnings: {} {}", warnings, "⚠️".yellow());
    
    if failed_compilations == 0 {
        println!("\n🎉 {} All files compiled successfully!", "SUCCESS".bold().green());
        if warnings > 0 {
            println!("💡 Consider addressing {} warnings for production code.", warnings);
        }
    } else {
        println!("\n⚠️  {} files failed to compile. Check the output above for details.", failed_compilations);
        std::process::exit(1);
    }
} 