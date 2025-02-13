use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "AI Coder Agent")]
#[command(about = "An AI Coder agent CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate shell completions
    Completions {
        /// The shell to generate completions for
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },

    /// Init the AI Coder agent.
    ///
    /// Uploads the relevant files(those which are not in .gitignore) as a collection to Open-WebUI via the API.
    Init {},

    /// Index the AI Coder agent.
    ///
    /// Indexes the files in the collection.
    Index {},

    /// Fix a bug from a given issue
    Fix {
        /// The issue number to fix (e.g. #14)
        #[arg(long, value_parser = validate_issue_number)]
        issue: u32,

        /// Additional instructions for fixing the issue
        #[arg(long)]
        further_instruction: Option<String>,
    },

    /// Refactor look on potential improvement to the project
    /// and interact with the user for further actions.
    ///
    /// Pulls the issues from Github or Gitlab.
    /// Runs the AI coder agent to generate a potential fix.
    /// Creates a pull request with the potential fix.
    Refactor {
        /// Optional file path to refactor specific file
        #[arg(long)]
        file: Option<String>,
    },
}

// Helper function to parse issue number from #N format
fn validate_issue_number(s: &str) -> Result<u32, String> {
    // Remove leading # if present
    let clean_issue = s.trim_start_matches('#');

    // Parse as integer
    match clean_issue.parse::<i32>() {
        Ok(num) if num > 0 => Ok(num as u32),
        Ok(0) => Err("Issue number cannot be 0".into()),
        Ok(_) => Err("Issue number cannot be negative".into()),
        Err(_) => Err(format!("Invalid issue number: {}", s)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_fix_command_valid() {
        let args = [
            "coder",
            "fix",
            "--issue",
            "#14",
            "--further-instruction",
            "please test",
        ];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Fix {
                issue,
                further_instruction,
            } => {
                assert_eq!(issue, 14);
                assert_eq!(further_instruction, Some("please test".to_string()));
            }
            _ => panic!("Expected Fix command"),
        }
    }

    #[test]
    fn test_fix_command_no_instructions() {
        let args = ["coder", "fix", "--issue", "#14"];
        let cli = Cli::parse_from(args);

        match cli.command {
            Commands::Fix {
                issue,
                further_instruction,
            } => {
                assert_eq!(issue, 14);
                assert_eq!(further_instruction, None);
            }
            _ => panic!("Expected Fix command"),
        }
    }

    #[test]
    fn test_fix_command_invalid_issue() {
        let args = ["coder", "fix", "--issue", "invalid"];
        let result = Cli::try_parse_from(args);

        assert!(result.is_err());
        assert!(result
            .err()
            .map(|e| e.to_string())
            .unwrap()
            .contains("Invalid issue number: invalid"));
    }

    #[test]
    fn test_issue_number_validation() {
        let result = validate_issue_number("invalid");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid issue number: invalid"
        );
        let result = validate_issue_number("#0");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Issue number cannot be 0");

        assert!(validate_issue_number("42").is_ok());
        assert!(validate_issue_number("#123").is_ok());

        let result = validate_issue_number("-1");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Issue number cannot be negative"
        );
    }
}
