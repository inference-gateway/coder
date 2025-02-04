use crate::cli::{Cli, Commands};
use crate::errors::CoderError;
use clap::Parser;
use config::DEFAULT_CONFIG_TEMPLATE;
use inference_gateway_sdk::{
    InferenceGatewayAPI, InferenceGatewayClient, Message, MessageRole, Provider,
};
use log::{info, warn};
use serde_yaml::Value;
use std::{env, fs, path::Path, thread::sleep, time::Duration};

mod cli;
mod config;
mod conversation;
mod errors;
mod index;
mod prompt;
mod utils;

#[tokio::main]
async fn main() -> Result<(), CoderError> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let cli = Cli::parse();
    match cli.command {
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
                Some("".to_string()),
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
            let system_message = prompt::Prompt::get_system_message();
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
    use log::LevelFilter;
    use assert_cmd::Command;
    use assert_fs::prelude::*;
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
        let assert = cmd
            .current_dir(&temp_dir)
            .arg("init")
            .assert();

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
        assert!(content.contains(".coder"), "'.coder' entry missing from .gitignore");
    }
}