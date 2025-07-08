use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

/// User configuration structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub defaults: Defaults,
    pub templates: TemplateConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Defaults {
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub default_directory: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub cache_directory: Option<PathBuf>,
    pub auto_update: bool,
    pub update_interval_days: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            defaults: Defaults {
                author_name: None,
                author_email: None,
                default_directory: None,
            },
            templates: TemplateConfig {
                cache_directory: None,
                auto_update: true,
                update_interval_days: 7,
            },
        }
    }
}

impl Config {
    /// Load configuration from file, creating default if it doesn't exist
    pub async fn load() -> Result<Self> {
        let config_path = get_config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path).await?;
            let config = toml::from_str(&content)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save().await?;
            Ok(config)
        }
    }

    /// Save configuration to file
    pub async fn save(&self) -> Result<()> {
        let config_path = get_config_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content).await?;

        Ok(())
    }

    /// Get the effective cache directory
    pub fn cache_directory(&self) -> Result<PathBuf> {
        if let Some(cache_dir) = &self.templates.cache_directory {
            Ok(cache_dir.clone())
        } else {
            Ok(dirs::cache_dir()
                .ok_or_else(|| anyhow::anyhow!("Failed to find cache directory"))?
                .join("claudeforge"))
        }
    }
}

/// Get the path to the configuration file
fn get_config_path() -> Result<PathBuf> {
    let config_dir =
        dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Failed to find config directory"))?;

    Ok(config_dir.join("claudeforge").join("config.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_default_config() {
        let config = Config::default();
        assert!(config.templates.auto_update);
        assert_eq!(config.templates.update_interval_days, 7);
    }

    #[tokio::test]
    async fn test_config_serialization() {
        let config = Config::default();
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();

        assert_eq!(
            config.templates.auto_update,
            deserialized.templates.auto_update
        );
        assert_eq!(
            config.templates.update_interval_days,
            deserialized.templates.update_interval_days
        );
    }
}
