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

    Provide the entire relative path to the file.

    For example, if you need to review the file `src/main.rs`, you would type:

    REQUEST: src/main.rs
    
    I will provide the content of each requested file before we proceed with the fix."#,
            issue_title, issue_body, tree
        )
    }

    pub fn create_review_prompt(
        requested_files: &[String],
        contents: &HashMap<String, String>,
    ) -> String {
        let mut prompt =
            String::from("I have retrieved the requested files. Here are their contents:\n\n");

        for file in requested_files {
            if let Some(content) = contents.get(file) {
                prompt.push_str(&format!("FILE: {}\n```rust\n{}\n```\n\n", file, content));
            }
        }

        prompt.push_str("\nPlease review these files and suggest specific changes needed to fix the issue. Be precise and include the full corrected code if needed. If there are no files content simply answer that file contents is not provided.\n");

        prompt
    }

    pub fn get_system_message() -> String {
        format!("You are a software engineer tasked with reviewing and fixing a GitHub issue.")
    }

    pub fn create_improved_bug_report_suggestion(issue_body: &str, issue_title: &str) -> String {
        format!(r#"Don't immediately attempt to fix it. First try to understand the issue by asking the following questions:

1. What is the issue about?
2. What is the expected behavior?
3. What are the steps to reproduce the issue?

If all the above questions are answered, then you can proceed to fix the issue. If you want to proceed with the fix, answer with the following format:

Issue is decribed correctly, proceeding with the fix.

If not, please improve this GitHub issue while keeping its core meaning:

The Github Issue:
---

**Title**: {}
**Description**: {:?}

Ask follow-up questions if you need to clarify more details.
For example you can ask: "What is the issue about?" or "What is the expected behavior?"

In the following format: Question - <your question here>

The revised issue description should be in the following format:
---
**Title:** <improved title>
**Description:** <improved description>

## Summary
<summary of the issue>

## Expected Behavior
<expected behavior>

## Steps to Reproduce
<steps to reproduce>
---

Finally, if revised an improved version of the issue, ask the user to confirm the changes by answering with the following format:

Would you like to update the GitHub issue with this improved description? (y/N)

"#, issue_title, issue_body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_review_prompt_empty_contents() {
        let requested_files = vec!["file1.rs".to_string()];
        let contents = HashMap::new();

        let prompt = Prompt::create_review_prompt(&requested_files, &contents);

        let expected = r#"I have retrieved the requested files. Here are their contents:


Please review these files and suggest specific changes needed to fix the issue. Be precise and include the full corrected code if needed. If there are no files content simply answer that file contents is not provided.
"#;

        assert_eq!(prompt, expected);
    }

    #[test]
    fn test_create_review_prompt_no_files() {
        let requested_files: Vec<String> = vec![];
        let contents = HashMap::new();

        let prompt = Prompt::create_review_prompt(&requested_files, &contents);

        let expected = r#"I have retrieved the requested files. Here are their contents:


Please review these files and suggest specific changes needed to fix the issue. Be precise and include the full corrected code if needed. If there are no files content simply answer that file contents is not provided.
"#;

        assert_eq!(prompt, expected);
    }
}
