use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, info};

use crate::cli::Language;
use crate::error::ClaudeForgeError;
use crate::git;
use crate::template::{loader::TemplateLoader, Template, ValueType};
use crate::utils::fs as fs_utils;

pub async fn create_project(
    language: Language,
    name: String,
    directory: Option<PathBuf>,
    skip_prompts: bool,
) -> Result<()> {
    info!("Creating new {} project: {}", language, name);

    let loader = TemplateLoader::new().await?;
    let template_path = loader.get_or_fetch(language.clone()).await?;

    let target_dir = directory.unwrap_or_else(|| PathBuf::from(".")).join(&name);

    // Check if directory exists
    if target_dir.exists() {
        if skip_prompts {
            info!("Directory exists, overwriting due to --yes flag");
        } else {
            return Err(ClaudeForgeError::DirectoryExists(target_dir).into());
        }
    }

    // Copy template files
    info!("Copying template files...");
    copy_template(&template_path, &target_dir).await?;

    // Customize files
    info!("Customizing project files...");
    let template = loader.get_template(language)?;
    customize_project_files(&target_dir, &name, template).await?;

    // Initialize git repository
    info!("Initializing git repository...");
    initialize_git_repo(&target_dir).await?;

    println!("✅ Project '{name}' created successfully!");
    println!("📁 Location: {}", target_dir.display());
    println!("🚀 Get started with: cd {name} && claude code .");

    Ok(())
}

async fn copy_template(template_path: &Path, target_dir: &Path) -> Result<()> {
    // Create target directory
    fs::create_dir_all(target_dir)
        .await
        .with_context(|| format!("Failed to create directory: {target_dir:?}"))?;

    // Copy all files except .git directory
    fs_utils::copy_dir_recursive(template_path, target_dir, Some(&[".git"])).await?;

    Ok(())
}

async fn customize_project_files(
    project_dir: &Path,
    project_name: &str,
    template: &Template,
) -> Result<()> {
    let replacements = build_replacements(project_name).await?;

    for customization in &template.files_to_customize {
        let file_path = project_dir.join(&customization.path);

        if file_path.exists() {
            debug!("Customizing file: {:?}", file_path);

            let content = fs::read_to_string(&file_path)
                .await
                .with_context(|| format!("Failed to read file: {file_path:?}"))?;

            let new_content =
                apply_replacements(&content, &replacements, &customization.replacements);

            fs::write(&file_path, new_content)
                .await
                .with_context(|| format!("Failed to write file: {file_path:?}"))?;
        } else {
            debug!("File not found for customization: {:?}", file_path);
        }
    }

    Ok(())
}

async fn build_replacements(project_name: &str) -> Result<HashMap<String, String>> {
    let mut replacements = HashMap::new();

    replacements.insert("{{PROJECT_NAME}}".to_string(), project_name.to_string());
    replacements.insert(
        "{{CURRENT_DATE}}".to_string(),
        chrono::Local::now().format("%Y-%m-%d").to_string(),
    );

    // Get git config for author info
    if let Ok(output) = tokio::process::Command::new("git")
        .args(["config", "user.name"])
        .output()
        .await
    {
        if output.status.success() {
            let author = String::from_utf8_lossy(&output.stdout).trim().to_string();
            replacements.insert("{{AUTHOR_NAME}}".to_string(), author);
        }
    }

    if let Ok(output) = tokio::process::Command::new("git")
        .args(["config", "user.email"])
        .output()
        .await
    {
        if output.status.success() {
            let email = String::from_utf8_lossy(&output.stdout).trim().to_string();
            replacements.insert("{{AUTHOR_EMAIL}}".to_string(), email);
        }
    }

    Ok(replacements)
}

fn apply_replacements(
    content: &str,
    global_replacements: &HashMap<String, String>,
    template_replacements: &[crate::template::Replacement],
) -> String {
    let mut result = content.to_string();

    // Apply template-specific replacements
    for replacement in template_replacements {
        let value = match &replacement.value_type {
            ValueType::ProjectName => global_replacements.get("{{PROJECT_NAME}}"),
            ValueType::AuthorName => global_replacements.get("{{AUTHOR_NAME}}"),
            ValueType::AuthorEmail => global_replacements.get("{{AUTHOR_EMAIL}}"),
            ValueType::CurrentDate => global_replacements.get("{{CURRENT_DATE}}"),
            ValueType::ProjectPath => None, // TODO: Implement project path replacement
            ValueType::Custom(custom_value) => Some(custom_value),
        };

        if let Some(value) = value {
            result = result.replace(&replacement.placeholder, value);
        }
    }

    // Apply global replacements
    for (placeholder, value) in global_replacements {
        result = result.replace(placeholder, value);
    }

    result
}

async fn initialize_git_repo(project_dir: &Path) -> Result<()> {
    // Remove existing .git directory if it exists
    let git_dir = project_dir.join(".git");
    if git_dir.exists() {
        fs::remove_dir_all(&git_dir).await?;
    }

    // Initialize new git repository
    git::init_repository(project_dir)?;

    // Add all files to initial commit
    git::add_all_and_commit(project_dir, "Initial commit from ClaudeForge")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_build_replacements() {
        let replacements = build_replacements("my-project").await.unwrap();
        assert_eq!(replacements.get("{{PROJECT_NAME}}").unwrap(), "my-project");
        assert!(replacements.contains_key("{{CURRENT_DATE}}"));
    }

    #[test]
    fn test_apply_replacements() {
        let mut global_replacements = HashMap::new();
        global_replacements.insert("{{PROJECT_NAME}}".to_string(), "test-project".to_string());

        let template_replacements = vec![crate::template::Replacement {
            placeholder: "my-project".to_string(),
            value_type: ValueType::ProjectName,
        }];

        let content = "This is my-project template";
        let result = apply_replacements(content, &global_replacements, &template_replacements);

        assert_eq!(result, "This is test-project template");
    }
}
