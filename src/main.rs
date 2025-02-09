use crate::cli::{Cli, Commands};
use crate::errors::CoderError;
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use config::DEFAULT_CONFIG_TEMPLATE;
use inference_gateway_sdk::{
    InferenceGatewayAPI, InferenceGatewayClient, Message, MessageRole, Provider, Tool,
    ToolFunction, ToolType,
};
use log::{debug, info, warn};
use serde_json::json;
use serde_yaml::Value;
use std::str::FromStr;
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

            let client = InferenceGatewayClient::new(
                &config["api"]["endpoint"].as_str().unwrap_or_default(),
            );

            let mut convo = conversation::Conversation::new(
                "deepseek-r1-distill-llama-70b".to_string(),
                Provider::Groq,
            );

            let system_prompt = format!(
                r#"You are a senior software engineer specializing in Rust development. Your task is to diagnose and fix bugs based on a Github issue.

WORKSPACE INFO:
- Project Structure:
{}

WORKFLOW:
1. Pull the issue from GitHub
2. Analyze the issue description to identify the root cause
3. At any time if you need to execute a tool, type the tool name followed by the required arguments, for example: `pull_github_issue issue=123`
4. Review the code and provide a fix
5. Submit the fix as a pull request to the repository
6. Wait for the pull request to be reviewed and merged
7. If there is a new comment on the pull request, analyze the comment and provide a response

FURTHER INSTRUCTIONS:
{}

"#,
                index::build_tree()?,
                if let Some(instr) = &further_instruction {
                    format!("Additional Instructions: {}", instr)
                } else {
                    String::new()
                }
            );

            convo.add_message(Message {
                role: MessageRole::System,
                content: system_prompt,
                tool_call_id: None,
            });

            let tools = vec![Tool {
                r#type: ToolType::Function,
                function: ToolFunction {
                    name: tools::Tool::GithubPullIssue.to_string(),
                    description: "Pull issue from GitHub".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "issue": {
                                "type": "number",
                                "description": "Issue number"
                            }
                        },
                        "required": ["issue"]
                    }),
                },
            }];

            convo.add_message(Message {
                role: MessageRole::User,
                content: format!("I need help fixing this issue #{}", issue),
                tool_call_id: None,
            });

            let timeout = Duration::from_secs(300);
            info!("Starting AI Coder agent...");
            info!("Press Ctrl+C to stop the agent.");
            loop {
                if timeout.as_secs() == 0 {
                    warn!("Timeout reached. Exiting...");
                    break;
                }

                let resp = client
                    .generate_content(
                        Provider::Groq,
                        "deepseek-r1-distill-llama-70b",
                        convo.clone().try_into()?,
                        Some(tools.clone()),
                    )
                    .await?;

                let response = resp.response;

                let assistant_message = utils::strip_thinking(&response.content);
                if assistant_message.is_none() {
                    warn!("Assistant message is empty. Exiting...");
                    break;
                }

                let assistant_message = assistant_message.unwrap().trim().to_string();

                convo.add_message(Message {
                    role: MessageRole::Assistant,
                    content: assistant_message.clone(),
                    tool_call_id: None,
                });

                debug!("{:?}", assistant_message);

                if response.tool_calls.is_some() {
                    let tools: Vec<inference_gateway_sdk::ToolCallResponse> =
                        response.tool_calls.unwrap();
                    for tool_call_response in tools {
                        let function_name = tool_call_response.function.name.as_str();
                        let function_arguments = tool_call_response.function.arguments;
                        let tool = tools::Tool::from_str(function_name)?;
                        info!("Using tool {:?}", tool);
                        match tool {
                            tools::Tool::GithubPullIssue => {
                                let function_args =
                                    function_arguments.as_str().ok_or_else(|| {
                                        CoderError::MissingArguments(
                                            "Function arguments not provided".to_string(),
                                        )
                                    })?;
                                let args: tools::GithubPullIssueArgs =
                                    serde_json::from_str(function_args)?;
                                info!(
                                    "Pulling issue #{:?} from GitHub {}/{}...",
                                    args.issue, git_owner, git_repo
                                );
                                let github_issue =
                                    tools::pull_github_issue(args.issue, git_owner, git_repo)
                                        .await?;
                                info!("Found issue: {}", github_issue.title);
                                info!("Description: {:?}", github_issue.body);
                                convo.add_message(Message {
                                    role: MessageRole::Tool,
                                    content: format!("Found issue: {}", github_issue.title),
                                    tool_call_id: Some(tool_call_response.id.clone()),
                                });
                                convo.add_message(Message {
                                    role: MessageRole::Tool,
                                    content: format!("Description: {:?}", github_issue.body),
                                    tool_call_id: Some(tool_call_response.id),
                                });
                            }
                            tools::Tool::GetFileContent => {
                                let path = function_arguments["path"]
                                    .as_str()
                                    .ok_or(CoderError::ToolError("Path not found".to_string()))?;
                                info!("Reading content from file: {}", path);
                                let content = tools::get_file_content(path)?;
                                convo.add_message(Message {
                                    role: MessageRole::Tool,
                                    content: format!("Reading content from file: {}", path),
                                    tool_call_id: Some(tool_call_response.id.clone()),
                                });
                                convo.add_message(Message {
                                    role: MessageRole::Tool,
                                    content: format!("Content: {}", content),
                                    tool_call_id: Some(tool_call_response.id),
                                });
                            }
                            tools::Tool::WriteFileContent => {
                                let path = function_arguments["path"]
                                    .as_str()
                                    .ok_or(CoderError::ToolError("Path not found".to_string()))?;
                                let content = function_arguments["content"].as_str().ok_or(
                                    CoderError::ToolError("Content not found".to_string()),
                                )?;
                                info!("Writing content to file: {}", path);
                                tools::write_file_content(path, content)?;
                                convo.add_message(Message {
                                    role: MessageRole::Tool,
                                    content: format!("Writing content to file: {}", path),
                                    tool_call_id: Some(tool_call_response.id),
                                });
                            }
                            tools::Tool::GithubCreatePullRequest => {
                                info!("Creating pull request...");
                                // let branch_name = tool_call_response.function.parameters["branch"]
                                //     .as_str()
                                //     .ok_or(CoderError::ToolError(
                                //         "Branch name not found".to_string(),
                                //     ))?;
                                // let issue = tool_call_response.function.parameters["issue"]
                                //     .as_u64()
                                //     .ok_or(CoderError::ToolError(
                                //         "Issue number not found".to_string(),
                                //     ))?;
                                // info!("Creating pull request for issue #{}...", issue);
                                // let pr = tools::github_create_pull_request(
                                //     git_owner,
                                //     git_repo,
                                //     format!("Fix issue #{}", issue),
                                //     branch_name,
                                //     "main",
                                //     format!(
                                //         "This PR addresses issue #{}. Please review the changes.",
                                //         issue
                                //     ),
                                // )
                                // .await?;
                                // convo.add_message(Message {
                                //     role: MessageRole::Tool,
                                //     content: format!(
                                //         "Creating pull request for issue #{}...",
                                //         issue
                                //     ),
                                // });
                            }
                            _ => {}
                        }
                    }
                }

                // TODO - Instead of sleeping the agent supposed to wait for user input on the Pull Request comments
                // Each pull request comment should be sent to the agent for further processing

                info!("Convo: {:?}", convo);
                info!("Iteration completed. Waiting for the next instruction...");

                sleep(Duration::from_secs(5));
            }
        }
        Commands::Refactor {} => {}
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
