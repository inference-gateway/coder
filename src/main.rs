use crate::cli::{Cli, Commands};
use crate::errors::CoderError;
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use config::DEFAULT_CONFIG_TEMPLATE;
use inference_gateway_sdk::{
    InferenceGatewayAPI, InferenceGatewayClient, Message, MessageRole, Provider,
};
use log::{info, warn};
use octocrab::Octocrab;
use serde_yaml::Value;
use std::process::Command;
use std::{env, fs, path::Path, thread::sleep, time::Duration};

mod cli;
mod config;
mod conversation;
mod errors;
mod index;
mod prompt;
mod tools;
mod utils;

#[tokio::main]
async fn main() -> Result<(), CoderError> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let cli = Cli::parse();
    match cli.command {
        Commands::Completions { shell } => {
            generate(shell, &mut Cli::command(), "coder", &mut std::io::stdout());
            return Ok(());
        }
        Commands::Init {} => {
            info!("Initializing AI Coder agent...");
            let coder_dir = Path::new(".coder");
            fs::create_dir_all(coder_dir)?;
            info!("Created .coder directory");
            fs::write(coder_dir.join("config.yaml"), DEFAULT_CONFIG_TEMPLATE)?;

            let gitignore_path = Path::new(".gitignore");
            let gitignore_content = if gitignore_path.exists() {
                let mut content = fs::read_to_string(gitignore_path)?;
                if !content.contains(".coder") {
                    if !content.ends_with('\n') {
                        content.push('\n');
                    }
                    content.push_str(".coder\n");
                }
                content
            } else {
                ".coder\n".to_string()
            };
            fs::write(gitignore_path, gitignore_content)?;

            return Ok(());
        }
        Commands::Index {} => {
            info!("Indexing files...");
            let coder_dir = Path::new(".coder");
            fs::create_dir_all(coder_dir)?;

            let tree = index::build_tree()?;
            let content = index::build_content()?;

            let index_content = format!(
                "---\n# AI Coder Index Configuration\n\ntree: |\n{}\n{}",
                tree.lines()
                    .map(|line| format!("  {}", line))
                    .collect::<Vec<_>>()
                    .join("\n"),
                content
            );

            fs::write(coder_dir.join("index.yaml"), index_content)?;
            info!("Created index at .coder/index.yaml");
        }
        Commands::Fix {
            issue,
            further_instruction,
        } => {
            info!("Fixing issue #{}...", issue);
            info!("Further instructions: {:?}", further_instruction);

            let coder_dir = Path::new(".coder");
            let config_path = coder_dir.join("config.yaml");
            let config_content = fs::read_to_string(config_path)?;
            let config: Value = serde_yaml::from_str(&config_content)?;

            let git_owner = config["github"]["owner"]
                .as_str()
                .ok_or(CoderError::ConfigError(
                    "GitHub owner not found".to_string(),
                ))?;
            let git_repo = config["github"]["repo"]
                .as_str()
                .ok_or(CoderError::ConfigError("GitHub repo not found".to_string()))?;

            info!("Fetching issue #{} from {}/{}", issue, git_owner, git_repo);

            let github_token = std::env::var("GITHUB_TOKEN")
                .map_err(|_| CoderError::ConfigError("GITHUB_TOKEN not set".to_string()))?;

            let octocrab = Octocrab::builder()
                .personal_token(github_token)
                .build()
                .map_err(|e| CoderError::GitHubError(e))?;

            let issue_details = octocrab
                .issues(git_owner, git_repo)
                .get(issue as u64)
                .await
                .map_err(|e| CoderError::GitHubError(e))?;

            let is_bug = issue_details
                .labels
                .iter()
                .any(|label| label.name.to_lowercase() == "bug");

            if !is_bug {
                warn!("Issue #{} is not labeled as a bug. This command is intended for bug fixes only.", issue);
                return Ok(());
            }

            info!("Found issue: {}", issue_details.title);
            // info!("Description: {:?}", issue_details.body);

            let client = InferenceGatewayClient::new(
                &config["api"]["endpoint"].as_str().unwrap_or_default(),
            );

            let mut convo = conversation::Conversation::new(
                "deepseek-r1-distill-llama-70b".to_string(),
                Provider::Groq,
            );

            let system_prompt = format!(
                r#"You are a senior software engineer specializing in Rust development. Your task is to diagnose and fix bugs using the following tools:

AVAILABLE TOOLS:
- get_file_content(path: &str) -> Result<String, Error>
- write_file_content(path: &str, content: &str) -> Result<(), Error>

WORKSPACE INFO:
- Project Structure: {}
- Issue Title: {}
- Issue Description: {}
- Repository: {}/{}
{}

WORKFLOW:
1. Analyze the issue description to identify the root cause
2. Request file contents using the format:
   REQUEST: <file_path>
3. After reviewing the files, propose fixes using the format:
   FILE: <file_path>
   ```rust
    <fixed content>
    ```
"#,
                index::build_tree()?,
                issue_details.title,
                issue_details.body.unwrap(),
                git_owner,
                git_repo,
                if let Some(instr) = &further_instruction {
                    format!("Additional Instructions: {}", instr)
                } else {
                    String::new()
                }
            );

            convo.add_message(Message {
                role: MessageRole::System,
                content: system_prompt,
            });
            let timeout = Duration::from_secs(300);
            info!("Starting AI Coder agent...");
            info!("Press Ctrl+C to stop the agent.");
            loop {
                if timeout.as_secs() == 0 {
                    warn!("Timeout reached. Exiting...");
                    break;
                }

                info!("Generating fix proposal...");

                convo.add_message(Message {
                    role: MessageRole::User,
                    content: "I need help fixing this issue".to_string(),
                });

                let resp = client
                    .generate_content(
                        Provider::Groq,
                        "deepseek-r1-distill-llama-70b",
                        convo.clone().try_into()?,
                    )
                    .await?;

                let assistant_message = utils::strip_thinking(&resp.response.content);
                if assistant_message.is_none() {
                    warn!("Assistant message is empty. Exiting...");
                    break;
                }

                convo.add_message(Message {
                    role: MessageRole::Assistant,
                    content: assistant_message.unwrap().trim().to_string(),
                });

                let file_requests = conversation::Conversation::parse_response_for_requested_files(
                    &resp.response.content,
                );

                info!("File requests: {:?}", file_requests);

                for file_path in file_requests {
                    match tools::get_file_content(&file_path) {
                        Ok(content) => {
                            info!("Retrieved content for {}", file_path);
                            convo.add_reviewed_file(file_path.clone());
                            convo.add_message(Message {
                                role: MessageRole::User,
                                content: format!(
                                    "Content of {}:\n```rust\n{}\n```",
                                    file_path, content
                                ),
                            });
                        }
                        Err(e) => {
                            warn!("Failed to retrieve content for {}: {}", file_path, e);
                            convo.add_message(Message {
                                role: MessageRole::User,
                                content: format!(
                                    "Could not retrieve content for {}: {}",
                                    file_path, e
                                ),
                            });
                        }
                    }
                }

                let resp = client
                    .generate_content(
                        Provider::Groq,
                        "deepseek-r1-distill-llama-70b",
                        convo.clone().try_into()?,
                    )
                    .await?;

                let assistant_message = utils::strip_thinking(&resp.response.content);
                if assistant_message.is_none() {
                    warn!("Assistant message is empty. Exiting...");
                    break;
                }
                let assistant_message_unwarpped = assistant_message.unwrap().trim().to_string();

                convo.add_message(Message {
                    role: MessageRole::Assistant,
                    content: assistant_message_unwarpped.clone(),
                });

                let fixes = conversation::Conversation::parse_response_for_fixes(
                    assistant_message_unwarpped.as_str(),
                );

                for fix in fixes.clone() {
                    for (file_path, content) in fix {
                        match tools::write_file_content(&file_path, &content) {
                            Ok(_) => {
                                info!("Wrote content to {}", file_path);
                                convo.add_message(Message {
                                    role: MessageRole::User,
                                    content: format!("Wrote content to {}", file_path),
                                });
                            }
                            Err(e) => {
                                warn!("Failed to write content to {}: {}", file_path, e);
                                convo.add_message(Message {
                                    role: MessageRole::User,
                                    content: format!(
                                        "Could not write content to {}: {}",
                                        file_path, e
                                    ),
                                });
                            }
                        }
                    }
                }

                // Create branch and PR if there are fixes
                if !fixes.is_empty() {
                    // Create a new branch
                    let branch_name = format!("fix/issue-{}", issue);
                    Command::new("git")
                        .args(["checkout", "-b", &branch_name])
                        .output()
                        .map_err(|e| CoderError::GitError(e.to_string()))?;

                    // Commit changes
                    Command::new("git")
                        .args(["add", "."])
                        .output()
                        .map_err(|e| CoderError::GitError(e.to_string()))?;

                    Command::new("git")
                        .args(["commit", "-m", &format!("fix: address issue #{}", issue)])
                        .output()
                        .map_err(|e| CoderError::GitError(e.to_string()))?;

                    // Push branch
                    Command::new("git")
                        .args(["push", "origin", &branch_name])
                        .output()
                        .map_err(|e| CoderError::GitError(e.to_string()))?;

                    // Create PR using octocrab
                    let pr = octocrab
                        .pulls(git_owner, git_repo)
                        .create(format!("Fix issue #{}", issue), branch_name, "main")
                        .body(format!(
                            "This PR addresses issue #{}. Please review the changes.",
                            issue
                        ))
                        .send()
                        .await
                        .map_err(|e| CoderError::GitHubError(e))?;

                    info!("Created PR: {}", pr.html_url.unwrap());
                }

                break;
            }
        }
        Commands::Refactor {} => {
            info!("Reading the configurations...");
            let coder_dir = Path::new(".coder");
            let config_path = coder_dir.join("config.yaml");
            let config_content = fs::read_to_string(config_path)?;
            let config: Value = serde_yaml::from_str(&config_content)?;

            let git_owner = config["github"]["owner"].as_str().unwrap_or("");
            let git_repo = config["github"]["repo"].as_str().unwrap_or("");

            info!(
                "Connecting to GitHub repository: {}/{}",
                git_owner, git_repo
            );

            info!("Creating an in memory database.");
            let mut convo = conversation::Conversation::new(
                "deepseek-r1-distill-llama-70b".to_string(),
                Provider::Groq,
            );

            // Read the tree structure from index.yaml
            let index_path = Path::new(".coder").join("index.yaml");
            let index_content = fs::read_to_string(index_path)?;
            let index: Value = serde_yaml::from_str(&index_content)?;
            // Get the tree structure
            let tree = index["tree"].as_str().unwrap_or("");

            // TODO: Replace with actual GitHub issue fetching
            let issue_title = "Fix error handling in cli.rs";
            let issue_body =
                "The error handling in cli.rs needs improvement. We should add proper error types.";

            let prompt = prompt::Prompt::create_initial_prompt(tree, issue_title, issue_body);
            let system_message = "".to_string();
            let model = "deepseek-r1-distill-llama-70b";
            convo.add_message(Message {
                role: MessageRole::System,
                content: system_message,
            });
            convo.add_message(Message {
                role: MessageRole::User,
                content: prompt,
            });

            info!("Intializing the inference gateway client.");
            let client = InferenceGatewayClient::new("http://localhost:8080");

            info!("Starting AI Coder agent...");
            info!("Press Ctrl+C to stop the agent.");
            loop {
                let resp = client
                    .generate_content(Provider::Groq, model, convo.clone().try_into()?)
                    .await?;
                let assistant_message = utils::strip_thinking(&resp.response.content);
                if assistant_message.is_none() {
                    info!("Assistant message is empty. Exiting...");
                    break;
                }
                let unwrapped_message = assistant_message.unwrap().to_string();
                let assistant_message = unwrapped_message.trim();

                let files_requests = conversation::Conversation::parse_response_for_requested_files(
                    assistant_message,
                );
                if files_requests.is_empty() {
                    warn!("No files requested. Exiting...");
                    // TODO - think about retry logic
                    break;
                }

                convo.add_message(Message {
                    role: MessageRole::Assistant,
                    content: assistant_message.trim().to_string(),
                });

                let contents = index::extract_file_contents(&index_content);
                let review_prompt =
                    prompt::Prompt::create_review_prompt(&files_requests, &contents);

                convo.add_message(Message {
                    role: MessageRole::User,
                    content: review_prompt.trim().to_string(),
                });

                let resp = client
                    .generate_content(Provider::Groq, model, convo.clone().try_into()?)
                    .await?;
                let assistant_message = utils::strip_thinking(&resp.response.content);
                if assistant_message.is_none() {
                    warn!("Assistant message is empty. Exiting...");
                    break;
                }

                convo.add_message(Message {
                    role: MessageRole::Assistant,
                    content: assistant_message.unwrap().trim().to_string(),
                });

                info!("{:?}", convo);

                // - Pull issues from GitHub
                // - Generate fixes using inference-gateway-sdk
                // - Create pull requests

                // sleep(Duration::from_secs(5));

                sleep(Duration::from_secs(5));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::config::DEFAULT_CONFIG_TEMPLATE;
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use log::LevelFilter;
    use predicates::prelude::*;
    use std::fs;

    #[test]
    fn test_init_command() {
        let _ = env_logger::builder()
            .filter_level(LevelFilter::Info)
            .is_test(true)
            .try_init();

        let temp_dir = assert_fs::TempDir::new().unwrap();

        let mut cmd = Command::cargo_bin("coder").unwrap();
        let assert = cmd.current_dir(&temp_dir).arg("init").assert();

        assert
            .success()
            .stderr(predicate::str::contains("Initializing AI Coder agent"))
            .stderr(predicate::str::contains("Created .coder directory"));

        let config_file = temp_dir.child(".coder/config.yaml");
        config_file.assert(predicate::path::exists());
        config_file.assert(predicate::str::contains(DEFAULT_CONFIG_TEMPLATE));

        let gitignore_path = temp_dir.join(".gitignore");
        assert!(fs::write(&gitignore_path, ".coder\n").is_ok());

        let gitignore = temp_dir.child(".gitignore");
        gitignore.assert(predicate::path::exists());
        gitignore.assert(predicate::str::contains(".coder"));

        let content = fs::read_to_string(&gitignore_path).unwrap();
        assert!(
            content.contains(".coder"),
            "'.coder' entry missing from .gitignore"
        );
    }
}
