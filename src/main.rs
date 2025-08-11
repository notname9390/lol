use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use indicatif::{MultiProgress, ProgressStyle};

mod compiler;
mod config;
mod file_detector;
mod language_support;
mod args;
mod appimage;

use compiler::Compiler;
use config::Config;
use file_detector::FileDetector;
use args::Args;
use appimage::AppImageBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Load configuration
    let config = Config::load().context("Failed to load configuration")?;
    
    // Validate project path
    if !args.project_path.exists() {
        anyhow::bail!("Project path does not exist: {:?}", args.project_path);
    }
    
    if !args.project_path.is_dir() {
        anyhow::bail!("Project path is not a directory: {:?}", args.project_path);
    }

    println!("🚀 {} - Multi-language Code Compiler", "lol".bold().blue());
    println!("📁 Project: {:?}", args.project_path);
    
    // Check if we're creating an AppImage
    if let Some(app_name) = &args.name {
        println!("🎯 Creating AppImage: {}", app_name.bold().green());
        return create_appimage(&args, &config, app_name).await;
    }
    
    println!("🔧 Parallel jobs: {}", args.jobs);
    println!();

    // Detect source files
    let file_detector = FileDetector::new();
    let source_files = file_detector.detect_files(&args.project_path, &args, &config)?;

    if source_files.is_empty() {
        println!("{} No source files found to compile.", "⚠️".yellow());
        return Ok(());
    }

    // Display detected files
    println!("📋 Detected source files:");
    for (lang, files) in &source_files {
        println!("  {}: {} files", lang.name().bold(), files.len());
        if args.verbose {
            for file in files {
                println!("    {}", file.display());
            }
        }
    }
    println!();

    // Initialize progress bars
    let multi_progress = MultiProgress::new();
    let progress_style = ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("#>-");

    // Compile files
    let compiler = Compiler::new(config, args.jobs);
    let results = compiler
        .compile_all(source_files, &multi_progress, &progress_style, &args)
        .await?;

    // Display results
    display_results(&results, args.verbose);

    Ok(())
}

async fn create_appimage(args: &Args, config: &Config, app_name: &str) -> Result<()> {
    println!("🔍 Scanning for source files...");
    
    // Detect source files
    let file_detector = FileDetector::new();
    let source_files = file_detector.detect_files(&args.project_path, args, config)?;

    if source_files.is_empty() {
        println!("{} No source files found to include in AppImage.", "⚠️".yellow());
        return Ok(());
    }

    // Display what will be included
    println!("📋 Files to include in AppImage:");
    for (lang, files) in &source_files {
        println!("  {}: {} files", lang.name().bold(), files.len());
        if args.verbose {
            for file in files {
                println!("    {}", file.display());
            }
        }
    }
    println!();

    // Create AppImage
    println!("🏗️  Building AppImage...");
    let appimage_builder = AppImageBuilder::new(app_name.to_string(), source_files);
    
    // Show source summary
    if args.verbose {
        println!("{}", appimage_builder.get_source_summary());
    }
    
    let appimage_path = appimage_builder.build()?;
    
    println!("✅ AppImage created successfully!");
    println!("📦 Output: {}", appimage_path.display());
    println!("\n🚀 You can now run your AppImage:");
    println!("   ./{}", appimage_path.file_name().unwrap().to_string_lossy());
    
    Ok(())
}

fn display_results(results: &[compiler::CompilationResult], verbose: bool) {
    println!("\n📊 Compilation Results:");
    println!("{}", "=".repeat(50));

    let mut total_files = 0;
    let mut successful_compilations = 0;
    let mut failed_compilations = 0;

    for result in results {
        total_files += result.files.len();
        
        match &result.status {
            compiler::CompilationStatus::Success { output } => {
                successful_compilations += result.files.len();
                println!("✅ {}: {} files compiled successfully", 
                    result.language.name().bold().green(), 
                    result.files.len()
                );
                if verbose && !output.is_empty() {
                    println!("   Output: {}", output);
                }
            }
            compiler::CompilationStatus::Failure { error } => {
                failed_compilations += result.files.len();
                println!("❌ {}: {} files failed to compile", 
                    result.language.name().bold().red(), 
                    result.files.len()
                );
                if verbose {
                    println!("   Error: {}", error);
                }
            }
        }
    }

    println!("{}", "=".repeat(50));
    println!("📈 Summary:");
    println!("  Total files: {}", total_files);
    println!("  Successful: {} {}", successful_compilations, "✅".green());
    println!("  Failed: {} {}", failed_compilations, "❌".red());
    
    if failed_compilations == 0 {
        println!("\n🎉 {} All files compiled successfully!", "SUCCESS".bold().green());
    } else {
        println!("\n⚠️  {} files failed to compile. Check the output above for details.", failed_compilations);
        std::process::exit(1);
    }
} 