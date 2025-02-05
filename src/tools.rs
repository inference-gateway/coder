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
