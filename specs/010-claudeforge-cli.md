# Spec 010: ClaudeForge CLI

## Feature Summary

ClaudeForge is a command-line tool that streamlines the creation of new projects optimized for development with Claude Code. It provides a simple interface to scaffold projects in multiple languages using curated templates that include comprehensive AI development guidelines, proper gitignore configurations, and best practices baked into the project structure.

The tool addresses the need for developers to quickly bootstrap projects that are pre-configured for AI-assisted development, eliminating the manual setup of CLAUDE.md files, project structures, and development configurations. It ensures consistency across projects and helps developers leverage Claude Code more effectively from the start.

## Goals & Requirements

### Functional Requirements
- Initialize new projects from language-specific templates (Rust, Go, with extensibility for more)
- Clone and configure templates from remote repositories
- Support both interactive and command-line argument modes
- Clean up git history from templates and initialize fresh repositories
- Allow custom project names and locations
- Provide template listing and descriptions
- Support offline mode with cached templates

### Non-functional Requirements
- Fast project initialization (< 5 seconds for typical project)
- Cross-platform compatibility (macOS, Linux, Windows)
- Minimal dependencies
- Clear error messages and recovery options
- Extensible architecture for adding new templates

### Success Criteria
- Projects can be created with a single command
- Generated projects immediately work with Claude Code
- All template files are properly configured for the new project
- Git repository is initialized with clean history
- Project-specific placeholders are replaced with actual values

## API/Interface Design

### Command Structure
```rust
/// Main CLI structure
#[derive(Parser, Debug)]
#[command(name = "claudeforge")]
#[command(about = "Create new projects optimized for Claude Code", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
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

#[derive(Debug, Clone, ValueEnum)]
enum Language {
    Rust,
    Go,
}
```

### Core Types
```rust
/// Template configuration
#[derive(Debug, Serialize, Deserialize)]
struct Template {
    name: String,
    language: Language,
    repository: String,
    description: String,
    files_to_customize: Vec<FileCustomization>,
}

/// File customization rules
#[derive(Debug, Serialize, Deserialize)]
struct FileCustomization {
    path: String,
    replacements: Vec<Replacement>,
}

/// Text replacement rule
#[derive(Debug, Serialize, Deserialize)]
struct Replacement {
    placeholder: String,
    value_type: ValueType,
}

#[derive(Debug, Serialize, Deserialize)]
enum ValueType {
    ProjectName,
    ProjectPath,
    AuthorName,
    AuthorEmail,
    CurrentDate,
    Custom(String),
}
```

### Error Types
```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum ClaudeForgeError {
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
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}
```

## File and Package Structure

```
claudeforge/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library exports
│   ├── cli.rs               # CLI argument parsing
│   ├── template/
│   │   ├── mod.rs           # Template management
│   │   ├── loader.rs        # Template loading/caching
│   │   ├── processor.rs     # File processing/customization
│   │   └── registry.rs      # Template registry
│   ├── git/
│   │   ├── mod.rs           # Git operations
│   │   └── clone.rs         # Repository cloning
│   ├── config/
│   │   ├── mod.rs           # Configuration management
│   │   └── templates.toml   # Built-in template definitions
│   └── utils/
│       ├── mod.rs           # Utility functions
│       └── fs.rs            # File system helpers
├── templates/               # Embedded template files (fallback)
│   ├── rust/
│   └── go/
└── tests/
    ├── integration/
    └── unit/
```

## Implementation Details

### Step 1: CLI Setup
```rust
// main.rs
use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    match cli.command {
        Commands::New { language, name, directory, yes } => {
            create_project(language, name, directory, yes)?;
        }
        Commands::List => {
            list_templates()?;
        }
        Commands::Update => {
            update_templates()?;
        }
        Commands::Version => {
            print_version();
        }
    }
    
    Ok(())
}
```

### Step 2: Template Loading
```rust
// template/loader.rs
use std::path::PathBuf;
use tokio::fs;

pub struct TemplateLoader {
    cache_dir: PathBuf,
    templates: HashMap<Language, Template>,
}

impl TemplateLoader {
    pub async fn new() -> Result<Self> {
        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| anyhow!("Failed to find cache directory"))?
            .join("claudeforge");
        
        fs::create_dir_all(&cache_dir).await?;
        
        let templates = load_template_registry()?;
        
        Ok(Self { cache_dir, templates })
    }
    
    pub async fn get_or_fetch(&self, language: Language) -> Result<PathBuf> {
        let template = self.templates.get(&language)
            .ok_or_else(|| ClaudeForgeError::TemplateNotFound(language.to_string()))?;
        
        let template_path = self.cache_dir.join(&template.name);
        
        if !template_path.exists() {
            self.fetch_template(template).await?;
        }
        
        Ok(template_path)
    }
}
```

### Step 3: Project Creation
```rust
// template/processor.rs
pub async fn create_project(
    language: Language,
    name: String,
    directory: Option<PathBuf>,
    skip_prompts: bool,
) -> Result<()> {
    let loader = TemplateLoader::new().await?;
    let template_path = loader.get_or_fetch(language).await?;
    
    let target_dir = directory.unwrap_or_else(|| PathBuf::from(".")).join(&name);
    
    if target_dir.exists() && !skip_prompts {
        confirm_overwrite(&target_dir)?;
    }
    
    // Copy template files
    copy_template(&template_path, &target_dir).await?;
    
    // Customize files
    let template = loader.get_template(language)?;
    customize_project_files(&target_dir, &name, &template).await?;
    
    // Initialize git repository
    initialize_git_repo(&target_dir).await?;
    
    println!("✅ Project '{}' created successfully!", name);
    println!("📁 Location: {}", target_dir.display());
    println!("🚀 Get started with: cd {} && claude code .", name);
    
    Ok(())
}
```

### Step 4: File Customization
```rust
// template/processor.rs
async fn customize_project_files(
    project_dir: &Path,
    project_name: &str,
    template: &Template,
) -> Result<()> {
    let replacements = build_replacements(project_name)?;
    
    for customization in &template.files_to_customize {
        let file_path = project_dir.join(&customization.path);
        
        if file_path.exists() {
            let content = fs::read_to_string(&file_path).await?;
            let new_content = apply_replacements(&content, &replacements);
            fs::write(&file_path, new_content).await?;
        }
    }
    
    Ok(())
}

fn build_replacements(project_name: &str) -> Result<HashMap<String, String>> {
    let mut replacements = HashMap::new();
    
    replacements.insert("{{PROJECT_NAME}}".to_string(), project_name.to_string());
    replacements.insert("{{CURRENT_DATE}}".to_string(), chrono::Local::now().format("%Y-%m-%d").to_string());
    
    // Get git config for author info
    if let Ok(output) = std::process::Command::new("git")
        .args(&["config", "user.name"])
        .output() 
    {
        let author = String::from_utf8_lossy(&output.stdout).trim().to_string();
        replacements.insert("{{AUTHOR_NAME}}".to_string(), author);
    }
    
    Ok(replacements)
}
```

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_build_replacements() {
        let replacements = build_replacements("my-project").unwrap();
        assert_eq!(replacements.get("{{PROJECT_NAME}}").unwrap(), "my-project");
        assert!(replacements.contains_key("{{CURRENT_DATE}}"));
    }
    
    #[tokio::test]
    async fn test_template_loader() {
        let loader = TemplateLoader::new().await.unwrap();
        assert!(loader.templates.contains_key(&Language::Rust));
        assert!(loader.templates.contains_key(&Language::Go));
    }
}
```

### Integration Tests
```rust
// tests/integration/create_project.rs
#[tokio::test]
async fn test_create_rust_project() {
    let temp_dir = tempfile::tempdir().unwrap();
    let project_name = "test-rust-project";
    
    create_project(
        Language::Rust,
        project_name.to_string(),
        Some(temp_dir.path().to_path_buf()),
        true,
    ).await.unwrap();
    
    let project_dir = temp_dir.path().join(project_name);
    assert!(project_dir.exists());
    assert!(project_dir.join("Cargo.toml").exists());
    assert!(project_dir.join("CLAUDE.md").exists());
    assert!(project_dir.join(".git").exists());
}
```

## Edge Cases & Error Handling

### Directory Already Exists
- Prompt user for confirmation before overwriting
- Provide --force flag to skip confirmation
- Ensure partial files are cleaned up on failure

### Network Failures
- Fall back to embedded templates if available
- Cache templates locally for offline use
- Provide clear error messages about network issues

### Git Not Installed
- Detect git availability at startup
- Provide helpful error message with installation instructions
- Allow project creation without git initialization

### Invalid Project Names
- Validate project names for filesystem compatibility
- Suggest corrections for common issues (spaces, special chars)
- Handle different naming conventions per language

## Dependencies

### Cargo.toml
```toml
[package]
name = "claudeforge"
version = "0.1.0"
edition = "2021"

[dependencies]
# CLI
clap = { version = "4", features = ["derive", "env"] }

# Async runtime
tokio = { version = "1", features = ["full"] }

# Error handling
anyhow = "1"
thiserror = "1"

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"

# Filesystem
dirs = "5"
tempfile = "3"

# Git operations
git2 = "0.18"

# HTTP client
reqwest = { version = "0.11", features = ["json"] }

# Utilities
chrono = "0.4"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[dev-dependencies]
# Testing
tempfile = "3"
mockall = "0.11"
```

## Configuration

### Built-in Templates Configuration
```toml
# config/templates.toml
[[templates]]
name = "rust-claude-code"
language = "rust"
repository = "https://github.com/iepathos/rust-claude-code"
description = "Comprehensive Rust starter template with Claude Code guidelines"

[[templates.files_to_customize]]
path = "Cargo.toml"
replacements = [
    { placeholder = "my-project", value_type = "ProjectName" },
]

[[templates.files_to_customize]]
path = "README.md"
replacements = [
    { placeholder = "yourusername", value_type = "AuthorName" },
    { placeholder = "my-rust-project", value_type = "ProjectName" },
]

[[templates]]
name = "go-claude-code"
language = "go"
repository = "https://github.com/iepathos/go-claude-code"
description = "Go project template optimized for Claude Code development"

[[templates.files_to_customize]]
path = "go.mod"
replacements = [
    { placeholder = "github.com/yourusername/my-project", value_type = "Custom" },
]
```

### User Configuration
```toml
# ~/.config/claudeforge/config.toml
[defaults]
author_name = "John Doe"
author_email = "john@example.com"
default_directory = "~/projects"

[templates]
cache_directory = "~/.cache/claudeforge"
auto_update = true
update_interval_days = 7
```

## Documentation

### CLI Help Text
```
claudeforge 0.1.0
Create new projects optimized for Claude Code

USAGE:
    claudeforge <SUBCOMMAND>

SUBCOMMANDS:
    new      Create a new project from a template
    list     List available templates
    update   Update cached templates
    version  Show version information
    help     Print this message or the help of the given subcommand(s)

EXAMPLES:
    # Create a new Rust project
    claudeforge new rust my-project

    # Create a Go project in a specific directory
    claudeforge new go my-service -d ~/work/projects

    # List all available templates
    claudeforge list

    # Update template cache
    claudeforge update
```

### README Documentation
The README should include:
- Installation instructions (cargo install, homebrew, etc.)
- Quick start guide
- Template descriptions and features
- Configuration options
- Contributing guidelines for new templates
- Troubleshooting common issues

### Example Usage Documentation
```bash
# Basic usage
claudeforge new rust my-awesome-project

# Specify target directory
claudeforge new go my-service --directory ~/projects

# Skip confirmation prompts
claudeforge new rust my-project --yes

# List available templates with descriptions
claudeforge list

# Update templates to latest versions
claudeforge update
```