
/// Removes the content between <think> blocks
pub fn strip_thinking(content: &str) -> String {
    let mut result = String::new();
    let mut in_think_block = false;
    
    // Split content into lines to preserve formatting
    for line in content.lines() {
        if line.contains("<think>") {
            in_think_block = true;
            continue;
        }
        if line.contains("</think>") {
            in_think_block = false;
            continue;
        }
        
        // Only append lines that are not inside think blocks
        if !in_think_block {
            result.push_str(line);
            result.push('\n');
        }
    }
    
    // Remove trailing newline if present
    if result.ends_with('\n') {
        result.pop();
    }
    
    result
}