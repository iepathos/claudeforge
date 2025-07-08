pub mod loader;
pub mod processor;
pub mod registry;

use crate::cli::Language;
use serde::{Deserialize, Serialize};

/// Template configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    pub name: String,
    pub language: Language,
    pub repository: String,
    pub description: String,
    pub files_to_customize: Vec<FileCustomization>,
}

/// File customization rules
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileCustomization {
    pub path: String,
    pub replacements: Vec<Replacement>,
}

/// Text replacement rule
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Replacement {
    pub placeholder: String,
    pub value_type: ValueType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ValueType {
    ProjectName,
    ProjectPath,
    AuthorName,
    AuthorEmail,
    CurrentDate,
    Custom(String),
}
