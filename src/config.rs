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
    pub analyse: String,
    pub linter: String,
    pub test_command: String,
    pub docs_url: String,
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
                linter: "cargo fmt".to_string(),
                analyse: "cargo clippy".to_string(),
                test_command: "cargo test".to_string(),
                docs_url: "https://docs.rs".to_string(),
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
                provider: "groq".to_string(),
                model: "deepseek-r1-distill-llama-70b".to_string(),
                max_tokens: Some(4000),
            },
            api: ApiConfig {
                endpoint: "http://localhost:8080/v1".to_string(),
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

    // Language settings
    config.language.name = std::env::var("CODER_LANGUAGE_NAME").unwrap_or(config.language.name);
    config.language.analyse =
        std::env::var("CODER_LANGUAGE_ANALYSE").unwrap_or(config.language.analyse);
    config.language.linter =
        std::env::var("CODER_LANGUAGE_LINTER").unwrap_or(config.language.linter);
    config.language.test_command =
        std::env::var("CODER_LANGUAGE_TEST_COMMAND").unwrap_or(config.language.test_command);
    config.language.docs_url =
        std::env::var("CODER_LANGUAGE_DOCS_URL").unwrap_or(config.language.docs_url);

    // API settings
    config.api.endpoint =
        std::env::var("CODER_INFERENCE_GATEWAY_URL").unwrap_or(config.api.endpoint);

    // SCM settings
    // CODER_SCM_TOKEN is not stored on disk
    config.scm.name = std::env::var("CODER_SCM_NAME").unwrap_or(config.scm.name);
    config.scm.owner = std::env::var("CODER_SCM_USERNAME").unwrap_or(config.scm.owner);
    config.scm.repository = std::env::var("CODER_SCM_REPOSITORY").unwrap_or(config.scm.repository);

    // Agent settings
    config.agent.provider = std::env::var("CODER_AGENT_PROVIDER").unwrap_or(config.agent.provider);
    config.agent.model = std::env::var("CODER_AGENT_MODEL").unwrap_or(config.agent.model);
    if let Ok(max_tokens) = std::env::var("CODER_AGENT_MAX_TOKENS") {
        match max_tokens.parse() {
            Ok(max_tokens) => config.agent.max_tokens = Some(max_tokens),
            Err(_) => error!("Invalid CODER_AGENT_MAX_TOKENS value"),
        }
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;
    use tempfile::NamedTempFile;

    fn create_test_config_file(content: &str) -> NamedTempFile {
        let file = NamedTempFile::new().unwrap();
        fs::write(file.path(), content).unwrap();
        file
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.language.name, "rust");
        assert_eq!(config.language.linter, "cargo fmt");
        assert_eq!(config.language.analyse, "cargo clippy");
        assert_eq!(config.language.test_command, "cargo test");
        assert_eq!(config.language.docs_url, "https://docs.rs");

        assert_eq!(config.agent.provider, "groq");
        assert_eq!(config.agent.model, "deepseek-r1-distill-llama-70b");
        assert_eq!(config.agent.max_tokens, Some(4000));

        assert_eq!(config.api.endpoint, "http://localhost:8080/v1");
    }

    #[test]
    #[serial]
    fn test_load_config_with_env_vars() {
        let config_content = r#"---
language:
  name: "rust"
  analyse: "cargo clippy"
  linter: "cargo fmt"
  test_command: "cargo test"
  docs_url: "https://docs.rs"
scm:
  name: "github"
  owner: "test"
  repository: "test"
agent:
  model: "default-model"
  provider: "default-provider"
  max_tokens: 1000
api:
  endpoint: "http://localhost:8080"
"#;
        let config_file = create_test_config_file(config_content);

        env::set_var("CODER_LANGUAGE_NAME", "python");
        env::set_var("CODER_SCM_NAME", "gitlab");
        env::set_var("CODER_AGENT_MODEL", "new-model");
        env::set_var("CODER_AGENT_MAX_TOKENS", "2000");

        let config = load(config_file.path()).unwrap();

        assert_eq!(config.language.name, "python");
        assert_eq!(config.scm.name, "gitlab");
        assert_eq!(config.agent.model, "new-model");
        assert_eq!(config.agent.max_tokens, Some(2000));

        env::remove_var("CODER_LANGUAGE_NAME");
        env::remove_var("CODER_SCM_NAME");
        env::remove_var("CODER_AGENT_MODEL");
        env::remove_var("CODER_AGENT_MAX_TOKENS");
    }

    #[test]
    fn test_invalid_config_file() {
        let invalid_content = "invalid: yaml: content";
        let config_file = create_test_config_file(invalid_content);

        let result = load(config_file.path());
        assert!(result.is_err());
    }

    #[test]
    #[serial]
    fn test_invalid_max_tokens() {
        let config_content = r#"---
    language:
      name: "rust"
      analyse: "cargo clippy"
      linter: "cargo fmt"
      test_command: "cargo test"
      docs_url: "https://docs.rs"
    scm:
      name: "github"
      owner: "test"
      repository: "test"
    agent:
      model: "default-model"
      provider: "default-provider"
      max_tokens: 1000
    api:
      endpoint: "http://localhost:8080"
    "#;
        let config_file = create_test_config_file(config_content);

        env::set_var("CODER_AGENT_MAX_TOKENS", "not_a_number");

        let config = load(config_file.path()).unwrap();
        assert_eq!(config.agent.max_tokens, Some(1000));

        env::remove_var("CODER_AGENT_MAX_TOKENS");
    }
}
