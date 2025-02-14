use log::error;
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
    pub analyse: Vec<String>,
    pub linter: Vec<String>,
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
    pub max_tokens: Option<usize>,
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
                linter: vec!["cargo fmt".to_string()],
                analyse: vec!["cargo clippy".to_string()],
                test_commands: vec!["cargo test".to_string()],
                docs_urls: vec!["https://docs.rs".to_string()],
            },
            scm: ScmConfig {
                name: "".to_string(),
                owner: "".to_string(),
                repository: "".to_string(),
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
                max_tokens: Some(4000),
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

// Load config from a file with environment variables if exists
pub fn load(path: &Path) -> Result<Config, CoderError> {
    let config_content = fs::read_to_string(path)?;
    let mut config: Config = serde_yaml::from_str(&config_content)?;

    // API settings
    config.api.endpoint =
        std::env::var("CODER_INFERENCE_GATEWAY_URL").unwrap_or_else(|_| config.api.endpoint);

    // SCM settings
    // CODER_SCM_TOKEN is not stored on disk
    config.scm.name = std::env::var("CODER_SCM_NAME").unwrap_or_else(|_| config.scm.name);
    config.scm.owner = std::env::var("CODER_SCM_USERNAME").unwrap_or_else(|_| config.scm.owner);
    config.scm.repository =
        std::env::var("CODER_SCM_REPOSITORY").unwrap_or_else(|_| config.scm.repository);

    // Agent settings
    config.agent.model = std::env::var("CODER_AGENT_MODEL").unwrap_or_else(|_| config.agent.model);
    config.agent.provider =
        std::env::var("CODER_AGENT_PROVIDER").unwrap_or_else(|_| config.agent.provider);
    if let Ok(max_tokens) = std::env::var("CODER_AGENT_MAX_TOKENS") {
        match max_tokens.parse() {
            Ok(max_tokens) => config.agent.max_tokens = Some(max_tokens),
            Err(_) => error!("Invalid CODER_AGENT_MAX_TOKENS value"),
        }
    }

    Ok(config)
}
