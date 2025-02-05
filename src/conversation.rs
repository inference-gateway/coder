use inference_gateway_sdk::{Message, Provider};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, time::SystemTime};

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

    /// Parse the response for any file fixes that were requested.
    /// This is a simple implementation that assumes the response is a string
    /// in the following format:
    /// ```plaintext
    /// ```rust
    /// FILE: src/errors.rs
    /// ```
    /// ```rust
    /// use inference_gateway_sdk::GatewayError;
    /// use thiserror::Error;
    ///
    /// ```
    /// ```
    ///
    /// This function returns a vector of hashmaps where each hashmap contains
    /// the file path and the content of the file that was fixed.
    pub fn parse_response_for_fixes(response: &str) -> Vec<HashMap<String, String>> {
        let mut fixes = Vec::new();
        let mut current_file = None;
        let mut current_content = String::new();
        let mut in_code_block = false;

        for line in response.lines() {
            if line.starts_with("FILE:") {
                if let Some(file) = current_file.take() {
                    let mut fix = HashMap::new();
                    fix.insert(file, current_content.trim().to_string());
                    fixes.push(fix);
                    current_content.clear();
                }
                current_file = Some(line.trim_start_matches("FILE:").trim().to_string());
            } else if line.starts_with("```rust") {
                in_code_block = true;
                continue;
            } else if line.starts_with("```") {
                in_code_block = false;
                continue;
            } else if in_code_block && current_file.is_some() {
                current_content.push_str(line);
                current_content.push('\n');
            }
        }

        if let Some(file) = current_file {
            let mut fix = HashMap::new();
            fix.insert(file, current_content.trim().to_string());
            fixes.push(fix);
        }

        fixes
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_response_for_file_fixes() {
        let response = r#"
FILE: src/errors.rs
```rust
use inference_gateway_sdk::GatewayError;
use thiserror::Error;

```
FILE: src/main.rs
```rust
fn main() {
    println!("Hello");
}
```

Would you like me to apply these changes?
"#;
        let fixes = Conversation::parse_response_for_fixes(response);

        assert_eq!(fixes.len(), 2);
        assert!(fixes[0].contains_key("src/errors.rs"));
        assert!(fixes[1].contains_key("src/main.rs"));

        assert_eq!(
            fixes[0].get("src/errors.rs").unwrap(),
            "use inference_gateway_sdk::GatewayError;\nuse thiserror::Error;"
        );
        assert_eq!(
            fixes[1].get("src/main.rs").unwrap(),
            "fn main() {\n    println!(\"Hello\");\n}"
        );
    }
}
