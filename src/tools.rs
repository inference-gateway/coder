use octocrab::Octocrab;
use std::{path::Path, process::Command};

use crate::errors::CoderError;
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
/// * `title` - Title of the pull request
/// * `body` - Body of the pull request
/// * `head` - Head branch
/// * `base` - Base branch
///
/// # Returns
///
/// * `Result<octocrab::models::pulls::PullRequest, CoderError>` - Result of creating the pull request
// pub fn create_pull_request(
//     branch_name: &str,
//     issue: u64,
//     title: &str,
//     body: &str,
//     head: &str,
//     base: &str,
// ) -> Result<octocrab::models::pulls::PullRequest, CoderError> {
//     let client = octocrab::instance();
//     let octocrab = Octocrab::builder()
//         .personal_token(github_token)
//         .build()
//         .map_err(|e| CoderError::GitHubError(e))?;
//     let github_token = std::env::var("GITHUB_TOKEN")
//         .map_err(|_| CoderError::ConfigError("GITHUB_TOKEN not set".to_string()))?;

//     Command::new("git")
//         .args(["checkout", "-b", &branch_name])
//         .output()
//         .map_err(|e| CoderError::GitError(e.to_string()))?;

//     // Commit changes
//     Command::new("git")
//         .args(["add", "."])
//         .output()
//         .map_err(|e| CoderError::GitError(e.to_string()))?;

//     Command::new("git")
//         .args(["commit", "-m", &format!("fix: address issue #{}", issue)])
//         .output()
//         .map_err(|e| CoderError::GitError(e.to_string()))?;

//     // Push branch
//     Command::new("git")
//         .args(["push", "origin", &branch_name])
//         .output()
//         .map_err(|e| CoderError::GitError(e.to_string()))?;

//     // Create PR using octocrab
//     let pr = octocrab
//         .pulls(git_owner, git_repo)
//         .create(format!("Fix issue #{}", issue), branch_name, "main")
//         .body(format!(
//             "This PR addresses issue #{}. Please review the changes.",
//             issue
//         ))
//         .send()
//         .await
//         .map_err(|e| CoderError::GitHubError(e))?;

//     info!("Created PR: {}", pr.html_url.unwrap());
//     Ok(pr)
// }

/// Pull issue from GitHub
///
/// # Arguments
///
/// * `issue_number` - Issue number
///
/// # Returns
///
/// * `Result<octocrab::models::issues::Issue, CoderError>` - Result of pulling the issue
pub async fn pull_issue(issue_number: u64) -> Result<octocrab::models::issues::Issue, CoderError> {
    let octocrab = Octocrab::builder()
        .personal_token(std::env::var("GITHUB_TOKEN").unwrap())
        .build()
        .map_err(|e| CoderError::GitHubError(e))?;

    let issue = octocrab
        .issues("octocrab", "octocrab")
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
