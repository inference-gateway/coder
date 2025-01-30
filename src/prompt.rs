use std::collections::HashMap;

pub struct Prompt;

impl Prompt {
    pub fn create_initial_prompt(tree: &str, issue_title: &str, issue_body: &str) -> String {
        format!(
            r#"
    ISSUE:
    Title: {}
    Description: {}
    
    PROJECT STRUCTURE:
    {}
    
    As a software engineer, please help me fix this issue:
    1. Review the project structure
    2. Tell me which files you need to examine to understand and fix the issue
    3. Wait for me to provide the content of each requested file
    4. Once you have reviewed the files, suggest the changes needed to fix the issue
    
    Please start by listing which files you need to review first. Request them one at a time using the format:
    REQUEST: <filename>
    
    I will provide the content of each requested file before we proceed with the fix."#,
            issue_title,
            issue_body,
            tree
        )
    }

    pub fn create_review_prompt(requested_files: &[String], contents: &HashMap<String, String>) -> String {
        let mut prompt = String::from("I have retrieved the requested files. Here are their contents:\n\n");
        
        for file in requested_files {
            if let Some(content) = contents.get(file) {
                prompt.push_str(&format!("FILE: {}\n```rust\n{}\n```\n\n", file, content));
            }
        }
        
        prompt.push_str("\nPlease review these files and suggest specific changes needed to fix the issue. Be precise and include the full corrected code if needed.\n");
        
        prompt
    }

    pub fn get_system_message() -> String {
        format!("You are a software engineer tasked with reviewing and fixing a GitHub issue.")
    }
}