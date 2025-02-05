use inference_gateway_sdk::GatewayError;
use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub enum CoderError {
    // Configuration-related errors
    #[error("Configuration error: {0}")]
    ConfigError(String),

    // Index-related errors
    #[error("Index error: {0}")]
    IndexError(String),

    // GitHub API errors
    #[error("GitHub API error: {0}")]
    GitHubError(#[from] octocrab::Error),

    // Input/Output errors
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    // Inference Gateway errors
    #[error("Inference Gateway error: {0}")]
    InferenceGateway(#[from] GatewayError),

    // JSON parsing errors
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    // YAML parsing errors
    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    // Add new error variants for better coverage
    #[error("Failed to parse configuration: {0}")]
    ConfigParseError(String),

    #[error("Invalid input parameters: {0}")]
    CLIError(String),

    #[error("Prompt validation failed: {0}")]
    PromptValidation(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl CoderError {
    pub fn new_unknown_error(&str) -> Self {
        Self::Unknown(format!("{0}".into()))
    }
}