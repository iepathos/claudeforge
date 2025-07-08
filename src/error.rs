use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClaudeForgeError {
    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    #[error("Failed to clone repository: {0}")]
    GitCloneError(String),

    #[error("Directory already exists: {0}")]
    DirectoryExists(PathBuf),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Git error: {0}")]
    GitError(#[from] git2::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Git not available: Please install git and try again")]
    GitNotAvailable,
}
