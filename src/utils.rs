use log::debug;

/// Removes the content between <think> blocks
pub fn strip_thinking(content: &str) -> Option<String> {
    let start_tag = "<think>";
    let end_tag = "</think>";

    let start = match content.find(start_tag) {
        Some(pos) => pos,
        None => return Some(content.to_string()),
    };

    let end = match content.find(end_tag) {
        Some(pos) => pos,
        None => return Some(content.to_string()),
    };

    if end < start {
        return None;
    }

    let before = &content[..start];
    let after_end = end + end_tag.len();
    let after = if after_end >= content.len() {
        ""
    } else {
        &content[after_end..]
    };

    // Log the content between think blocks in debug mode
    let thinking = &content[start + start_tag.len()..end];
    debug!("AI thinking process: {}", thinking);

    Some(format!("{}{}", before, after))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_thinking() {
        let input = r#"Something else. <think>
This should be removed
</think>
End"#;
        let expected = Some("Something else. \nEnd".to_string());

        let content = strip_thinking(input);
        assert!(content.is_some());
        assert_eq!(content, expected);
    }
}
