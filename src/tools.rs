use inference_gateway_sdk::{Tool, ToolFunction, ToolType};
use log::{info, warn};
use octocrab::Octocrab;
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use serde_json::json;
use std::{
    fmt::{self, Display},
    path::Path,
    process::Command,
    str::FromStr,
};

use crate::config;
use crate::errors::CoderError;

// Tool structure for language-agnostic code fixes
#[derive(Debug, Clone)]
pub enum Tools {
    // Issue management
    IssueValidate, // Validate issue format
    IssuePull,     // Pull issue from repo

    // Code specific tools
    CodeRead,    // Read file content
    CodeWrite,   // Write file content
    CodeAnalyse, // Analyse code (language-specific)
    CodeLint,    // Lint code (language-specific)
    CodeTest,    // Run tests (language-specific)

    // Version control
    PullRequest, // Create PR with fixes

    // Documentation
    DocsReference, // Get documentation references

    // Done
    Done, // Done with the task
}

fn deserialize_issue_number<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Int(u64),
    }

    match StringOrInt::deserialize(deserializer)? {
        StringOrInt::String(s) => s.parse().map_err(D::Error::custom),
        StringOrInt::Int(i) => Ok(i),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocsReferenceArgs {
    pub term: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssuePullArgs {
    #[serde(deserialize_with = "deserialize_issue_number")]
    pub issue: u64,
    pub scm: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeReadArgs {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeWriteArgs {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PullRequestArgs {
    pub branch_name: String,
    #[serde(deserialize_with = "deserialize_issue_number")]
    pub issue: u64,
    pub title: String,
    pub body: String,
}

impl FromStr for Tools {
    type Err = CoderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "issue_validate" => Ok(Tools::IssueValidate),
            "issue_pull" => Ok(Tools::IssuePull),
            "pull_request" => Ok(Tools::PullRequest),
            "code_read" => Ok(Tools::CodeRead),
            "code_analyse" => Ok(Tools::CodeAnalyse),
            "code_lint" => Ok(Tools::CodeLint),
            "code_write" => Ok(Tools::CodeWrite),
            "code_test" => Ok(Tools::CodeTest),
            "docs_reference" => Ok(Tools::DocsReference),
            "done" => Ok(Tools::Done),
            _ => Err(CoderError::ConfigError(format!("Invalid tool: {}", s))),
        }
    }
}

impl Display for Tools {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Tools::IssueValidate => "issue_validate",
            Tools::IssuePull => "issue_pull",
            Tools::PullRequest => "pull_request",
            Tools::CodeRead => "code_read",
            Tools::CodeAnalyse => "code_analyse",
            Tools::CodeLint => "code_lint",
            Tools::CodeWrite => "code_write",
            Tools::CodeTest => "code_test",
            Tools::DocsReference => "docs_reference",
            Tools::Done => "done",
        };
        write!(f, "{}", s)
    }
}

/// Read file content from .coder/index.yaml
///
/// # Arguments
///
/// * `path` - Path to file
///
/// # Returns
///
/// * `String` - File content
///
/// # Example
///
/// ```
/// let content = code_read("src/main.rs");
/// ```
pub fn code_read(path: &str) -> Result<String, CoderError> {
    let index_path = Path::new(".coder/index.yaml");

    if !index_path.exists() {
        return Err(CoderError::ConfigError("Index file not found".to_string()));
    }

    let index_content = std::fs::read_to_string(index_path)
        .map_err(|e| CoderError::ConfigError(format!("Failed to read index file: {}", e)))?;

    let index: serde_yaml::Value = serde_yaml::from_str(&index_content)
        .map_err(|e| CoderError::ConfigError(format!("Failed to parse index file: {}", e)))?;

    index
        .get("content")
        .and_then(|c| c.get(path))
        .and_then(|v| v.as_str())
        .map(String::from)
        .ok_or_else(|| CoderError::ConfigError(format!("Content not found for path: {}", path)))
}

/// Create a pull request
///
/// # Arguments
///
/// * `scm_name` - Owner of the repository
/// * `owner` - Owner of the repository
/// * `repo` - Name of the repository  
/// * `branch_name` - Name of the branch
/// * `issue` - Issue number
/// * `title` - Title of the pull request
/// * `body` - Body of the pull request
///
/// # Returns
///
/// * `Result<octocrab::models::pulls::PullRequest, CoderError>` - Result of creating the pull request
pub async fn pull_request(
    scm_name: &str,
    owner: &str,
    repo: &str,
    branch_name: &str,
    issue: u64,
    title: &str,
    body: &str,
) -> Result<octocrab::models::pulls::PullRequest, CoderError> {
    if scm_name == "github" {
        info!(
            "Creating PR for issue #{} on branch {} with title: {}",
            issue, branch_name, title
        );
    } else {
        info!(
            "Creating a MR for issue #{} on branch {} with title: {}",
            issue, branch_name, title
        );
    }

    let github_token = std::env::var("GITHUB_TOKEN")
        .map_err(|_| CoderError::ConfigError("GITHUB_TOKEN not set".to_string()))?;

    let octocrab = Octocrab::builder()
        .personal_token(github_token)
        .build()
        .map_err(CoderError::GitHubError)?;

    Command::new("git")
        .args(["checkout", "-b", branch_name])
        .output()
        .map_err(|e| CoderError::GitError(e.to_string()))?;

    Command::new("git")
        .args(["add", "."])
        .output()
        .map_err(|e| CoderError::GitError(e.to_string()))?;

    Command::new("git")
        .args(["commit", "-m", &format!("fix: address issue #{}", issue)])
        .output()
        .map_err(|e| CoderError::GitError(e.to_string()))?;

    Command::new("git")
        .args(["push", "origin", branch_name])
        .output()
        .map_err(|e| CoderError::GitError(e.to_string()))?;

    let pr = octocrab
        .pulls(owner, repo)
        .create(title, branch_name, "main")
        .body(body)
        .send()
        .await
        .map_err(CoderError::GitHubError)?;

    Command::new("git")
        .args(["checkout", "main"])
        .output()
        .map_err(|e| CoderError::GitError(e.to_string()))?;

    Command::new("git")
        .args(["branch", "-D", branch_name])
        .output()
        .map_err(|e| CoderError::GitError(e.to_string()))?;

    info!("Created PR: {}", pr.clone().html_url.unwrap());
    Ok(pr)
}

/// Pull issue from SCN
///
/// # Arguments
///
/// * `scm_name` - The name of the SCM (e.g. github, gitlab)
/// * `issue_number` - Issue number
/// * `owner` - Owner of the repository
/// * `repo` - Name of the repository
///
/// # Returns
///
/// * `Result<octocrab::models::issues::Issue, CoderError>` - Result of pulling the issue
pub async fn issue_pull(
    scm_name: &str,
    issue_number: u64,
    owner: &str,
    repo: &str,
) -> Result<octocrab::models::issues::Issue, CoderError> {
    if scm_name == "github" {
        info!("Pulling issue #{} from GitHub", issue_number);
    } else {
        info!("Pulling MR #{} from GitLab", issue_number);
    }

    let octocrab = Octocrab::builder()
        .personal_token(std::env::var("GITHUB_TOKEN").unwrap())
        .build()
        .map_err(CoderError::GitHubError)?;

    let issue = octocrab
        .issues(owner, repo)
        .get(issue_number)
        .await
        .map_err(CoderError::GitHubError)?;

    Ok(issue)
}

pub fn issue_validate(
    config: &config::Config,
    issue_number: u64,
    issue_title: &str,
    issue_body: Option<String>,
) -> Result<(), CoderError> {
    if issue_number == 0 {
        warn!("Issue number cannot be 0");
        return Err(CoderError::ConfigError(
            "Issue number cannot be 0".to_string(),
        ));
    }

    if issue_title.trim().is_empty() {
        warn!("Issue title cannot be empty");
        return Err(CoderError::ConfigError(
            "Issue title cannot be empty".to_string(),
        ));
    }

    if let Some(template) = &config.scm.issue_template {
        let body = issue_body.ok_or_else(|| {
            warn!("Issue body is required");
            CoderError::ConfigError(
                "Issue body is required when template is configured".to_string(),
            )
        })?;

        for section in template.split("##").skip(1) {
            let section_name = section
                .lines()
                .next()
                .ok_or_else(|| CoderError::ConfigError("Invalid template section".to_string()))?
                .trim();

            if !body.contains(&format!("## {}", section_name)) {
                warn!("Missing required section: {}", section_name);
                return Err(CoderError::ConfigError(format!(
                    "Missing required section: {}",
                    section_name
                )));
            }
        }
    }

    info!("Validating issue: {}", issue_number);
    Ok(())
}

/// Write file content
///
/// # Arguments
///
/// * `path` - Path to file
/// * `content` - Content to write
///
/// # Returns
///
/// * `Result<(), CoderError>` - Result of writing the file content
pub fn code_write(path: &str, content: &str) -> Result<(), CoderError> {
    let file_path = Path::new(path);
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(file_path, content)?;
    Ok(())
}

/// Get documentation references
///
/// # Arguments
///
/// * `term` - Term to get documentation references for
///
/// # Returns
///
/// * `Result<(), CoderError>` - Result of getting documentation references
pub async fn docs_reference(term: &str) -> Result<(), CoderError> {
    info!("Getting documentation references for the term: {}", term);
    Ok(())
}

/// Done with the task
///
/// # Returns
///
/// * `Result<(), CoderError>` - Result of completing the task
pub fn done() -> Result<(), CoderError> {
    info!("Task completed");

    // TODO - perhaps add some cleanup code here
    Ok(())
}

pub fn get_tools() -> Vec<Tool> {
    vec![
        Tool {
            r#type: ToolType::Function,
            function: ToolFunction {
                name: Tools::IssueValidate.to_string(),
                description: "Validate the issue".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "scm": {
                            "type": "string",
                            "description": "SCM name lowercase"
                        },
                        "issue": {
                            "type": "number",
                            "description": "Issue number"
                        }
                    },
                    "required": ["scm", "issue"]
                }),
            },
        },
        Tool {
            r#type: ToolType::Function,
            function: ToolFunction {
                name: Tools::IssuePull.to_string(),
                description: "Pull the issue from SCM".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "scm": {
                            "type": "string",
                            "description": "SCM name lowercase"
                        },
                        "issue": {
                            "type": "number",
                            "description": "Issue number"
                        }
                    },
                    "required": ["scm", "issue"]
                }),
            },
        },
        Tool {
            r#type: ToolType::Function,
            function: ToolFunction {
                name: Tools::CodeLint.to_string(),
                description: "Lint the code".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
        },
        Tool {
            r#type: ToolType::Function,
            function: ToolFunction {
                name: Tools::CodeAnalyse.to_string(),
                description: "Analyse the code".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
        },
        Tool {
            r#type: ToolType::Function,
            function: ToolFunction {
                name: Tools::CodeTest.to_string(),
                description: "Test the code".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
        },
        Tool {
            r#type: ToolType::Function,
            function: ToolFunction {
                name: Tools::DocsReference.to_string(),
                description: "Reference the docs".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "term": {
                            "type": "string",
                            "description": "The term to search for"
                        }
                    },
                    "required": ["term"]
                }),
            },
        },
        Tool {
            r#type: ToolType::Function,
            function: ToolFunction {
                name: Tools::CodeRead.to_string(),
                description: "Read a code content".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "The path to the file"
                        }
                    },
                    "required": ["path"]
                }),
            },
        },
        Tool {
            r#type: ToolType::Function,
            function: ToolFunction {
                name: Tools::CodeWrite.to_string(),
                description: "Write code to a file".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "The path to the file"
                        },
                        "content": {
                            "type": "string",
                            "description": "The content to write"
                        }
                    },
                    "required": ["path", "content"]
                }),
            },
        },
        Tool {
            r#type: ToolType::Function,
            function: ToolFunction {
                name: Tools::PullRequest.to_string(),
                description: "Create a Pull Request".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "branch_name": {
                            "type": "string",
                            "description": "The branch name"
                        },
                        "issue": {
                            "type": "number",
                            "description": "The issue number"
                        },
                        "title": {
                            "type": "string",
                            "description": "The pull request title"
                        },
                        "body": {
                            "type": "string",
                            "description": "The pull request body"
                        },

                    },
                    "required": ["branch_name", "issue", "title", "body"]
                }),
            },
        },
        Tool {
            r#type: ToolType::Function,
            function: ToolFunction {
                name: Tools::Done.to_string(),
                description: "Finish the task".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
        },
    ]
}

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    status: String,
    message: Option<String>,
    result: Option<serde_json::Value>,
    retry: bool,
}

/// Execute a language-specific command from config
pub async fn execute_language_specific_command(
    config: &config::LanguageConfig,
    command_type: CommandType,
) -> Result<StatusResponse, CoderError> {
    let command = match command_type {
        CommandType::Lint => config.linter.first(),
        CommandType::Analyse => config.analyse.first(),
        CommandType::Test => config.test_commands.first(),
    }
    .ok_or_else(|| {
        CoderError::ConfigError(format!(
            "No {} command configured for language {}",
            command_type, config.name
        ))
    })?;

    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Err(CoderError::ConfigError("Empty command".to_string()));
    }

    let output = Command::new(parts[0])
        .args(&parts[1..])
        .output()
        .map_err(|e| CoderError::CommandError(e.to_string()))?;

    if !output.status.success() {
        return Err(CoderError::CommandError(format!(
            "Command '{}' failed: {}",
            command,
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    Ok(StatusResponse {
        status: "ok".to_string(),
        message: Some(format!("Command '{}' succeeded", command)),
        result: Some(serde_json::Value::String(
            String::from_utf8_lossy(&output.stdout).to_string(),
        )),
        retry: false,
    })
}

#[derive(Debug)]
pub enum CommandType {
    Analyse,
    Lint,
    Test,
}

impl Display for CommandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandType::Lint => write!(f, "lint"),
            CommandType::Analyse => write!(f, "analyse"),
            CommandType::Test => write!(f, "test"),
        }
    }
}

pub async fn handle_tool_calls(
    tool: &Tools,
    args: Option<&str>,
    config: &config::Config,
) -> Result<serde_json::Value, CoderError> {
    info!(
        "Handling tool call: {} with args: {}",
        tool,
        args.unwrap_or_default()
    );
    match tool {
        Tools::CodeRead => {
            let args = args.ok_or_else(|| {
                CoderError::MissingArguments("CodeRead requires arguments".to_string())
            })?;
            let args: CodeReadArgs = serde_json::from_str(args)?;
            let content = code_read(&args.path)?;
            let response = StatusResponse {
                status: "ok".to_string(),
                message: Some("Code read".to_string()),
                result: Some(serde_json::to_value(content)?),
                retry: false,
            };
            Ok(serde_json::to_value(response)?)
        }
        Tools::CodeWrite => {
            let args = args.ok_or_else(|| {
                CoderError::MissingArguments("CodeWrite requires arguments".to_string())
            })?;
            let args: CodeWriteArgs = serde_json::from_str(args)?;
            code_write(&args.path, &args.content)?;
            let mut retry = false;
            let output = Command::new("git")
                .args(["diff", "--exit-code", "--", &args.path])
                .output()
                .map_err(|e| CoderError::GitError(e.to_string()))?;
            if !output.status.success() {
                warn!("File was not written: {}", args.path);
                retry = true;
            }
            let response = StatusResponse {
                status: "ok".to_string(),
                message: Some("Code written".to_string()),
                result: None,
                retry,
            };
            Ok(serde_json::to_value(response)?)
        }
        Tools::IssueValidate => {
            let args = args.ok_or_else(|| {
                CoderError::MissingArguments("IssueValidate requires arguments".to_string())
            })?;
            let args: IssuePullArgs = serde_json::from_str(args)?;
            let issue = issue_pull(
                &config.scm.name,
                args.issue,
                &config.scm.owner,
                &config.scm.repository,
            )
            .await?;
            issue_validate(config, issue.number, &issue.title, issue.body.clone())?;
            #[derive(Debug, Serialize)]
            struct SanitizedIssue {
                number: u64,
                title: String,
                body: Option<String>,
            }
            let sanitized = SanitizedIssue {
                number: issue.number,
                title: issue.title,
                body: issue.body,
            };
            let response = StatusResponse {
                status: "ok".to_string(),
                message: Some("Issue validated".to_string()),
                result: Some(serde_json::to_value(sanitized)?),
                retry: false,
            };
            Ok(serde_json::to_value(response)?)
        }
        Tools::IssuePull => {
            let args = args.ok_or_else(|| {
                CoderError::MissingArguments("IssuePull requires arguments".to_string())
            })?;
            let args: IssuePullArgs = serde_json::from_str(args)?;
            let issue = issue_pull(
                &config.scm.name,
                args.issue,
                &config.scm.owner,
                &config.scm.repository,
            )
            .await?;
            #[derive(Debug, Serialize)]
            struct SanitizedIssue {
                number: u64,
                title: String,
                body: Option<String>,
            }
            let sanitized = SanitizedIssue {
                number: issue.number,
                title: issue.title,
                body: issue.body,
            };
            let response = StatusResponse {
                status: "ok".to_string(),
                message: Some("Issue pulled".to_string()),
                result: Some(serde_json::to_value(sanitized)?),
                retry: false,
            };
            Ok(serde_json::to_value(response)?)
        }
        Tools::PullRequest => {
            let args = args.ok_or_else(|| {
                CoderError::MissingArguments("PullRequest requires arguments".to_string())
            })?;
            let args: PullRequestArgs = serde_json::from_str(args)?;
            let pr = pull_request(
                &config.scm.name,
                &config.scm.owner,
                &config.scm.repository,
                &args.branch_name,
                args.issue,
                &args.title,
                &args.body,
            )
            .await?;
            #[derive(Debug, Serialize)]
            struct SanitizedPullRequest {
                number: u64,
                title: Option<String>,
                body: Option<String>,
            }
            let sanitized = SanitizedPullRequest {
                number: pr.number,
                title: pr.title,
                body: pr.body,
            };
            let response = StatusResponse {
                status: "ok".to_string(),
                message: Some("Pull request created".to_string()),
                result: Some(serde_json::to_value(sanitized)?),
                retry: false,
            };
            Ok(serde_json::to_value(response)?)
        }
        Tools::CodeLint => {
            let response =
                execute_language_specific_command(&config.language, CommandType::Lint).await?;
            Ok(serde_json::to_value(response)?)
        }
        Tools::CodeAnalyse => {
            let response =
                execute_language_specific_command(&config.language, CommandType::Analyse).await?;
            Ok(serde_json::to_value(response)?)
        }
        Tools::CodeTest => {
            let response =
                execute_language_specific_command(&config.language, CommandType::Test).await?;
            Ok(serde_json::to_value(response)?)
        }
        Tools::DocsReference => {
            let args = args.ok_or_else(|| {
                CoderError::MissingArguments("DocsReference requires arguments".to_string())
            })?;
            let args: DocsReferenceArgs = serde_json::from_str(args)?;
            let response = docs_reference(&args.term).await?;
            Ok(serde_json::to_value(response)?)
        }
        Tools::Done => {
            done()?;
            let response = StatusResponse {
                status: "ok".to_string(),
                message: Some("Task completed".to_string()),
                result: None,
                retry: false,
            };
            Ok(serde_json::to_value(response)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, create_dir_all, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_code_read_success() -> Result<(), Box<dyn std::error::Error>> {
        let yaml_content = r#"
content:
  src/main.rs: |
    fn main() {}

"#;
        let dir = tempdir()?;
        let path = dir.path().to_path_buf();
        let coder_dir = path.join(".coder");

        fs::create_dir_all(&coder_dir)?;
        let mut file = File::create(coder_dir.join("index.yaml"))?;
        write!(file, "{}", yaml_content)?;

        std::env::set_current_dir(&dir)?;

        let result = code_read("src/main.rs");
        assert!(result.is_ok());
        assert_eq!(result?, "fn main() {}\n");
        drop(file);
        drop(coder_dir);
        dir.close()?;
        Ok(())
    }

    #[test]
    fn test_code_read_missing_index() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        std::env::set_current_dir(&dir)?;

        let result = code_read("src/main.rs");
        assert!(result.is_err());
        dir.close()?;
        Ok(())
    }

    #[test]
    fn test_code_write_success() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let path = dir.path().to_path_buf();
        std::env::set_current_dir(&path)?;

        let src_dir = path.join("src");
        create_dir_all(&src_dir)?;
        code_write(src_dir.join("new.rs").to_str().unwrap(), "fn new() {}")?;

        let source_file_path = src_dir.join("new.rs");
        let fs_content = fs::read_to_string(&source_file_path)?;
        assert_eq!(fs_content, "fn new() {}");

        dir.close()?;
        Ok(())
    }
}
