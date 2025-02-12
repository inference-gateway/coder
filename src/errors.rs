use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoderError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Git error: {0}")]
    GitError(String),

    #[error("Command error: {0}")]
    CommandError(String),

    #[error("Missing arguments: {0}")]
    MissingArguments(String),

    #[error("Tokenization error: {0}")]
    TokenizationError(String),

    #[error("GitHub API error: {0}")]
    GitHubError(#[from] octocrab::Error),

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Inference Gateway error: {0}")]
    InferenceGatewayError(#[from] inference_gateway_sdk::GatewayError),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),
}
