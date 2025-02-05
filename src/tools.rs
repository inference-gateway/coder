use std::path::Path;

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

    // Ensure index file exists
    if !index_path.exists() {
        return Err(CoderError::ConfigError("Index file not found".to_string()));
    }

    // Read and parse index file
    let index_content = std::fs::read_to_string(index_path)
        .map_err(|e| CoderError::ConfigError(format!("Failed to read index file: {}", e)))?;

    let index: serde_yaml::Value = serde_yaml::from_str(&index_content)
        .map_err(|e| CoderError::ConfigError(format!("Failed to parse index file: {}", e)))?;

    // Extract content using safe navigation
    index
        .get("content")
        .and_then(|c| c.get(path))
        .and_then(|v| v.as_str())
        .map(String::from)
        .ok_or_else(|| CoderError::ConfigError(format!("Content not found for path: {}", path)))
}

/// Write file content to .coder/index.yaml and replaces the file on the specific location
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
    // Update index.yaml
    let index_path = std::path::Path::new(".coder").join("index.yaml");
    let mut index_content = std::fs::read_to_string(&index_path)?;
    let mut index: serde_yaml::Value = serde_yaml::from_str(&index_content)?;

    index["content"][path] = serde_yaml::Value::String(content.to_string());
    index_content = serde_yaml::to_string(&index)?;

    std::fs::write(index_path, index_content)?;

    // Write to actual file path
    let file_path = std::path::Path::new(path);
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(file_path, content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn setup_test_dir(content: &str) -> std::path::PathBuf {
        let dir = tempdir().unwrap();
        let path = dir.path().to_path_buf();
        let coder_dir = path.join(".coder");
        fs::create_dir_all(&coder_dir).unwrap();
        fs::write(coder_dir.join("index.yaml"), content).unwrap();
        std::mem::forget(dir);
        path
    }

    #[test]
    fn test_get_file_content_success() {
        let yaml_content = r#"
            content:
              src/main.rs: |
                fn main() {}

"#;
        let dir = setup_test_dir(yaml_content);
        std::env::set_current_dir(&dir).unwrap();

        let result = get_file_content("src/main.rs");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "fn main() {}\n");
        drop(dir);
    }

    #[test]
    fn test_get_file_content_missing_index() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        let result = get_file_content("src/main.rs");
        assert!(result.is_err());
        drop(dir);
    }

    #[test]
    fn test_write_file_content_success() {
        let dir: tempfile::TempDir = tempdir().unwrap();
        let path = dir.path().to_path_buf();
        std::env::set_current_dir(&path).unwrap();

        fs::create_dir(".coder").unwrap();
        fs::write(
            ".coder/index.yaml",
            "content:\n  existing.rs: \"old content\"\n",
        )
        .unwrap();

        let result = write_file_content("src/new.rs", "fn new() {}");
        assert!(result.is_ok());

        let index: serde_yaml::Value =
            serde_yaml::from_str(&fs::read_to_string(".coder/index.yaml").unwrap()).unwrap();
        assert_eq!(
            index["content"]["src/new.rs"].as_str().unwrap(),
            "fn new() {}"
        );

        assert_eq!(fs::read_to_string("src/new.rs").unwrap(), "fn new() {}");
        drop(dir);
    }
}
