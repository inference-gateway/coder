use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoderError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Index-related error: {0}")]
    IndexError(String),

    #[error("Git error: {0}")]
    GitError(String),

    #[error("GitHub API error: {0}")]
    GitHubError(#[from] octocrab::Error),

    #[error("Tool error: {0}")]
    ToolError(String),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Inference Gateway error: {0}")]
    InferenceGatewayError(#[from] inference_gateway_sdk::GatewayError),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("File not found: {0}")]
    FileNotFoundError(String),

    #[error("Permission denied: {0}")]
    PermissionDeniedError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Parsing error: {0}")]
    ParseError(String, #[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
