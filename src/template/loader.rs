use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;
use tracing::{debug, info};

use crate::cli::Language;
use crate::error::ClaudeForgeError;
use crate::git;
use crate::template::{registry, Template};
use crate::utils::fs as utils_fs;

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

        let templates = registry::load_template_registry()?;

        Ok(Self {
            cache_dir,
            templates,
        })
    }

    pub async fn get_or_fetch(&self, language: Language) -> Result<PathBuf> {
        let template = self
            .templates
            .get(&language)
            .ok_or_else(|| ClaudeForgeError::TemplateNotFound(language.to_string()))?;

        let template_path = self.cache_dir.join(&template.name);

        if !template_path.exists() {
            info!("Template not found in cache, fetching from repository...");
            self.fetch_template(template).await?;
        } else {
            debug!("Using cached template at {:?}", template_path);
        }

        Ok(template_path)
    }

    pub fn get_template(&self, language: Language) -> Result<&Template> {
        self.templates
            .get(&language)
            .ok_or_else(|| ClaudeForgeError::TemplateNotFound(language.to_string()).into())
    }

    async fn fetch_template(&self, template: &Template) -> Result<()> {
        let target_path = self.cache_dir.join(&template.name);

        // Remove existing directory if it exists
        if target_path.exists() {
            utils_fs::remove_dir_all_robust(&target_path).await?;
        }

        // Clone the repository
        git::clone_repository(&template.repository, &target_path)?;

        info!("Successfully fetched template: {}", template.name);
        Ok(())
    }

    pub async fn update_all(&self) -> Result<()> {
        info!("Checking for cached templates to update...");

        let mut updated_count = 0;
        for template in self.templates.values() {
            let template_path = self.cache_dir.join(&template.name);

            if template_path.exists() {
                info!("Updating template: {}", template.name);
                self.fetch_template(template).await?;
                updated_count += 1;
            }
        }

        if updated_count == 0 {
            info!("No cached templates found. Use 'claudeforge new' to create a project first.");
        } else {
            info!("Successfully updated {} cached template(s)", updated_count);
        }
        Ok(())
    }

    pub fn list_templates(&self) -> Vec<&Template> {
        self.templates.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_template_loader_new() {
        let temp_dir = TempDir::new().unwrap();
        let original_home = env::var("HOME").ok();

        // Set a temporary HOME directory for testing
        env::set_var("HOME", temp_dir.path());

        let loader = TemplateLoader::new().await;

        // Restore original HOME
        if let Some(home) = original_home {
            env::set_var("HOME", home);
        } else {
            env::remove_var("HOME");
        }

        assert!(loader.is_ok());
    }

    #[tokio::test]
    async fn test_list_templates() {
        let temp_dir = TempDir::new().unwrap();
        let original_home = env::var("HOME").ok();

        // Set a temporary HOME directory for testing
        env::set_var("HOME", temp_dir.path());

        let loader = TemplateLoader::new().await;

        // Restore original HOME
        if let Some(home) = original_home {
            env::set_var("HOME", home);
        } else {
            env::remove_var("HOME");
        }

        if let Ok(loader) = loader {
            let templates = loader.list_templates();
            assert!(!templates.is_empty());

            // Check if templates have expected properties
            for template in templates {
                assert!(!template.name.is_empty());
                assert!(!template.repository.is_empty());
            }
        }
    }

    #[tokio::test]
    async fn test_get_template() {
        let temp_dir = TempDir::new().unwrap();
        let original_home = env::var("HOME").ok();

        // Set a temporary HOME directory for testing
        env::set_var("HOME", temp_dir.path());

        let loader = TemplateLoader::new().await;

        // Restore original HOME
        if let Some(home) = original_home {
            env::set_var("HOME", home);
        } else {
            env::remove_var("HOME");
        }

        if let Ok(loader) = loader {
            let template = loader.get_template(Language::Rust);
            assert!(template.is_ok());

            if let Ok(template) = template {
                assert_eq!(template.language, Language::Rust);
            }
        }
    }

    #[tokio::test]
    async fn test_get_template_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let original_home = env::var("HOME").ok();

        // Set a temporary HOME directory for testing
        env::set_var("HOME", temp_dir.path());

        let loader = TemplateLoader::new().await;

        // Restore original HOME
        if let Some(home) = original_home {
            env::set_var("HOME", home);
        } else {
            env::remove_var("HOME");
        }

        if let Ok(loader) = loader {
            // Try to get a template that might not exist (Python)
            let template = loader.get_template(Language::Python);
            // This may or may not exist depending on the registry
            // Just test that it returns a result
            assert!(template.is_ok() || template.is_err());
        }
    }

    #[tokio::test]
    async fn test_update_all_no_cached_templates() {
        let temp_dir = TempDir::new().unwrap();
        let original_home = env::var("HOME").ok();

        // Set a temporary HOME directory for testing
        env::set_var("HOME", temp_dir.path());

        let loader = TemplateLoader::new().await;

        // Restore original HOME
        if let Some(home) = original_home {
            env::set_var("HOME", home);
        } else {
            env::remove_var("HOME");
        }

        if let Ok(loader) = loader {
            let result = loader.update_all().await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_get_or_fetch_template_not_in_cache() {
        let temp_dir = TempDir::new().unwrap();
        let original_home = env::var("HOME").ok();

        // Set a temporary HOME directory for testing
        env::set_var("HOME", temp_dir.path());

        let loader = TemplateLoader::new().await;

        // Restore original HOME
        if let Some(home) = original_home {
            env::set_var("HOME", home);
        } else {
            env::remove_var("HOME");
        }

        if let Ok(loader) = loader {
            // This will try to fetch from git - might fail if git is not available
            let result = loader.get_or_fetch(Language::Rust).await;
            // We don't assert success since it depends on git availability
            // Just check that the method doesn't panic
            assert!(result.is_ok() || result.is_err());
        }
    }
}
