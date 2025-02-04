use inference_gateway_sdk::{Message, Provider};
use serde::{Deserialize, Serialize};
use std::{fmt, time::SystemTime};

#[derive(Serialize, Deserialize, Clone)]
pub struct Conversation {
    id: String,
    github_issue_id: Option<String>,
    created_at: SystemTime,
    messages: Vec<Message>,
    metadata: ConversationMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn parse_response_for_requested_files(response: &str) -> Vec<String> {
        response
            .lines()
            .filter(|line| line.starts_with("REQUEST:"))
            .map(|line| line.trim_start_matches("REQUEST:").trim().to_string())
            .collect()
    }
}

impl TryInto<Vec<Message>> for Conversation {
    type Error = crate::errors::CoderError;

    fn try_into(self) -> Result<Vec<Message>, Self::Error> {
        Ok(self.messages)
    }
}

impl fmt::Debug for Conversation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Conversation {{")?;
        writeln!(f, "  id: {:?}", self.id)?;
        writeln!(
            f,
            "  issue: {:?}",
            self.github_issue_id.as_deref().unwrap_or("None")
        )?;
        writeln!(f, "  created: {:?}", self.created_at)?;
        writeln!(f, "  messages: [")?;
        for msg in &self.messages {
            writeln!(f, "    {{")?;
            writeln!(f, "      role: {:?}", msg.role)?;
            writeln!(f, "      content: \"{}\"", msg.content.replace("\\n", "\n"))?;
            writeln!(f, "    }},")?;
        }
        writeln!(f, "  ]")?;
        writeln!(f, "  metadata: {{")?;
        writeln!(f, "    repository: {:?}", self.metadata.repository)?;
        writeln!(f, "    model: {:?}", self.metadata.model)?;
        writeln!(f, "    provider: {:?}", self.metadata.provider)?;
        writeln!(f, "    files_reviewed: {:?}", self.metadata.files_reviewed)?;
        writeln!(f, "  }}")?;
        write!(f, "}}")
    }
}
