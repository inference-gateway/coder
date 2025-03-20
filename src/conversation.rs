use inference_gateway_sdk::{Message, Provider};
use serde::{Deserialize, Serialize};
use std::{fmt, time::SystemTime};

use crate::errors::CoderError;
use tiktoken_rs::o200k_base;

#[derive(Serialize, Deserialize, Clone)]
pub struct Conversation {
    id: String,
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
    max_tokens: Option<usize>,
}

impl Conversation {
    pub fn new(model: String, provider: Provider, max_tokens: Option<usize>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
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
                max_tokens,
            },
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    #[allow(dead_code)]
    pub fn get_current_tokens(&self) -> Result<usize, CoderError> {
        let bpe = o200k_base().map_err(|e| CoderError::TokenizationError(e.to_string()))?;

        let mut tokens = 0;
        for msg in &self.messages {
            tokens += bpe.encode_with_special_tokens(&msg.content).len();
        }

        Ok(tokens)
    }

    // pub fn add_reviewed_file(&mut self, file: String) {
    //     self.metadata.files_reviewed.push(file);
    // }
}

impl TryInto<Vec<Message>> for Conversation {
    type Error = CoderError;

    fn try_into(self) -> Result<Vec<Message>, Self::Error> {
        if let Some(limit) = self.metadata.max_tokens {
            let bpe = o200k_base().map_err(|e| CoderError::TokenizationError(e.to_string()))?;
            let mut tokens = 0;
            let mut messages = Vec::new();

            for msg in self.messages.iter().rev() {
                let msg_tokens = bpe.encode_with_special_tokens(&msg.content).len();
                if tokens + msg_tokens > limit {
                    break;
                }
                tokens += msg_tokens;
                messages.push(msg.clone());
            }

            messages.reverse();
            Ok(messages)
        } else {
            Ok(self.messages)
        }
    }
}

impl fmt::Debug for Conversation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Conversation {{")?;
        writeln!(f, "  id: {:?}", self.id)?;
        writeln!(f, "  created: {:?}", self.created_at)?;
        writeln!(f, "  messages: [")?;
        for msg in &self.messages {
            writeln!(f, "    {{")?;
            writeln!(f, "      role: {:?}", msg.role)?;
            writeln!(f, "      content: \"{}\"", msg.content.replace("\\n", "\n"))?;
            writeln!(f, "      tool_call_id: \"{:?}\"", msg.tool_call_id)?;
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

#[cfg(test)]
mod tests {
    use crate::conversation::Message;
    use crate::MessageRole;

    #[test]
    fn test_message_tool_call_id_serialization() {
        // Test with tool_call_id present
        let message = Message {
            role: MessageRole::Tool,
            content: "Test content".to_string(),
            tool_call_id: Some("call_123".to_string()),
            ..Default::default()
        };

        let serialized = serde_json::to_string(&message).unwrap();
        let expected = r#"{"role":"tool","content":"Test content","tool_call_id":"call_123"}"#;
        assert_eq!(serialized, expected);

        // Test deserialization
        let deserialized: Message = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.tool_call_id, Some("call_123".to_string()));

        // Test without tool_call_id
        let message_without_id = Message {
            role: MessageRole::Tool,
            content: "Test content".to_string(),
            tool_call_id: None,
            ..Default::default()
        };

        let serialized = serde_json::to_string(&message_without_id).unwrap();
        let expected = r#"{"role":"tool","content":"Test content"}"#;
        assert_eq!(serialized, expected);
    }
}
