use inference_gateway_sdk::{Message, Provider};
use serde::{Serialize, Deserialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Conversation {
    id: String,
    github_issue_id: Option<String>,
    created_at: SystemTime,
    messages: Vec<Message>,
    metadata: ConversationMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConversationMetadata {
    repository: String,
    model: String,
    provider: Provider,
    files_reviewed: Vec<String>,
}

impl Conversation {
    pub fn new(github_issue_id: Option<String>, model: String, provider: Provider) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            github_issue_id,
            created_at: SystemTime::now(),
            messages: Vec::new(),
            metadata: ConversationMetadata {
                repository: std::env::current_dir()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                model,
                provider,
                files_reviewed: Vec::new(),
            },
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn add_reviewed_file(&mut self, file: String) {
        self.metadata.files_reviewed.push(file);
    }

    pub fn parse_response(response: &str) -> Vec<String> {
        response.lines()
            .filter(|line| line.starts_with("REQUEST:"))
            .map(|line| line.trim_start_matches("REQUEST:").trim().to_string())
            .collect()
    }
}
