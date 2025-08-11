use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use dirs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_parallel_jobs")]
    pub parallel_jobs: usize,
    
    #[serde(default = "default_compiler_flags")]
    pub compiler_flags: HashMap<String, String>,
    
    #[serde(default = "default_ignore_patterns")]
    pub ignore_patterns: Vec<String>,
    
    #[serde(default = "default_include_patterns")]
    pub include_patterns: Vec<String>,
    
    #[serde(default = "default_output_directory")]
    pub output_directory: Option<String>,
    
    #[serde(default = "default_verbose_output")]
    pub verbose_output: bool,
    
    #[serde(default = "default_auto_clean")]
    pub auto_clean: bool,
    
    #[serde(default = "default_watch_mode")]
    pub watch_mode: bool,
    
    #[serde(default = "default_language_settings")]
    pub language_settings: HashMap<String, LanguageConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    
    #[serde(default = "default_compiler_path")]
    pub compiler_path: Option<String>,
    
    #[serde(default = "default_compiler_flags_vec")]
    pub compiler_flags: Vec<String>,
    
    #[serde(default = "default_output_format")]
    pub output_format: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        
        if config_path.exists() {
            let config_content = fs::read_to_string(&config_path)
                .context("Failed to read configuration file")?;
            
            let config: Config = serde_json::from_str(&config_content)
                .context("Failed to parse configuration file")?;
            
            Ok(config)
        } else {
            // Create default configuration
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        
        // Ensure config directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create configuration directory")?;
        }
        
        let config_content = serde_json::to_string_pretty(self)
            .context("Failed to serialize configuration")?;
        
        fs::write(&config_path, config_content)
            .context("Failed to write configuration file")?;
        
        Ok(())
    }

    pub fn get_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        
        Ok(config_dir.join("lol").join("config.json"))
    }

    pub fn get_compiler_flags(&self, language: &str) -> Option<&String> {
        self.compiler_flags.get(language)
    }

    pub fn set_compiler_flags(&mut self, language: &str, flags: String) {
        self.compiler_flags.insert(language.to_string(), flags);
    }

    pub fn add_ignore_pattern(&mut self, pattern: String) {
        if !self.ignore_patterns.contains(&pattern) {
            self.ignore_patterns.push(pattern);
        }
    }

    pub fn remove_ignore_pattern(&mut self, pattern: &str) {
        self.ignore_patterns.retain(|p| p != pattern);
    }

    pub fn add_include_pattern(&mut self, pattern: String) {
        if !self.include_patterns.contains(&pattern) {
            self.include_patterns.push(pattern);
        }
    }

    pub fn remove_include_pattern(&mut self, pattern: &str) {
        self.include_patterns.retain(|p| p != pattern);
    }

    pub fn should_ignore_file(&self, file_path: &Path) -> bool {
        let file_path_str = file_path.to_string_lossy();
        
        // Check ignore patterns
        for pattern in &self.ignore_patterns {
            if Self::matches_pattern(&file_path_str, pattern) {
                return true;
            }
        }
        
        // Check include patterns (if any are specified, file must match at least one)
        if !self.include_patterns.is_empty() {
            let mut matches_include = false;
            for pattern in &self.include_patterns {
                if Self::matches_pattern(&file_path_str, pattern) {
                    matches_include = true;
                    break;
                }
            }
            if !matches_include {
                return true;
            }
        }
        
        false
    }

    fn matches_pattern(file_path: &str, pattern: &str) -> bool {
        // Simple glob-like pattern matching
        if pattern.contains('*') {
            let regex_pattern = pattern
                .replace(".", "\\.")
                .replace("*", ".*");
            
            if let Ok(regex) = regex::Regex::new(&format!("^{}$", regex_pattern)) {
                return regex.is_match(file_path);
            }
        }
        
        file_path.contains(pattern)
    }

    pub fn get_language_config(&self, language: &str) -> Option<&LanguageConfig> {
        self.language_settings.get(language)
    }

    pub fn set_language_config(&mut self, language: &str, config: LanguageConfig) {
        self.language_settings.insert(language.to_string(), config);
    }

    pub fn is_language_enabled(&self, language: &str) -> bool {
        self.language_settings
            .get(language)
            .map(|config| config.enabled)
            .unwrap_or(true) // Default to enabled if not specified
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            parallel_jobs: default_parallel_jobs(),
            compiler_flags: default_compiler_flags(),
            ignore_patterns: default_ignore_patterns(),
            include_patterns: default_include_patterns(),
            output_directory: default_output_directory(),
            verbose_output: default_verbose_output(),
            auto_clean: default_auto_clean(),
            watch_mode: default_watch_mode(),
            language_settings: default_language_settings(),
        }
    }
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
            compiler_path: default_compiler_path(),
            compiler_flags: default_compiler_flags_vec(),
            output_format: default_output_format(),
        }
    }
}

fn default_parallel_jobs() -> usize {
    num_cpus::get()
}

fn default_compiler_flags() -> HashMap<String, String> {
    let mut flags = HashMap::new();
    flags.insert("c".to_string(), "-Wall -Wextra -std=c99".to_string());
    flags.insert("cpp".to_string(), "-Wall -Wextra -std=c++17".to_string());
    flags.insert("rust".to_string(), "--release".to_string());
    flags.insert("go".to_string(), "-ldflags=-s -ldflags=-w".to_string());
    flags
}

fn default_compiler_flags_vec() -> Vec<String> {
    Vec::new()
}

fn default_ignore_patterns() -> Vec<String> {
    vec![
        "*.o".to_string(),
        "*.obj".to_string(),
        "*.exe".to_string(),
        "*.dll".to_string(),
        "*.so".to_string(),
        "*.dylib".to_string(),
        "*.a".to_string(),
        "*.lib".to_string(),
        "target/".to_string(),
        "build/".to_string(),
        "dist/".to_string(),
        "node_modules/".to_string(),
        ".git/".to_string(),
        ".svn/".to_string(),
        ".hg/".to_string(),
    ]
}

fn default_include_patterns() -> Vec<String> {
    Vec::new()
}

fn default_output_directory() -> Option<String> {
    Some("build".to_string())
}

fn default_verbose_output() -> bool {
    false
}

fn default_auto_clean() -> bool {
    false
}

fn default_watch_mode() -> bool {
    false
}

fn default_enabled() -> bool {
    true
}

fn default_compiler_path() -> Option<String> {
    None
}

fn default_output_format() -> Option<String> {
    None
}

fn default_language_settings() -> HashMap<String, LanguageConfig> {
    let mut settings = HashMap::new();
    
    // C language settings
    settings.insert("c".to_string(), LanguageConfig {
        enabled: true,
        compiler_path: None,
        compiler_flags: vec!["-Wall".to_string(), "-Wextra".to_string(), "-std=c99".to_string()],
        output_format: Some("o".to_string()),
    });
    
    // C++ language settings
    settings.insert("cpp".to_string(), LanguageConfig {
        enabled: true,
        compiler_path: None,
        compiler_flags: vec!["-Wall".to_string(), "-Wextra".to_string(), "-std=c++17".to_string()],
        output_format: Some("o".to_string()),
    });
    
    // Rust language settings
    settings.insert("rust".to_string(), LanguageConfig {
        enabled: true,
        compiler_path: None,
        compiler_flags: vec!["--release".to_string()],
        output_format: None,
    });
    
    // Go language settings
    settings.insert("go".to_string(), LanguageConfig {
        enabled: true,
        compiler_path: None,
        compiler_flags: vec!["-ldflags=-s".to_string(), "-ldflags=-w".to_string()],
        output_format: None,
    });
    
    settings
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.parallel_jobs, num_cpus::get());
        assert!(!config.ignore_patterns.is_empty());
        assert!(config.language_settings.contains_key("c"));
    }

    #[test]
    fn test_ignore_patterns() {
        let mut config = Config::default();
        
        // Test ignore patterns
        assert!(config.should_ignore_file(Path::new("file.o")));
        assert!(config.should_ignore_file(Path::new("build/file.cpp")));
        assert!(!config.should_ignore_file(Path::new("main.c")));
    }

    #[test]
    fn test_include_patterns() {
        let mut config = Config::default();
        config.include_patterns = vec!["*.c".to_string(), "*.cpp".to_string()];
        
        assert!(config.should_ignore_file(Path::new("file.py")));
        assert!(!config.should_ignore_file(Path::new("main.c")));
        assert!(!config.should_ignore_file(Path::new("helper.cpp")));
    }

    #[test]
    fn test_pattern_matching() {
        assert!(Config::matches_pattern("file.o", "*.o"));
        assert!(Config::matches_pattern("src/main.c", "*.c"));
        assert!(Config::matches_pattern("build/", "build/"));
        assert!(!Config::matches_pattern("main.c", "*.o"));
    }

    #[test]
    fn test_config_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let original_config = Config::default();
        
        // Modify config
        let mut config = original_config.clone();
        config.parallel_jobs = 8;
        config.add_ignore_pattern("*.tmp".to_string());
        
        // Save and reload
        config.save().unwrap();
        let loaded_config = Config::load().unwrap();
        
        assert_eq!(loaded_config.parallel_jobs, 8);
        assert!(loaded_config.ignore_patterns.contains(&"*.tmp".to_string()));
    }
} 