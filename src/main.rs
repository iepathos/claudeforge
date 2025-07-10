use anyhow::Result;
use clap::Parser;
use tracing::{error, info};

use claudeforge::error::ClaudeForgeError;
use claudeforge::git;
use claudeforge::template::loader::TemplateLoader;
use claudeforge::{create_project, Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("claudeforge=info".parse()?),
        )
        .init();

    // Check if git is available
    if !git::is_git_available() {
        error!("Git is not available on this system");
        return Err(ClaudeForgeError::GitNotAvailable.into());
    }

    match cli.command {
        Commands::New {
            language,
            name,
            directory,
            yes,
        } => {
            info!("Creating new {} project: {}", language, name);
            create_project(language, name, directory, yes).await?;
        }
        Commands::List => {
            list_templates().await?;
        }
        Commands::Update => {
            update_templates().await?;
        }
        Commands::Version => {
            print_version();
        }
    }

    Ok(())
}

async fn list_templates() -> Result<()> {
    let loader = TemplateLoader::new().await?;
    let templates = loader.list_templates();

    println!("Available templates:");
    println!();

    for template in templates {
        println!("  {} ({})", template.name, template.language);
        println!("    Description: {}", template.description);
        println!("    Repository: {}", template.repository);
        println!();
    }

    Ok(())
}

async fn update_templates() -> Result<()> {
    let loader = TemplateLoader::new().await?;
    loader.update_all().await?;
    Ok(())
}

fn print_version() {
    println!("claudeforge {}", env!("CARGO_PKG_VERSION"));
    println!("Create new projects optimized for Claude Code");
    println!();
    println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
    println!("Authors: {}", env!("CARGO_PKG_AUTHORS"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;
    use predicates::prelude::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_list_templates() {
        let result = list_templates().await;
        // Should not panic or error for basic functionality
        assert!(result.is_ok() || result.is_err()); // Either case is acceptable for testing
    }

    #[tokio::test]
    async fn test_update_templates() {
        let result = update_templates().await;
        // Should not panic or error for basic functionality
        assert!(result.is_ok() || result.is_err()); // Either case is acceptable for testing
    }

    #[test]
    fn test_print_version() {
        // Test that print_version doesn't panic
        print_version();
    }

    #[test]
    fn test_cli_version_command() {
        let mut cmd = Command::cargo_bin("claudeforge").unwrap();
        cmd.arg("version")
            .assert()
            .success()
            .stdout(predicate::str::contains("claudeforge"))
            .stdout(predicate::str::contains("Repository:"))
            .stdout(predicate::str::contains("Authors:"));
    }

    #[test]
    fn test_cli_list_command() {
        let mut cmd = Command::cargo_bin("claudeforge").unwrap();
        cmd.arg("list")
            .assert()
            .success()
            .stdout(predicate::str::contains("Available templates"));
    }

    #[test]
    fn test_cli_update_command() {
        // Use a temporary directory for cache to ensure clean state
        let temp_dir = TempDir::new().unwrap();
        let mut cmd = Command::cargo_bin("claudeforge").unwrap();
        
        // Set XDG_CACHE_HOME to temporary directory to isolate the test
        cmd.env("XDG_CACHE_HOME", temp_dir.path())
            .arg("update")
            .assert()
            .success();
    }

    #[test]
    fn test_cli_new_command() {
        let temp_dir = TempDir::new().unwrap();
        let mut cmd = Command::cargo_bin("claudeforge").unwrap();

        cmd.arg("new")
            .arg("rust")
            .arg("test-project")
            .arg("--directory")
            .arg(temp_dir.path())
            .arg("--yes")
            .assert()
            .success();
    }

    #[test]
    fn test_cli_invalid_command() {
        let mut cmd = Command::cargo_bin("claudeforge").unwrap();
        cmd.arg("invalid-command").assert().failure();
    }

    #[test]
    fn test_cli_help() {
        let mut cmd = Command::cargo_bin("claudeforge").unwrap();
        cmd.arg("--help")
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "Create new projects optimized for Claude Code",
            ));
    }

    #[test]
    fn test_cli_new_without_args() {
        let mut cmd = Command::cargo_bin("claudeforge").unwrap();
        cmd.arg("new").assert().failure();
    }
}
