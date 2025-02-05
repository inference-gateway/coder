use inference_gateway_sdk::GatewayError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoderError {
    #[error("Failed to parse integer: {0}")]
    ConfigError(String),

    #[error("Failed to parse integer: {0}")]
    IndexError(String),

    #[error("Failed to parse integer: {0}")]
    GitHubError(#[from] octocrab::Error),

    #[error("IO error occurred: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Inference Gateway error: {0}")]
    InferenceGateway(#[from] GatewayError),

    #[error("Failed to deserialize JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("YAML error occurred: {0}")]
    YamlError(#[from] serde_yaml::Error),
}
