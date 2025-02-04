use clap::Parser;
use config::DEFAULT_CONFIG_TEMPLATE;
use serde_yaml::Value;
use crate::cli::{Cli, Commands};
use inference_gateway_sdk::{
    InferenceGatewayAPI,
    InferenceGatewayClient,
    MessageRole,
    Provider,
    Message
};
use log::info;
use std::{env, fs, path::Path, thread::sleep, time::Duration};
use crate::errors::CoderError;

mod cli;
mod errors;
mod conversation;
mod config;
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
            println!("Initializing AI Coder agent...");
            let coder_dir = Path::new(".coder");
            fs::create_dir_all(coder_dir)?;
            info!("Created .coder directory");
            fs::write(coder_dir.join("config.yaml"), DEFAULT_CONFIG_TEMPLATE)?;
            return Ok(());
        },
        Commands::Index {} => {
            println!("Indexing files...");
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
            println!("Created index at .coder/index.yaml");
      },
        Commands::Start {} => {
            println!("Reading the configurations...");
            let coder_dir = Path::new(".coder");
            let config_path = coder_dir.join("config.yaml");
            let config_content = fs::read_to_string(config_path)?;
            let config: Value = serde_yaml::from_str(&config_content)?;

            let git_owner = config["github"]["owner"].as_str().unwrap_or("");
            let git_repo = config["github"]["repo"].as_str().unwrap_or("");

            println!("Connecting to GitHub repository: {}/{}", git_owner, git_repo);


            
            println!("Creating an in memory database.");
            let mut convo = conversation::Conversation::new(Some("".to_string()), "deepseek-r1-distill-llama-70b".to_string(), Provider::Groq);
            

            // Read the tree structure from index.yaml
            let index_path = Path::new(".coder").join("index.yaml");
            let index_content = fs::read_to_string(index_path)?;
            let index: Value = serde_yaml::from_str(&index_content)?;
            // Get the tree structure
            let tree = index["tree"].as_str().unwrap_or("");

            // TODO: Replace with actual GitHub issue fetching
            let issue_title = "Fix error handling in cli.rs";
            let issue_body = "The error handling in cli.rs needs improvement. We should add proper error types.";

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

            println!("Intializing the inference gateway client.");
            let client = InferenceGatewayClient::new("http://localhost:8080");

            println!("Starting AI Coder agent...");
            println!("Press Ctrl+C to stop the agent.");
            loop {
                let resp = client.generate_content(Provider::Groq, model, convo.clone().try_into()?).await?;

                let files_requests = conversation::Conversation::parse_response(&resp.response.content);

                convo.add_message(Message {
                    role: MessageRole::Assistant,
                    content: resp.response.content,
                });

                let contents = index::extract_file_contents(&index_content);
                let review_prompt = prompt::Prompt::create_review_prompt(&files_requests, &contents);

                convo.add_message(Message {
                    role: MessageRole::User,
                    content: review_prompt,
                });

                let resp = client.generate_content(Provider::Groq, model, convo.clone().try_into()?).await?;

                convo.add_message(Message {
                    role: MessageRole::Assistant,
                    content: resp.response.content,
                });


                println!("{:?}", convo);


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
