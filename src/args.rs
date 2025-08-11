use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "lol",
    about = "The Fast Multi-language Code Compiler CLI App",
    long_about = "lol is a simple yet powerful CLI tool designed to speed up compiling source code across multiple programming languages with a single command. It can also create AppImages with consolidated source code.",
    version,
    author
)]
pub struct Args {
    /// Project directory to compile or create AppImage from
    #[arg(value_name = "PROJECT_PATH")]
    pub project_path: PathBuf,

    /// Compile C files
    #[arg(long)]
    pub c: bool,

    /// Compile C++ files
    #[arg(long)]
    pub cpp: bool,

    /// Compile Python files
    #[arg(long)]
    pub python: bool,

    /// Compile Java files
    #[arg(long)]
    pub java: bool,

    /// Compile Rust files
    #[arg(long)]
    pub rust: bool,

    /// Compile Go files
    #[arg(long)]
    pub go: bool,

    /// Compile JavaScript/TypeScript files
    #[arg(long)]
    pub js: bool,

    /// Compile TypeScript files
    #[arg(long)]
    pub ts: bool,

    /// Compile all detected languages
    #[arg(long)]
    pub all: bool,

    /// Show verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Number of parallel compilation jobs
    #[arg(short, long, default_value_t = num_cpus::get())]
    pub jobs: usize,

    /// Custom compiler flags for C/C++
    #[arg(long, value_name = "FLAGS")]
    pub cflags: Option<String>,

    /// Custom compiler flags for C++
    #[arg(long, value_name = "FLAGS")]
    pub cxxflags: Option<String>,

    /// Create an AppImage with consolidated source code (instead of compiling)
    #[arg(long, value_name = "NAME")]
    pub name: Option<String>,
} 