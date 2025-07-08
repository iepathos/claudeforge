use crate::cli::Language;
use crate::template::{FileCustomization, Replacement, Template, ValueType};
use anyhow::Result;
use std::collections::HashMap;

/// Load the built-in template registry
pub fn load_template_registry() -> Result<HashMap<Language, Template>> {
    let mut templates = HashMap::new();

    // Rust template
    templates.insert(
        Language::Rust,
        Template {
            name: "rust-claude-code".to_string(),
            language: Language::Rust,
            repository: "https://github.com/iepathos/rust-claude-code".to_string(),
            description: "Comprehensive Rust starter template with Claude Code guidelines"
                .to_string(),
            files_to_customize: vec![
                FileCustomization {
                    path: "Cargo.toml".to_string(),
                    replacements: vec![Replacement {
                        placeholder: "my-project".to_string(),
                        value_type: ValueType::ProjectName,
                    }],
                },
                FileCustomization {
                    path: "README.md".to_string(),
                    replacements: vec![
                        Replacement {
                            placeholder: "yourusername".to_string(),
                            value_type: ValueType::AuthorName,
                        },
                        Replacement {
                            placeholder: "my-rust-project".to_string(),
                            value_type: ValueType::ProjectName,
                        },
                    ],
                },
            ],
        },
    );

    // Go template
    templates.insert(
        Language::Go,
        Template {
            name: "go-claude-code".to_string(),
            language: Language::Go,
            repository: "https://github.com/iepathos/go-claude-code".to_string(),
            description: "Go project template optimized for Claude Code development".to_string(),
            files_to_customize: vec![
                FileCustomization {
                    path: "go.mod".to_string(),
                    replacements: vec![Replacement {
                        placeholder: "github.com/yourusername/my-project".to_string(),
                        value_type: ValueType::Custom("github.com/user/project".to_string()),
                    }],
                },
                FileCustomization {
                    path: "README.md".to_string(),
                    replacements: vec![
                        Replacement {
                            placeholder: "yourusername".to_string(),
                            value_type: ValueType::AuthorName,
                        },
                        Replacement {
                            placeholder: "my-go-project".to_string(),
                            value_type: ValueType::ProjectName,
                        },
                    ],
                },
            ],
        },
    );

    Ok(templates)
}

/// Load templates from a configuration file (future enhancement)
pub async fn load_templates_from_config(_config_path: &str) -> Result<HashMap<Language, Template>> {
    // TODO: Implement loading from external config file
    load_template_registry()
}
