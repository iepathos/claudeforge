use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;
use tracing::{debug, info};

use crate::cli::Language;
use crate::error::ClaudeForgeError;
use crate::git;
use crate::template::{registry, Template};

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
            fs::remove_dir_all(&target_path).await?;
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
