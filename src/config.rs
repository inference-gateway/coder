pub const DEFAULT_CONFIG_TEMPLATE: &str = r#"---
# AI Coder Configuration
api:
  endpoint: http://localhost:8080

github:
  owner: ""
  repo: ""

agent:
  model: deepseek-r1-distill-llama-70b
  provider: groq
"#;
