# Spec 030: Custom Template Support

## Feature Summary

This specification defines the implementation of custom template support for ClaudeForge, allowing users to define, manage, and use their own project templates without modifying the source code. The feature extends the existing template registry system to support user-defined templates through configuration files and CLI commands.

The current system hard-codes templates in the registry, limiting users to only Rust and Go templates. This enhancement will enable users to add templates for any language or framework, store them in their configuration, and manage them through intuitive CLI commands.

## Goals & Requirements

### Functional Requirements
- **FR1**: Users can add custom templates through CLI commands
- **FR2**: Custom templates are stored in user configuration files
- **FR3**: Custom templates can be used alongside built-in templates
- **FR4**: Users can list, add, and remove custom templates
- **FR5**: Custom templates support the same file customization features as built-in templates
- **FR6**: Template validation ensures repositories are accessible and properly structured
- **FR7**: Configuration-based template loading merges custom and built-in templates

### Non-Functional Requirements
- **NFR1**: Template operations should complete within 5 seconds under normal network conditions
- **NFR2**: Configuration file must remain backwards compatible
- **NFR3**: Template caching system should work identically for custom templates
- **NFR4**: Error messages should be clear and actionable for template management
- **NFR5**: Custom templates should not impact built-in template performance

### Success Criteria
- Users can add custom templates without modifying source code
- Custom templates integrate seamlessly with existing workflows
- Configuration system supports multiple custom templates
- Template validation prevents invalid configurations
- All existing functionality continues to work unchanged

## API/Interface Design

### Configuration Structure

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub cache_directory: Option<PathBuf>,
    pub auto_update: bool,
    pub update_interval_days: u32,
    pub custom_templates: Vec<CustomTemplate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomTemplate {
    pub name: String,
    pub language: String,
    pub repository: String,
    pub description: String,
    pub files_to_customize: Vec<FileCustomization>,
}
```

### Enhanced Language Support

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "custom")]
    Custom(String),
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Rust => write!(f, "rust"),
            Language::Go => write!(f, "go"),
            Language::Custom(lang) => write!(f, "{}", lang),
        }
    }
}

impl std::str::FromStr for Language {
    type Err = std::convert::Infallible;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rust" => Ok(Language::Rust),
            "go" => Ok(Language::Go),
            lang => Ok(Language::Custom(lang.to_string())),
        }
    }
}
```

### CLI Commands

```rust
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new project from a template
    New {
        /// The language/template to use
        language: String,
        /// The name of the project
        name: String,
        /// The directory to create the project in
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// List all available templates
    List,
    /// Update all cached templates
    Update,
    /// Add a custom template
    Add {
        /// Template name
        name: String,
        /// Language identifier
        language: String,
        /// Git repository URL
        repository: String,
        /// Template description
        #[arg(short, long)]
        description: Option<String>,
        /// Files to customize (format: path:placeholder:value_type)
        #[arg(short, long)]
        files: Vec<String>,
    },
    /// Remove a custom template
    Remove {
        /// Template language identifier
        language: String,
    },
    /// Show template details
    Show {
        /// Template language identifier
        language: String,
    },
}
```

### Template Management Functions

```rust
impl Config {
    /// Add a custom template to the configuration
    pub async fn add_custom_template(&mut self, template: CustomTemplate) -> Result<()>;
    
    /// Remove a custom template by language
    pub async fn remove_custom_template(&mut self, language: &str) -> Result<bool>;
    
    /// Get a custom template by language
    pub fn get_custom_template(&self, language: &str) -> Option<&CustomTemplate>;
    
    /// List all custom templates
    pub fn list_custom_templates(&self) -> &[CustomTemplate];
    
    /// Validate a custom template configuration
    pub async fn validate_custom_template(&self, template: &CustomTemplate) -> Result<()>;
}
```

## File and Package Structure

### Modified Files

```
src/
├── config/
│   └── mod.rs              # Extended with custom template support
├── template/
│   ├── mod.rs              # Enhanced Template and FileCustomization structs
│   ├── registry.rs         # Implement load_templates_from_config()
│   ├── loader.rs           # Support custom template loading
│   └── processor.rs        # Handle custom language processing
├── cli.rs                  # Extended with template management commands
└── main.rs                 # Updated command handling
```

### New Files

```
src/
├── template/
│   └── validator.rs        # Template validation functionality
└── error.rs                # Enhanced error types for template management
```

## Implementation Details

### Phase 1: Configuration Enhancement

1. **Extend TemplateConfig Structure**
   - Add `custom_templates` field to `TemplateConfig`
   - Implement serialization/deserialization for `CustomTemplate`
   - Add validation methods for template configuration

2. **Update Config Loading**
   - Modify `Config::load()` to handle custom template fields
   - Ensure backward compatibility with existing config files
   - Add default empty vector for custom templates

### Phase 2: Template Registry Enhancement

1. **Implement load_templates_from_config()**
   ```rust
   pub async fn load_templates_from_config(config: &Config) -> Result<HashMap<String, Template>> {
       let mut templates = HashMap::new();
       
       // Load built-in templates
       for (lang, template) in load_builtin_templates()? {
           templates.insert(lang.to_string(), template);
       }
       
       // Add custom templates
       for custom in &config.templates.custom_templates {
           let template = Template {
               name: custom.name.clone(),
               language: Language::Custom(custom.language.clone()),
               repository: custom.repository.clone(),
               description: custom.description.clone(),
               files_to_customize: custom.files_to_customize.clone(),
           };
           templates.insert(custom.language.clone(), template);
       }
       
       Ok(templates)
   }
   ```

2. **Update Template Registry Loading**
   - Modify `TemplateRegistry::new()` to use config-based loading
   - Implement template merging logic
   - Add conflict resolution for duplicate language identifiers

### Phase 3: CLI Command Implementation

1. **Add Template Management Commands**
   ```rust
   async fn handle_add_template(
       name: String,
       language: String,
       repository: String,
       description: Option<String>,
       files: Vec<String>,
   ) -> Result<()> {
       let mut config = Config::load().await?;
       
       // Parse file customization specifications
       let files_to_customize = parse_file_customizations(files)?;
       
       let template = CustomTemplate {
           name,
           language: language.clone(),
           repository,
           description: description.unwrap_or_else(|| format!("Custom {} template", language)),
           files_to_customize,
       };
       
       // Validate template before adding
       config.validate_custom_template(&template).await?;
       
       // Add to configuration
       config.add_custom_template(template).await?;
       
       println!("✅ Custom template '{}' added successfully", language);
       Ok(())
   }
   ```

2. **Enhanced New Command**
   - Support custom language identifiers
   - Maintain backward compatibility with existing usage
   - Add proper error handling for unknown templates

### Phase 4: Template Validation

1. **Repository Validation**
   ```rust
   pub async fn validate_repository_access(repo_url: &str) -> Result<()> {
       // Test git clone access without actually cloning
       let output = Command::new("git")
           .args(["ls-remote", repo_url])
           .output()
           .await?;
       
       if !output.status.success() {
           return Err(ClaudeForgeError::TemplateValidation(
               format!("Cannot access repository: {}", repo_url)
           ));
       }
       
       Ok(())
   }
   ```

2. **Template Structure Validation**
   - Verify file customization paths exist in repository
   - Validate placeholder syntax
   - Check for required template metadata

## Testing Strategy

### Unit Tests

1. **Configuration Tests** (`src/config/mod.rs`)
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_add_custom_template() {
           let mut config = Config::default();
           let template = CustomTemplate {
               name: "test-template".to_string(),
               language: "python".to_string(),
               repository: "https://github.com/user/python-template".to_string(),
               description: "Test template".to_string(),
               files_to_customize: vec![],
           };
           
           config.add_custom_template(template).await.unwrap();
           assert_eq!(config.templates.custom_templates.len(), 1);
       }
       
       #[test]
       fn test_remove_custom_template() {
           // Test template removal
       }
       
       #[test]
       fn test_duplicate_language_handling() {
           // Test conflict resolution
       }
   }
   ```

2. **Template Registry Tests** (`src/template/registry.rs`)
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[tokio::test]
       async fn test_load_templates_from_config() {
           let config = create_test_config_with_custom_templates();
           let templates = load_templates_from_config(&config).await.unwrap();
           
           assert!(templates.contains_key("rust"));
           assert!(templates.contains_key("go"));
           assert!(templates.contains_key("python"));
       }
   }
   ```

3. **CLI Tests** (`src/cli.rs`)
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[tokio::test]
       async fn test_add_template_command() {
           // Test template addition through CLI
       }
       
       #[tokio::test]
       async fn test_list_templates_with_custom() {
           // Test template listing includes custom templates
       }
   }
   ```

### Integration Tests

1. **Template Lifecycle Tests** (`tests/template_management.rs`)
   ```rust
   #[tokio::test]
   async fn test_full_template_lifecycle() {
       // Test: add template -> list -> use -> remove
   }
   
   #[tokio::test]
   async fn test_custom_template_project_creation() {
       // Test creating project from custom template
   }
   ```

2. **Configuration Persistence Tests** (`tests/config_persistence.rs`)
   ```rust
   #[tokio::test]
   async fn test_config_persistence_across_restarts() {
       // Test custom templates persist across app restarts
   }
   ```

### Mock Requirements

1. **Git Repository Mocking**
   - Mock successful/failed repository access
   - Mock template repository structure
   - Mock git clone operations

2. **File System Mocking**
   - Mock configuration file operations
   - Mock template cache directory operations
   - Mock project creation file operations

## Edge Cases & Error Handling

### Template Addition Errors
- **Duplicate Language**: Prompt user to overwrite or choose different identifier
- **Invalid Repository**: Provide clear error message with repository URL
- **Network Issues**: Graceful handling with retry suggestions
- **Malformed File Customizations**: Validate syntax and provide examples

### Template Usage Errors
- **Missing Template**: Clear error message with available templates list
- **Repository Access Issues**: Distinguish between network and authentication errors
- **Template Corruption**: Detect and handle corrupted cached templates
- **File Customization Failures**: Handle missing placeholders or invalid paths

### Configuration Errors
- **Invalid Config Format**: Provide migration guidance for config updates
- **Permission Issues**: Clear guidance on file permission requirements
- **Disk Space**: Handle insufficient disk space for template caching

### Error Types

```rust
#[derive(Error, Debug)]
pub enum ClaudeForgeError {
    // Existing errors...
    
    #[error("Template validation failed: {0}")]
    TemplateValidation(String),
    
    #[error("Duplicate template language: {0}")]
    DuplicateTemplate(String),
    
    #[error("Invalid file customization format: {0}")]
    InvalidFileCustomization(String),
    
    #[error("Template not found: {0}")]
    TemplateNotFound(String),
    
    #[error("Repository access failed: {0}")]
    RepositoryAccess(String),
}
```

## Dependencies

### New Dependencies
- No new external dependencies required
- Leverages existing dependencies: `serde`, `tokio`, `git2`, `clap`

### Updated Dependencies
- `clap`: Extended for new subcommands
- `serde`: Enhanced serialization for new configuration structure
- `tokio`: Async support for template validation

## Configuration

### Configuration File Format

```toml
[defaults]
author_name = "John Doe"
author_email = "john@example.com"

[templates]
cache_directory = "~/.cache/claudeforge"
auto_update = true
update_interval_days = 7

[[templates.custom_templates]]
name = "my-python-template"
language = "python"
repository = "https://github.com/user/python-template"
description = "My Python template with Flask setup"

[[templates.custom_templates.files_to_customize]]
path = "pyproject.toml"
[[templates.custom_templates.files_to_customize.replacements]]
placeholder = "my-project"
value_type = "ProjectName"

[[templates.custom_templates.files_to_customize]]
path = "src/main.py"
[[templates.custom_templates.files_to_customize.replacements]]
placeholder = "Author Name"
value_type = "AuthorName"

[[templates.custom_templates]]
name = "react-typescript"
language = "typescript"
repository = "https://github.com/user/react-typescript-template"
description = "React TypeScript template with Tailwind CSS"
```

### Environment Variables
- `CLAUDEFORGE_CONFIG_DIR`: Override default configuration directory
- `CLAUDEFORGE_CACHE_DIR`: Override default cache directory
- `CLAUDEFORGE_TEMPLATE_TIMEOUT`: Template operation timeout in seconds

## Documentation

### GoDoc Requirements

```rust
/// Add a custom template to the user's configuration.
/// 
/// This function validates the template configuration before adding it to
/// prevent invalid templates from being stored. The template repository
/// must be accessible and the file customization paths must be valid.
/// 
/// # Arguments
/// 
/// * `template` - The custom template configuration to add
/// 
/// # Returns
/// 
/// Returns `Ok(())` if the template was successfully added, or an error
/// if the template is invalid or cannot be added.
/// 
/// # Examples
/// 
/// ```rust
/// use claudeforge::config::{Config, CustomTemplate};
/// 
/// let mut config = Config::load().await?;
/// let template = CustomTemplate {
///     name: "my-template".to_string(),
///     language: "python".to_string(),
///     repository: "https://github.com/user/python-template".to_string(),
///     description: "My Python template".to_string(),
///     files_to_customize: vec![],
/// };
/// 
/// config.add_custom_template(template).await?;
/// ```
pub async fn add_custom_template(&mut self, template: CustomTemplate) -> Result<()>
```

### CLI Help Documentation

```
claudeforge add --help
Add a custom template

Usage: claudeforge add [OPTIONS] <NAME> <LANGUAGE> <REPOSITORY>

Arguments:
  <NAME>         Template name
  <LANGUAGE>     Language identifier (e.g., python, typescript, java)
  <REPOSITORY>   Git repository URL

Options:
  -d, --description <DESCRIPTION>  Template description
  -f, --files <FILES>              Files to customize (format: path:placeholder:value_type)
  -h, --help                       Print help
```

### README Updates

Add section to README.md:

```markdown
## Custom Templates

ClaudeForge supports custom templates in addition to built-in templates. You can add your own templates for any language or framework.

### Adding Custom Templates

```bash
# Add a Python template
claudeforge add my-python python https://github.com/user/python-template \
  --description "My Python template with Flask" \
  --files "pyproject.toml:my-project:ProjectName"

# Add a TypeScript template
claudeforge add react-ts typescript https://github.com/user/react-typescript-template \
  --description "React TypeScript template"
```

### Managing Custom Templates

```bash
# List all templates (built-in and custom)
claudeforge list

# Show template details
claudeforge show python

# Remove a custom template
claudeforge remove python

# Update all templates
claudeforge update
```

### Template Structure

Custom templates should follow the same structure as built-in templates:
- Git repository with project files
- Placeholder strings for customization
- Optional `.claudeforge.toml` metadata file
```

## Example Usage

### CLI Usage Examples

```bash
# Add a Python template
claudeforge add flask-api python https://github.com/user/flask-api-template \
  --description "Flask API template with authentication" \
  --files "requirements.txt:flask-api:ProjectName" \
  --files "app/__init__.py:Author:AuthorName"

# Add a Next.js template
claudeforge add nextjs-app typescript https://github.com/user/nextjs-template \
  --description "Next.js template with TypeScript and Tailwind"

# Use custom templates
claudeforge new python my-api
claudeforge new typescript my-app

# List all templates
claudeforge list
# Output:
# Available templates:
# ✅ rust - Comprehensive Rust starter template (built-in)
# ✅ go - Go project template (built-in)
# ✅ python - Flask API template with authentication (custom)
# ✅ typescript - Next.js template with TypeScript and Tailwind (custom)

# Show template details
claudeforge show python
# Output:
# Template: flask-api
# Language: python
# Repository: https://github.com/user/flask-api-template
# Description: Flask API template with authentication
# Files to customize:
#   - requirements.txt: flask-api -> ProjectName
#   - app/__init__.py: Author -> AuthorName

# Remove custom template
claudeforge remove python
# Output: ✅ Custom template 'python' removed successfully
```

### Configuration File Example

```toml
[defaults]
author_name = "Jane Developer"
author_email = "jane@example.com"

[templates]
cache_directory = "/home/jane/.cache/claudeforge"
auto_update = true
update_interval_days = 7

[[templates.custom_templates]]
name = "flask-api"
language = "python"
repository = "https://github.com/user/flask-api-template"
description = "Flask API template with authentication"

[[templates.custom_templates.files_to_customize]]
path = "requirements.txt"
[[templates.custom_templates.files_to_customize.replacements]]
placeholder = "flask-api"
value_type = "ProjectName"

[[templates.custom_templates.files_to_customize]]
path = "app/__init__.py"
[[templates.custom_templates.files_to_customize.replacements]]
placeholder = "Author"
value_type = "AuthorName"

[[templates.custom_templates]]
name = "nextjs-app"
language = "typescript"
repository = "https://github.com/user/nextjs-template"
description = "Next.js template with TypeScript and Tailwind"

[[templates.custom_templates.files_to_customize]]
path = "package.json"
[[templates.custom_templates.files_to_customize.replacements]]
placeholder = "nextjs-app"
value_type = "ProjectName"
```