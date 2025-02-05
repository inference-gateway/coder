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
/// let content = read_file("src/main.rs");
/// ```
pub fn get_file_content(path: &str) -> Result<String, CoderError> {
    let index_path = std::path::Path::new(".coder").join("index.yaml");
    let index_content = std::fs::read_to_string(index_path)?;
    let index: serde_yaml::Value = serde_yaml::from_str(&index_content)?;

    let content = index["content"][path]
        .as_str()
        .ok_or_else(|| CoderError::ConfigError(format!("File {} not found in index", path)))?;

    Ok(content.to_string())
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
    }

    #[test]
    fn test_get_file_content_not_found() {
        let yaml_content = r#"
            content:
              src/main.rs: |
                fn main() {}
        "#;
        let dir = setup_test_dir(yaml_content);
        std::env::set_current_dir(&dir).unwrap();

        let result = get_file_content("src/nonexistent.rs");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CoderError::ConfigError(_)));
    }

    #[test]
    fn test_get_file_content_invalid_yaml() {
        let yaml_content = "invalid: yaml: content: ][";
        let dir = setup_test_dir(yaml_content);
        std::env::set_current_dir(&dir).unwrap();

        let result = get_file_content("src/main.rs");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_file_content_missing_index() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        let result = get_file_content("src/main.rs");
        assert!(result.is_err());
    }

    #[test]
    fn test_write_file_content_success() {
        let dir = tempdir().unwrap();
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
    }

    #[test]
    fn test_write_file_content_missing_index() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(&dir).unwrap();
    
        std::fs::create_dir(".coder").unwrap();
        std::fs::write(
            ".coder/index.yaml",
            "content:\n  {}\n"
        ).unwrap();
    
        let result = write_file_content("test.rs", "content");
        assert!(result.is_ok(), "Expected success when writing new file");
    
        assert_eq!(std::fs::read_to_string("test.rs").unwrap(), "content");
    
        let yaml_content = std::fs::read_to_string(".coder/index.yaml").unwrap();
        let index: serde_yaml::Value = serde_yaml::from_str(&yaml_content).unwrap();
        
        assert!(index.get("content").is_some(), "content key should exist");
        assert!(index["content"].get("test.rs").is_some(), "test.rs key should exist");
        assert_eq!(
            index["content"]["test.rs"].as_str(),
            Some("content"),
            "content value should match"
        );
    }

    #[test]
    fn test_write_file_content_invalid_yaml() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        fs::create_dir(".coder").unwrap();
        fs::write(".coder/index.yaml", "invalid: yaml: ][").unwrap();

        let result = write_file_content("test.rs", "content");
        assert!(result.is_err());
    }
}
