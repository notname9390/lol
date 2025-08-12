use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    C,
    Cpp,
    Python,
    Java,
    Rust,
    Go,
    JavaScript,
    TypeScript,
    CSharp,
    Swift,
    Kotlin,
    Scala,
    Haskell,
    FSharp,
    OCaml,
    Nim,
    Zig,
    V,
    Odin,
    Jai,
}

impl Language {
    pub fn name(&self) -> &'static str {
        match self {
            Language::C => "C",
            Language::Cpp => "C++",
            Language::Python => "Python",
            Language::Java => "Java",
            Language::Rust => "Rust",
            Language::Go => "Go",
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::CSharp => "C#",
            Language::Swift => "Swift",
            Language::Kotlin => "Kotlin",
            Language::Scala => "Scala",
            Language::Haskell => "Haskell",
            Language::FSharp => "F#",
            Language::OCaml => "OCaml",
            Language::Nim => "Nim",
            Language::Zig => "Zig",
            Language::V => "V",
            Language::Odin => "Odin",
            Language::Jai => "Jai",
        }
    }

    pub fn extensions(&self) -> Vec<&'static str> {
        match self {
            Language::C => vec!["c", "h"],
            Language::Cpp => vec!["cpp", "cc", "cxx", "c++", "hpp", "hxx", "h++"],
            Language::Python => vec!["py", "pyw", "pyx", "pxd"],
            Language::Java => vec!["java"],
            Language::Rust => vec!["rs"],
            Language::Go => vec!["go"],
            Language::JavaScript => vec!["js", "mjs", "cjs"],
            Language::TypeScript => vec!["ts", "tsx"],
            Language::CSharp => vec!["cs"],
            Language::Swift => vec!["swift"],
            Language::Kotlin => vec!["kt", "kts"],
            Language::Scala => vec!["scala", "sc"],
            Language::Haskell => vec!["hs", "lhs"],
            Language::FSharp => vec!["fs", "fsx", "fsi"],
            Language::OCaml => vec!["ml", "mli"],
            Language::Nim => vec!["nim"],
            Language::Zig => vec!["zig"],
            Language::V => vec!["v"],
            Language::Odin => vec!["odin"],
            Language::Jai => vec!["jai"],
        }
    }

    pub fn is_compiled(&self) -> bool {
        match self {
            Language::C | Language::Cpp | Language::Java | Language::Rust | 
            Language::Go | Language::CSharp | Language::Swift | Language::Kotlin | 
            Language::Scala | Language::Haskell | Language::FSharp | Language::OCaml |
            Language::Nim | Language::Zig | Language::V | Language::Odin | Language::Jai => true,
            Language::Python | Language::JavaScript | Language::TypeScript => false,
        }
    }

    pub fn needs_compiler_check(&self) -> bool {
        match self {
            Language::C | Language::Cpp | Language::Java | Language::Rust | 
            Language::Go | Language::CSharp | Language::Swift | Language::Kotlin | 
            Language::Scala | Language::Haskell | Language::FSharp | Language::OCaml |
            Language::Nim | Language::Zig | Language::V | Language::Odin | Language::Jai => true,
            Language::Python | Language::JavaScript | Language::TypeScript => false,
        }
    }

    pub fn check_compiler_available(&self) -> bool {
        if !self.needs_compiler_check() {
            return true;
        }

        let (compiler, args) = self.get_compiler_command();
        Command::new(compiler)
            .args(args)
            .output()
            .is_ok()
    }

    pub fn get_compiler_command(&self) -> (&'static str, Vec<&'static str>) {
        match self {
            Language::C => ("gcc", vec!["--version"]),
            Language::Cpp => ("g++", vec!["--version"]),
            Language::Java => ("javac", vec!["-version"]),
            Language::Rust => ("rustc", vec!["--version"]),
            Language::Go => ("go", vec!["version"]),
            Language::CSharp => ("dotnet", vec!["--version"]),
            Language::Swift => ("swiftc", vec!["--version"]),
            Language::Kotlin => ("kotlinc", vec!["-version"]),
            Language::Scala => ("scalac", vec!["-version"]),
            Language::Haskell => ("ghc", vec!["--version"]),
            Language::FSharp => ("fsharpc", vec!["--help"]),
            Language::OCaml => ("ocamlc", vec!["-version"]),
            Language::Nim => ("nim", vec!["--version"]),
            Language::Zig => ("zig", vec!["version"]),
            Language::V => ("v", vec!["version"]),
            Language::Odin => ("odin", vec!["version"]),
            Language::Jai => ("jai", vec!["--version"]),
            Language::Python | Language::JavaScript | Language::TypeScript => ("", vec![]),
        }
    }

    pub fn get_compilation_command(&self, file: &PathBuf, custom_flags: Option<&str>) -> Result<Command> {
        let mut cmd;
        let mut args: Vec<String> = Vec::new();

        match self {
            Language::C => {
                cmd = Command::new("gcc");
                args.push("-c".to_string());
                if let Some(flags) = custom_flags {
                    args.extend(flags.split_whitespace().map(|s| {
                        if s.starts_with('-') {
                            s.to_string()
                        } else {
                            format!("-{}", s)
                        }
                    }));
                }
                args.push("-o".to_string());
                let output_file = file.with_extension("o");
                let output_file_str = output_file.to_str().unwrap().to_string();
                args.push(output_file_str);
                args.push(file.to_str().unwrap().to_string());
            }
            Language::Cpp => {
                cmd = Command::new("g++");
                args.push("-c".to_string());
                if let Some(flags) = custom_flags {
                    args.extend(flags.split_whitespace().map(|s| {
                        if s.starts_with('-') {
                            s.to_string()
                        } else {
                            format!("-{}", s)
                        }
                    }));
                }
                args.push("-o".to_string());
                let output_file = file.with_extension("o");
                let output_file_str = output_file.to_str().unwrap().to_string();
                args.push(output_file_str);
                args.push(file.to_str().unwrap().to_string());
            }
            Language::Python => {
                cmd = Command::new("python3");
                args.push("-m".to_string());
                args.push("py_compile".to_string());
                args.push(file.to_str().unwrap().to_string());
            }
            Language::Java => {
                cmd = Command::new("javac");
                args.push(file.to_str().unwrap().to_string());
            }
            Language::Rust => {
                cmd = Command::new("rustc");
                args.push(file.to_str().unwrap().to_string());
            }
            Language::Go => {
                cmd = Command::new("go");
                args.push("build".to_string());
                args.push(file.to_str().unwrap().to_string());
            }
            Language::CSharp => {
                cmd = Command::new("dotnet");
                args.push("build".to_string());
                args.push(file.to_str().unwrap().to_string());
            }
            Language::Swift => {
                cmd = Command::new("swiftc");
                args.push(file.to_str().unwrap().to_string());
            }
            Language::Kotlin => {
                cmd = Command::new("kotlinc");
                args.push(file.to_str().unwrap().to_string());
            }
            Language::Scala => {
                cmd = Command::new("scalac");
                args.push(file.to_str().unwrap().to_string());
            }
            Language::Haskell => {
                cmd = Command::new("ghc");
                args.push("-c".to_string());
                args.push(file.to_str().unwrap().to_string());
            }
            Language::FSharp => {
                cmd = Command::new("fsharpc");
                args.push(file.to_str().unwrap().to_string());
            }
            Language::OCaml => {
                cmd = Command::new("ocamlc");
                args.push("-c".to_string());
                args.push(file.to_str().unwrap().to_string());
            }
            Language::Nim => {
                cmd = Command::new("nim");
                args.push("compile".to_string());
                args.push("--run".to_string());
                args.push(file.to_str().unwrap().to_string());
            }
            Language::Zig => {
                cmd = Command::new("zig");
                args.push("build-exe".to_string());
                args.push(file.to_str().unwrap().to_string());
            }
            Language::V => {
                cmd = Command::new("v");
                args.push(file.to_str().unwrap().to_string());
            }
            Language::Odin => {
                cmd = Command::new("odin");
                args.push("build".to_string());
                args.push(file.to_str().unwrap().to_string());
            }
            Language::Jai => {
                cmd = Command::new("jai");
                args.push(file.to_str().unwrap().to_string());
            }
            Language::JavaScript | Language::TypeScript => {
                // For JS/TS, we'll just do syntax checking
                if self == &Language::TypeScript {
                    cmd = Command::new("tsc");
                    args.push("--noEmit".to_string());
                    args.push(file.to_str().unwrap().to_string());
                } else {
                    cmd = Command::new("node");
                    args.push("--check".to_string());
                    args.push(file.to_str().unwrap().to_string());
                }
            }
        }

        cmd.args(args);
        Ok(cmd)
    }
}

pub struct LanguageSupport {
    languages: HashMap<String, Language>,
}

impl LanguageSupport {
    pub fn new() -> Self {
        let mut languages = HashMap::new();
        
        for lang in [
            Language::C, Language::Cpp, Language::Python, Language::Java,
            Language::Rust, Language::Go, Language::CSharp, Language::Swift,
            Language::Kotlin, Language::Scala, Language::Haskell, Language::FSharp,
            Language::OCaml, Language::Nim, Language::Zig, Language::V,
            Language::Odin, Language::Jai, Language::JavaScript, Language::TypeScript,
        ] {
            for ext in lang.extensions() {
                languages.insert(ext.to_string(), lang.clone());
            }
        }
        
        Self { languages }
    }

    pub fn get_language_by_extension(&self, extension: &str) -> Option<&Language> {
        self.languages.get(extension)
    }

    pub fn get_available_languages(&self) -> Vec<&Language> {
        self.languages.values().collect::<Vec<_>>()
    }

    pub fn get_supported_extensions(&self) -> Vec<&String> {
        self.languages.keys().collect::<Vec<_>>()
    }
}

impl Default for LanguageSupport {
    fn default() -> Self {
        Self::new()
    }
} 