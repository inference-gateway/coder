use log::info;
use octocrab::Octocrab;
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use std::{
    fmt::{self, Display},
    path::Path,
    process::Command,
    str::FromStr,
};

use crate::errors::CoderError;

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
pub struct GithubPullIssueArgs {
    #[serde(deserialize_with = "deserialize_issue_number")]
    pub issue: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetFileContentArgs {
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WriteFileContentArgs {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GithubCreatePullRequestArgs {
    pub branch_name: String,
    pub issue: u64,
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone)]
pub enum Tool {
    GithubPullIssue,
    GithubCreatePullRequest,
    GetFileContent,
    WriteFileContent,
}

impl FromStr for Tool {
    type Err = CoderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "github_pull_issue" => Ok(Tool::GithubPullIssue),
            "github_create_pull_request" => Ok(Tool::GithubCreatePullRequest),
            "get_file_content" => Ok(Tool::GetFileContent),
            "write_file_content" => Ok(Tool::WriteFileContent),
            _ => Err(CoderError::ConfigError(format!("Invalid tool: {}", s))),
        }
    }
}

impl Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Tool::GithubPullIssue => "github_pull_issue",
            Tool::GithubCreatePullRequest => "github_create_pull_request",
            Tool::GetFileContent => "get_file_content",
            Tool::WriteFileContent => "write_file_content",
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
/// let content = get_file_content("src/main.rs");
/// ```
pub fn get_file_content(path: &str) -> Result<String, CoderError> {
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
/// * `github_owner` - Owner of the repository
/// * `github_repo` - Name of the repository  
/// * `branch_name` - Name of the branch
/// * `issue` - Issue number
/// * `title` - Title of the pull request
/// * `body` - Body of the pull request
///
/// # Returns
///
/// * `Result<octocrab::models::pulls::PullRequest, CoderError>` - Result of creating the pull request
pub async fn github_create_pull_request(
    github_owner: &str,
    github_repo: &str,
    branch_name: &str,
    issue: u64,
    title: &str,
    body: &str,
) -> Result<octocrab::models::pulls::PullRequest, CoderError> {
    let github_token = std::env::var("GITHUB_TOKEN")
        .map_err(|_| CoderError::ConfigError("GITHUB_TOKEN not set".to_string()))?;

    let octocrab = Octocrab::builder()
        .personal_token(github_token)
        .build()
        .map_err(|e| CoderError::GitHubError(e))?;

    Command::new("git")
        .args(["checkout", "-b", &branch_name])
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
        .args(["push", "origin", &branch_name])
        .output()
        .map_err(|e| CoderError::GitError(e.to_string()))?;

    let pr = octocrab
        .pulls(github_owner, github_repo)
        .create(title, branch_name, "main")
        .body(body)
        .send()
        .await
        .map_err(|e| CoderError::GitHubError(e))?;

    info!("Created PR: {}", pr.clone().html_url.unwrap());
    Ok(pr)
}

/// Pull issue from GitHub
///
/// # Arguments
///
/// * `issue_number` - Issue number
///
/// # Returns
///
/// * `Result<octocrab::models::issues::Issue, CoderError>` - Result of pulling the issue
pub async fn pull_github_issue(
    issue_number: u64,
    github_owner: &str,
    github_repo: &str,
) -> Result<octocrab::models::issues::Issue, CoderError> {
    let octocrab = Octocrab::builder()
        .personal_token(std::env::var("GITHUB_TOKEN").unwrap())
        .build()
        .map_err(|e| CoderError::GitHubError(e))?;

    let issue = octocrab
        .issues(github_owner, github_repo)
        .get(issue_number)
        .await
        .map_err(|e| CoderError::GitHubError(e))?;

    Ok(issue)
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
pub fn write_file_content(path: &str, content: &str) -> Result<(), CoderError> {
    let file_path = Path::new(path);
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(file_path, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, create_dir_all, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_get_file_content_success() -> Result<(), Box<dyn std::error::Error>> {
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

        let result = get_file_content("src/main.rs");
        assert!(result.is_ok());
        assert_eq!(result?, "fn main() {}\n");
        drop(file);
        drop(coder_dir);
        dir.close()?;
        Ok(())
    }

    #[test]
    fn test_get_file_content_missing_index() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        std::env::set_current_dir(&dir)?;

        let result = get_file_content("src/main.rs");
        assert!(result.is_err());
        dir.close()?;
        Ok(())
    }

    #[test]
    fn test_write_file_content_success() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempdir()?;
        let path = dir.path().to_path_buf();
        std::env::set_current_dir(&path)?;

        let src_dir = path.join("src");
        create_dir_all(&src_dir)?;
        let _result = write_file_content(src_dir.join("new.rs").to_str().unwrap(), "fn new() {}")?;

        let source_file_path = src_dir.join("new.rs");
        let fs_content = fs::read_to_string(&source_file_path)?;
        assert_eq!(fs_content, "fn new() {}");

        dir.close()?;
        Ok(())
    }
}
