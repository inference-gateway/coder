use inference_gateway_sdk::{Message, Provider};
use serde::{Deserialize, Serialize};
use std::{fmt, time::SystemTime};

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
}

impl Conversation {
    pub fn new(model: String, provider: Provider) -> Self {
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
            },
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    // pub fn add_reviewed_file(&mut self, file: String) {
    //     self.metadata.files_reviewed.push(file);
    // }
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
    use serde_json;

    #[test]
    fn test_message_tool_call_id_serialization() {
        // Test with tool_call_id present
        let message = Message {
            role: MessageRole::Tool,
            content: "Test content".to_string(),
            tool_call_id: Some("call_123".to_string()),
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
        };

        let serialized = serde_json::to_string(&message_without_id).unwrap();
        let expected = r#"{"role":"tool","content":"Test content"}"#;
        assert_eq!(serialized, expected);
    }
}
