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
        #[arg(long, value_parser = parse_issue_number)]
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
    Refactor {},
}

// Helper function to parse issue number from #N format
fn parse_issue_number(s: &str) -> Result<u32, String> {
    let num = s
        .trim_start_matches('#')
        .parse::<u32>()
        .map_err(|_| format!("Invalid issue number: {}", s))?;

    if num == 0 {
        return Err("Issue number cannot be 0".into());
    }
    Ok(num)
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
    #[should_panic]
    fn test_fix_command_invalid_issue() {
        let args = ["coder", "fix", "--issue", "invalid"];
        let _cli = Cli::parse_from(args);
    }

    #[test]
    #[should_panic]
    fn test_fix_command_zero_issue() {
        let args = ["coder", "fix", "--issue", "#0"];
        let _cli = Cli::parse_from(args);
    }
}
