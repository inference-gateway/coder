use inference_gateway_sdk::GatewayError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoderError {
    #[error("IO error occurred: {0}")]
    Io(#[from] std::io::Error),

    #[error("Inference Gateway error: {0}")]
    InferenceGateway(#[from] GatewayError),

    #[error("Failed to deserialize JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("YAML error occurred: {0}")]
    Yaml(#[from] serde_yaml::Error),
}
