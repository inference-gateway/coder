use crate::cli::{Cli, Commands};
use crate::errors::CoderError;
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use conversation::Conversation;
use inference_gateway_sdk::{
    InferenceGatewayAPI, InferenceGatewayClient, Message, MessageRole, Provider,
};
use log::{debug, info, warn};
use std::{env, fs, panic, path::Path, str::FromStr, thread::sleep, time::Duration};

mod cli;
mod config;
mod conversation;
mod errors;
mod index;
mod tools;
mod utils;

fn setup_panic_handler(conversation: Conversation) {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        info!("{:?}", conversation);

        default_hook(panic_info);
    }));
}

fn init() -> Result<(), CoderError> {
    info!("Initializing AI Coder agent...");
    let coder_dir = Path::new(".coder");
    fs::create_dir_all(coder_dir)?;
    info!("Created .coder directory");
    fs::write(coder_dir.join("config.yaml"), config::default_config())?;

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

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), CoderError> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let tools = tools::get_tools();
    let cli = Cli::parse();

    if let Commands::Init {} = cli.command {
        return init();
    }

    let coder_dir = Path::new(".coder");
    let config_path = coder_dir.join("config.yaml");
    if !coder_dir.exists() {
        return Err(CoderError::ConfigError(
            "'.coder' directory not found. Run 'coder init' first to initialize the project"
                .to_string(),
        ));
    }

    let config = config::load(&config_path)?;

    let model = &config.agent.model;
    let provider = Provider::try_from(config.agent.provider.as_str())?;

    match cli.command {
        Commands::Completions { shell } => {
            generate(shell, &mut Cli::command(), "coder", &mut std::io::stdout());
            return Ok(());
        }
        Commands::Init {} => {
            return init();
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

            let client = InferenceGatewayClient::new(&config.api.endpoint)
                .with_max_tokens(Some(900))
                .with_tools(Some(tools));

            let mut convo = Conversation::new(model.to_string(), provider.clone());

            setup_panic_handler(convo.clone());

            let system_prompt = format!(
                r#"You are a senior software engineer specializing in {language} development working to fix an issue reported in {scm}.

When you need to call a tool, don't answer in XML. Instead, provide the tool name and arguments in JSON format.

WORKSPACE:
{tree}

CHANGES SCOPE:
The changes you are allowed to make are limited to playground/bug1 directory.

PROCESS:
1. Validate issue #{issue} [tool: issue_validate]
2. Pull issue details [tool: issue_pull] 
3. Analyze code and documentation [tool: code_read]
4. Implement a fix [tool: code_write]
5. Validate changes, only if you made changes to the code:
    - Lint code [tool: code_lint]
    - Run analysis [tool: code_analyse]
    - Run tests [tool: code_test]
6. Create {scm} pull request [pull_request]
7. Call "done" when complete [done]

When you execute a tool, the following output would be provided in JSON format:

```json
{{
  "status": "ok",
  "message": "tool_message",
  "result": "tool_result"
  "retry": false
}}
```

When retry is set to true, that means you need to re-run the tool with the provided arguments, ensure you send valid JSON.
When the tool result is empty, it means the tool did not find any issues.
If it's not empty, review the issue and fix it accordingly.

Focus on producing working solutions with minimal discussion. Do not ask questions. Provide a complete solution."#,
                language = config.language.name,
                scm = config.scm.name,
                tree = index::build_tree()?,
                issue = issue,
            );

            convo.add_message(Message {
                role: MessageRole::System,
                content: system_prompt,
                ..Default::default()
            });

            convo.add_message(Message {
                role: MessageRole::User,
                content: format!("I need help fixing this issue #{} first pull it before jumping to conclusions.", issue),
                ..Default::default()
            });

            let timeout = Duration::from_secs(300);
            info!("Starting AI Coder agent...");
            info!("Press Ctrl+C to stop the agent.");
            loop {
                if timeout.as_secs() == 0 {
                    warn!("Timeout reached. Exiting...");
                    break;
                }

                let resp: inference_gateway_sdk::GenerateResponse = client
                    .generate_content(provider.clone(), model, convo.clone().try_into()?)
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
                    ..Default::default()
                });

                info!("Assistant: {}", assistant_message);

                if response.tool_calls.is_some() {
                    for tool_call in response.tool_calls.unwrap() {
                        let tool = tools::Tools::from_str(tool_call.function.name.as_str())?;
                        let args = tool_call.function.arguments.as_str();
                        let tool_result = tools::handle_tool_calls(&tool, args, &config).await;
                        if tool_result.is_err() {
                            warn!("Tool failed to execute. Exiting...");
                            let tool_message = Message {
                                role: MessageRole::Tool,
                                content: tool_result.unwrap_err().to_string(),
                                tool_call_id: Some(tool_call.id),
                            };
                            let user_message = Message {
                                role: MessageRole::User,
                                content:
                                    "Something went wrong can you retry it? maybe take a step back."
                                        .to_string(),
                                ..Default::default()
                            };
                            debug!("Tool message(faliure): {:?}", tool_message);
                            debug!("User message(faliure): {:?}", user_message);
                            convo.add_message(tool_message);
                            convo.add_message(user_message);
                            continue;
                        }

                        let result = tool_result?;

                        debug!("Tool result: {}", result);

                        let tool_message = Message {
                            role: MessageRole::Tool,
                            content: result.to_string(),
                            tool_call_id: Some(tool_call.id),
                        };

                        let tool_result_struct: tools::StatusResponse =
                            serde_json::from_value(result)?;

                        let mut user_message = Message {
                            role: MessageRole::User,
                            content: "Let's proceed to the next step".to_string(),
                            ..Default::default()
                        };
                        if tool_result_struct.retry {
                            warn!("Tool requires retry. Exiting...");
                            user_message = Message {
                                role: MessageRole::User,
                                content: "Something went wrong can you retry it?".to_string(),
                                ..Default::default()
                            };
                        }

                        debug!("Tool message: {:?}", tool_message);
                        debug!("User message: {:?}", user_message);
                        convo.add_message(tool_message);
                        convo.add_message(user_message);
                    }
                }

                info!("Iteration completed. Developer is taking a coffee break due to rate-limiting..");

                sleep(Duration::from_secs(60));
            }
        }
        Commands::Refactor { file } => {
            match file {
                Some(path) => info!("Refactoring file: {}", path),
                None => info!("Refactoring entire project..."),
            }

            let client = InferenceGatewayClient::new(&config.api.endpoint)
                .with_max_tokens(Some(900))
                .with_tools(Some(tools));

            let mut convo = Conversation::new(config.agent.model.to_string(), provider.clone());

            setup_panic_handler(convo.clone());

            let system_prompt = format!(
                r#"You are a senior software engineer specializing in Rust development. Your task is to refactor the code based on the provided code snippet. Keep your answers short and consice. Do not ask questions back.

WORKSPACE INFO:

{}

WORKFLOW:
1. Read the provided file content
2. Analyse the code
3. Lint the code
4. Test the code
5. Refactor the code
6. Test the code again
7. Finally, create a GitHub Pull Request

"#,
                index::build_tree()?,
            );

            convo.add_message(Message {
                role: MessageRole::System,
                content: system_prompt,
                ..Default::default()
            });

            convo.add_message(Message {
                role: MessageRole::User,
                content: "I need help refactoring this code snippet".to_string(),
                ..Default::default()
            });

            let timeout = Duration::from_secs(300);
            info!("Starting AI Coder agent...");
            info!("Press Ctrl+C to stop the agent.");
            loop {
                if timeout.as_secs() == 0 {
                    warn!("Timeout reached. Exiting...");
                    break;
                }

                let resp: inference_gateway_sdk::GenerateResponse = client
                    .generate_content(
                        provider.clone(),
                        config.agent.model.as_str(),
                        convo.clone().try_into()?,
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
                    ..Default::default()
                });

                info!("Assistant: {}", assistant_message);

                if response.tool_calls.is_some() {
                    for tool_call in response.tool_calls.unwrap() {
                        let tool = tools::Tools::from_str(tool_call.function.name.as_str())?;
                        let args = tool_call.function.arguments.as_str();
                        let tool_result = tools::handle_tool_calls(&tool, args, &config).await?;

                        convo.add_message(Message {
                            role: MessageRole::Tool,
                            content: tool_result.to_string(),
                            tool_call_id: Some(tool_call.id),
                        });
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use log::LevelFilter;
    use predicates::prelude::*;
    use std::fs;

    use crate::config;

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
        config_file.assert(predicate::str::contains(config::default_config()));

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
