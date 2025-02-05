use inference_gateway_sdk::GatewayError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoderError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Index error: {0}")]
    IndexError(String),

    #[error("GitHub API error: {0}")]
    GitHubError(#[from] octocrab::Error),

    #[error("IO error occurred: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Inference Gateway error: {0}")]
    InferenceGateway(#[from] GatewayError),

    #[error("Failed to deserialize JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("YAML error occurred: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("String error: {0}")]
    StringError(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),
}
