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
    println!("Updating cached templates...");

    let loader = TemplateLoader::new().await?;
    loader.update_all().await?;

    println!("✅ All templates updated successfully!");
    Ok(())
}

fn print_version() {
    println!("claudeforge {}", env!("CARGO_PKG_VERSION"));
    println!("Create new projects optimized for Claude Code");
    println!();
    println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
    println!("Authors: {}", env!("CARGO_PKG_AUTHORS"));
}
