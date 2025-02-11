pub const DEFAULT_CONFIG_TEMPLATE: &str = r#"---
# AI Coder Configuration
api:
  endpoint: http://localhost:8080

scm:
  name: "github"
  repository: "owner/repo"
  issue_template: |
    ## Description
    ## Steps to Reproduce
    ## Expected Behavior
    ## Actual Behavior
    ## Environment

agent:
  model: deepseek-r1-distill-llama-70b
  provider: groq

language:
  name: "rust"
  formatters: ["cargo fmt"]
  linters: ["cargo clippy"]
  test_commands: ["cargo test"]
  docs_urls: ["https://docs.rs"]

"#;
