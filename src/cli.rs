use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Create new projects optimized for Claude Code
#[derive(Parser, Debug)]
#[command(name = "claudeforge")]
#[command(about = "Create new projects optimized for Claude Code", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new project from a template
    New {
        /// Language template to use (rust, go)
        #[arg(value_enum)]
        language: Language,

        /// Project name
        name: String,

        /// Target directory (defaults to current directory)
        #[arg(short, long)]
        directory: Option<PathBuf>,

        /// Skip interactive prompts
        #[arg(short, long)]
        yes: bool,
    },

    /// List available templates
    List,

    /// Update cached templates
    Update,

    /// Show version information
    Version,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Language {
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "go")]
    Go,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Rust => write!(f, "rust"),
            Language::Go => write!(f, "go"),
        }
    }
}
