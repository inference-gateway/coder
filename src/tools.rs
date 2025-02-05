
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn setup_test_dir(content: &str) -> tempfile::TempDir {
        let dir = tempdir().unwrap();
        let coder_dir = dir.path().join(".coder");
        fs::create_dir(&coder_dir).unwrap();
        fs::write(coder_dir.join("index.yaml"), content).unwrap();
        dir
    }

    #[test]
    fn test_get_file_content_success() {
        let yaml_content = r#"
            content:
              "src/main.rs": "fn main() {}"
        "#;
        let dir = setup_test_dir(yaml_content);
        std::env::set_current_dir(&dir).unwrap();
        
        let result = get_file_content("src/main.rs");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "fn main() {}");
    }

    #[test]
    fn test_get_file_content_not_found() {
        let yaml_content = r#"
            content:
              "src/main.rs": "fn main() {}"
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
}