use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::errors::CoderError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub language: LanguageConfig,
    pub scm: ScmConfig,
    pub agent: AgentConfig,
    pub api: ApiConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageConfig {
    pub name: String,
    pub formatters: Vec<String>,
    pub linters: Vec<String>,
    pub test_commands: Vec<String>,
    pub docs_urls: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScmConfig {
    pub name: String,
    pub owner: String,
    pub repository: String,
    pub issue_template: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgentConfig {
    pub model: String,
    pub provider: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiConfig {
    pub endpoint: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: LanguageConfig {
                name: "rust".to_string(),
                formatters: vec!["cargo fmt".to_string()],
                linters: vec!["cargo clippy".to_string()],
                test_commands: vec!["cargo test".to_string()],
                docs_urls: vec!["https://docs.rs".to_string()],
            },
            scm: ScmConfig {
                name: "github".to_string(),
                owner: "owner".to_string(),
                repository: "owner/repo".to_string(),
                issue_template: Some(
                    [
                        "## Description",
                        "## Steps to Reproduce",
                        "## Expected Behavior",
                        "## Actual Behavior",
                        "## Environment",
                    ]
                    .join("\n"),
                ),
            },
            agent: AgentConfig {
                model: "deepseek-r1-distill-llama-70b".to_string(),
                provider: "groq".to_string(),
            },
            api: ApiConfig {
                endpoint: "http://localhost:8080".to_string(),
            },
        }
    }
}

pub fn default_config() -> String {
    match serde_yaml::to_string(&Config::default()) {
        Ok(yaml) => format!("---\n# AI Coder Configuration\n{yaml}"),
        Err(_) => String::new(), // Handle error appropriately
    }
}

// Load config with proper error handling
pub fn load(path: &Path) -> Result<Config, CoderError> {
    let config_content = fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&config_content)?;
    Ok(config)
}
