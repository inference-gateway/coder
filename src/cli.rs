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
    
    /// Start the AI Coder agent.
    /// 
    /// Pulls the issues from Github or Gitlab.
    /// Runs the AI coder agent to generate a potential fix.
    /// Creates a pull request with the potential fix.
    Start {},
}