# Spec 060: Template Variable Support

## Feature Summary

This specification defines advanced template variable support for ClaudeForge templates, enabling sophisticated placeholder replacement with conditional logic, loops, filters, and custom variables. This enhancement extends the current simple string replacement system to support complex templating scenarios commonly needed when creating projects from templates.

The current system only supports basic placeholder replacement like `{{PROJECT_NAME}}` with simple string substitution. This enhancement introduces a full templating engine with support for user-defined variables, conditional rendering, iteration, and advanced text transformations, making templates more flexible and powerful.

## Goals & Requirements

### Functional Requirements
- **FR1**: Support advanced template syntax with conditional logic (`{{#if}}`, `{{#unless}}`)
- **FR2**: Enable iteration over collections (`{{#each}}`, `{{#for}}`)
- **FR3**: Provide built-in filters for text transformation (`{{name | uppercase}}`, `{{date | format}}`)
- **FR4**: Allow user-defined custom variables in addition to built-in ones
- **FR5**: Support nested template variables and expressions
- **FR6**: Enable template includes and partials for reusable snippets
- **FR7**: Provide escaped and raw output modes for different file types
- **FR8**: Support template inheritance with blocks and extends

### Non-Functional Requirements
- **NFR1**: Template processing should complete within 5 seconds for typical projects
- **NFR2**: Template syntax should be intuitive and similar to popular engines (Handlebars/Jinja2)
- **NFR3**: Variable resolution should be type-safe with clear error messages
- **NFR4**: Template cache should include processed variable metadata
- **NFR5**: Memory usage should remain reasonable for large templates (< 100MB)

### Success Criteria
- Templates can dynamically generate content based on user input and conditions
- Complex project structures can be created with minimal template duplication
- Template authors can create reusable, configurable templates
- Error messages clearly indicate template syntax issues and missing variables
- Existing simple placeholder templates continue to work unchanged

## API/Interface Design

### Template Syntax Extensions

```handlebars
{{!-- Basic variable replacement (existing) --}}
{{PROJECT_NAME}}
{{AUTHOR_NAME}}
{{AUTHOR_EMAIL}}

{{!-- Custom variables --}}
{{my_variable}}
{{config.database_type}}

{{!-- Conditional blocks --}}
{{#if use_database}}
use database::Connection;
{{/if}}

{{#unless is_library}}
fn main() {
    // Application entry point
}
{{/unless}}

{{!-- Iteration --}}
{{#each dependencies}}
{{this.name}} = "{{this.version}}"
{{/each}}

{{#for i in 0..test_count}}
#[test]
fn test_{{i}}() {
    // Generated test
}
{{/for}}

{{!-- Filters --}}
{{PROJECT_NAME | snake_case}}
{{AUTHOR_NAME | title_case}}
{{CURRENT_DATE | format "Y-m-d"}}

{{!-- Nested expressions --}}
{{#if (eq language "rust")}}
[dependencies]
{{#each rust_dependencies}}
{{@key}} = "{{this}}"
{{/each}}
{{/if}}

{{!-- Template includes --}}
{{> license_header}}
{{> cargo_dependencies config=dependencies}}

{{!-- Raw/escaped output --}}
{{description}}           <!-- HTML escaped -->
{{{raw_content}}}        <!-- Raw output -->
```

### Variable Definition System

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariables {
    pub built_in: HashMap<String, VariableValue>,
    pub custom: HashMap<String, VariableValue>,
    pub computed: HashMap<String, ComputedVariable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<VariableValue>),
    Object(HashMap<String, VariableValue>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputedVariable {
    pub expression: String,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    pub variables: HashMap<String, VariableDefinition>,
    pub prompts: Vec<VariablePrompt>,
    pub includes: Vec<TemplateInclude>,
    pub filters: HashMap<String, FilterConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDefinition {
    pub name: String,
    pub var_type: VariableType,
    pub default: Option<VariableValue>,
    pub description: String,
    pub required: bool,
    pub validation: Option<VariableValidation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableType {
    String,
    Number,
    Boolean,
    Array(Box<VariableType>),
    Object(HashMap<String, VariableType>),
    Choice(Vec<String>),
}
```

### CLI Integration

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
        /// Set template variables (key=value format)
        #[arg(short, long)]
        var: Vec<String>,
        /// Variable definition file (YAML/TOML)
        #[arg(long)]
        vars_file: Option<PathBuf>,
        /// Skip interactive prompts, use defaults
        #[arg(long)]
        no_prompt: bool,
    },
}
```

### Template Engine Interface

```rust
pub struct TemplateEngine {
    variables: TemplateVariables,
    filters: FilterRegistry,
    partials: PartialRegistry,
    config: TemplateEngineConfig,
}

impl TemplateEngine {
    pub fn new() -> Self;
    pub fn with_variables(variables: TemplateVariables) -> Self;
    pub fn register_filter<F>(&mut self, name: &str, filter: F) 
    where 
        F: Filter + Send + Sync + 'static;
    pub fn register_partial(&mut self, name: &str, template: &str) -> Result<()>;
    pub fn render(&self, template: &str, context: &Context) -> Result<String>;
    pub fn render_file(&self, path: &Path, context: &Context) -> Result<String>;
    pub fn validate_template(&self, template: &str) -> Result<Vec<ValidationIssue>>;
}

pub trait Filter: Send + Sync {
    fn apply(&self, input: &VariableValue, args: &[VariableValue]) -> Result<VariableValue>;
}

#[derive(Debug, Clone)]
pub struct Context {
    pub variables: HashMap<String, VariableValue>,
    pub globals: HashMap<String, VariableValue>,
}
```

## File and Package Structure

### New Files

```
src/
├── template/
│   ├── engine/                 # Template engine implementation
│   │   ├── mod.rs             # Engine core
│   │   ├── parser.rs          # Template syntax parser
│   │   ├── renderer.rs        # Template renderer
│   │   ├── context.rs         # Variable context management
│   │   ├── filters/           # Built-in filters
│   │   │   ├── mod.rs
│   │   │   ├── string.rs      # String transformation filters
│   │   │   ├── date.rs        # Date formatting filters
│   │   │   ├── case.rs        # Case conversion filters
│   │   │   └── collection.rs  # Array/object filters
│   │   ├── partials.rs        # Template includes/partials
│   │   └── validation.rs      # Template validation
│   ├── variables/             # Variable management
│   │   ├── mod.rs
│   │   ├── builtin.rs         # Built-in variables
│   │   ├── prompt.rs          # Interactive variable prompts
│   │   ├── resolver.rs        # Variable resolution
│   │   └── validator.rs       # Variable validation
│   └── config/                # Template configuration
│       ├── mod.rs
│       ├── parser.rs          # Config file parsing
│       └── schema.rs          # Config schema validation
```

### Modified Files

```
src/
├── template/
│   ├── mod.rs                 # Extended with template engine support
│   ├── processor.rs           # Updated to use template engine
│   └── loader.rs              # Enhanced to load template configs
├── cli.rs                     # Extended with variable flags
└── main.rs                    # Updated command handling
```

## Implementation Details

### Phase 1: Template Engine Foundation

1. **Template Parser Implementation**
   ```rust
   pub struct TemplateParser {
       tokens: Vec<Token>,
       position: usize,
   }
   
   #[derive(Debug, Clone)]
   pub enum Token {
       Text(String),
       Variable(String),
       BlockStart(String, Vec<String>),
       BlockEnd(String),
       Partial(String, HashMap<String, String>),
       Filter(String, Vec<String>),
   }
   
   impl TemplateParser {
       pub fn parse(template: &str) -> Result<Vec<Node>, ParseError> {
           // Tokenize template into structured nodes
           // Handle nested blocks and expressions
           // Validate syntax and structure
       }
   }
   ```

2. **Basic Variable Resolution**
   ```rust
   pub struct VariableResolver {
       context: Context,
       built_in_providers: Vec<Box<dyn VariableProvider>>,
   }
   
   pub trait VariableProvider {
       fn provide(&self, name: &str, context: &Context) -> Option<VariableValue>;
   }
   
   // Built-in providers
   struct GitConfigProvider;
   struct SystemInfoProvider;
   struct DateTimeProvider;
   ```

3. **Simple Conditional Logic**
   ```rust
   pub enum ConditionalExpression {
       If(Box<Expression>),
       Unless(Box<Expression>),
       Each(String),
       With(String),
   }
   
   pub enum Expression {
       Variable(String),
       Literal(VariableValue),
       Comparison(Box<Expression>, ComparisonOp, Box<Expression>),
       LogicalOp(Box<Expression>, LogicalOp, Box<Expression>),
   }
   ```

### Phase 2: Advanced Features

1. **Filter System Implementation**
   ```rust
   pub struct FilterRegistry {
       filters: HashMap<String, Box<dyn Filter>>,
   }
   
   // Built-in filters
   pub struct SnakeCaseFilter;
   impl Filter for SnakeCaseFilter {
       fn apply(&self, input: &VariableValue, _args: &[VariableValue]) -> Result<VariableValue> {
           match input {
               VariableValue::String(s) => {
                   Ok(VariableValue::String(to_snake_case(s)))
               }
               _ => Err(FilterError::InvalidInput("Expected string".to_string()))
           }
       }
   }
   
   pub struct DateFormatFilter;
   impl Filter for DateFormatFilter {
       fn apply(&self, input: &VariableValue, args: &[VariableValue]) -> Result<VariableValue> {
           // Format date according to format string
       }
   }
   ```

2. **Template Includes/Partials**
   ```rust
   pub struct PartialRegistry {
       partials: HashMap<String, CompiledTemplate>,
       search_paths: Vec<PathBuf>,
   }
   
   impl PartialRegistry {
       pub fn register(&mut self, name: &str, template: &str) -> Result<()>;
       pub fn resolve(&self, name: &str, context: &Context) -> Result<String>;
       pub fn load_from_directory(&mut self, path: &Path) -> Result<()>;
   }
   ```

3. **Complex Expression Evaluation**
   ```rust
   pub struct ExpressionEvaluator {
       context: Context,
       functions: FunctionRegistry,
   }
   
   impl ExpressionEvaluator {
       pub fn evaluate(&self, expr: &Expression) -> Result<VariableValue> {
           match expr {
               Expression::Variable(name) => self.resolve_variable(name),
               Expression::Comparison(left, op, right) => {
                   let left_val = self.evaluate(left)?;
                   let right_val = self.evaluate(right)?;
                   self.apply_comparison(left_val, op, right_val)
               }
               // Handle other expression types
           }
       }
   }
   ```

### Phase 3: Configuration and CLI Integration

1. **Template Configuration Loading**
   ```rust
   // .claudeforge.toml in template repository
   [template]
   name = "Advanced Rust Template"
   description = "Rust template with configurable features"
   
   [variables]
   use_database = { type = "boolean", default = false, description = "Include database support" }
   database_type = { type = "choice", choices = ["postgres", "mysql", "sqlite"], default = "postgres" }
   features = { type = "array", default = ["cli", "logging"], description = "Enabled features" }
   
   [[prompts]]
   variable = "use_database"
   message = "Do you want database support?"
   
   [[prompts]]
   variable = "database_type"
   message = "Which database?"
   condition = "use_database"
   
   [filters.project_name]
   snake_case = true
   kebab_case = true
   ```

2. **Interactive Variable Prompts**
   ```rust
   pub struct VariablePrompter {
       stdin: Stdin,
       stdout: Stdout,
   }
   
   impl VariablePrompter {
       pub async fn prompt_for_variables(
           &self, 
           definitions: &[VariableDefinition],
           existing: &HashMap<String, VariableValue>
       ) -> Result<HashMap<String, VariableValue>> {
           // Interactive prompting with validation
           // Support for different input types
           // Conditional prompts based on previous answers
       }
   }
   ```

### Phase 4: Advanced Template Features

1. **Template Inheritance**
   ```handlebars
   {{!-- base.hbs --}}
   # {{PROJECT_NAME}}
   
   {{#block "description"}}
   Default project description
   {{/block}}
   
   ## Features
   {{#block "features"}}
   - Default feature list
   {{/block}}
   
   {{!-- child.hbs --}}
   {{extends "base"}}
   
   {{#block "description"}}
   {{PROJECT_NAME}} is a {{language}} application
   {{/block}}
   
   {{#block "features"}}
   {{#each selected_features}}
   - {{this}}
   {{/each}}
   {{/block}}
   ```

2. **Computed Variables**
   ```rust
   pub struct ComputedVariableEvaluator {
       expressions: HashMap<String, CompiledExpression>,
   }
   
   // Configuration example
   [computed]
   snake_case_name = "PROJECT_NAME | snake_case"
   has_database = "use_database && (database_type != '')"
   test_command = "if is_library then 'cargo test' else 'cargo test --bin ' + PROJECT_NAME"
   ```

## Testing Strategy

### Unit Tests

1. **Template Parser Tests** (`src/template/engine/parser.rs`)
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_basic_variable_parsing() {
           let template = "Hello {{name}}!";
           let nodes = TemplateParser::parse(template).unwrap();
           
           assert_eq!(nodes.len(), 3);
           assert!(matches!(nodes[0], Node::Text(_)));
           assert!(matches!(nodes[1], Node::Variable(_)));
           assert!(matches!(nodes[2], Node::Text(_)));
       }
       
       #[test]
       fn test_conditional_block_parsing() {
           let template = "{{#if enabled}}Feature enabled{{/if}}";
           let nodes = TemplateParser::parse(template).unwrap();
           
           assert_eq!(nodes.len(), 1);
           assert!(matches!(nodes[0], Node::Conditional(_)));
       }
       
       #[test]
       fn test_nested_expressions() {
           let template = "{{#if (eq type 'library')}}Library mode{{/if}}";
           let result = TemplateParser::parse(template);
           assert!(result.is_ok());
       }
   }
   ```

2. **Variable Resolution Tests** (`src/template/variables/resolver.rs`)
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_built_in_variable_resolution() {
           let resolver = VariableResolver::new();
           let context = Context::new();
           
           let value = resolver.resolve("PROJECT_NAME", &context).unwrap();
           assert!(matches!(value, VariableValue::String(_)));
       }
       
       #[test]
       fn test_custom_variable_resolution() {
           let mut context = Context::new();
           context.set("my_var", VariableValue::String("test".to_string()));
           
           let resolver = VariableResolver::new();
           let value = resolver.resolve("my_var", &context).unwrap();
           assert_eq!(value, VariableValue::String("test".to_string()));
       }
   }
   ```

3. **Filter Tests** (`src/template/engine/filters/`)
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_snake_case_filter() {
           let filter = SnakeCaseFilter;
           let input = VariableValue::String("MyProject".to_string());
           let result = filter.apply(&input, &[]).unwrap();
           
           assert_eq!(result, VariableValue::String("my_project".to_string()));
       }
       
       #[test]
       fn test_date_format_filter() {
           let filter = DateFormatFilter;
           let input = VariableValue::String("2023-12-25".to_string());
           let format = VariableValue::String("%B %d, %Y".to_string());
           let result = filter.apply(&input, &[format]).unwrap();
           
           assert_eq!(result, VariableValue::String("December 25, 2023".to_string()));
       }
   }
   ```

### Integration Tests

1. **Template Processing Tests** (`tests/template_processing.rs`)
   ```rust
   #[tokio::test]
   async fn test_conditional_template_processing() {
       let template_content = r#"
       {{#if use_database}}
       use database::Connection;
       {{/if}}
       
       fn main() {
           {{#unless use_database}}
           println!("No database");
           {{/unless}}
       }
       "#;
       
       let mut variables = HashMap::new();
       variables.insert("use_database".to_string(), VariableValue::Boolean(true));
       
       let engine = TemplateEngine::new();
       let result = engine.render(template_content, &Context::from(variables)).unwrap();
       
       assert!(result.contains("use database::Connection;"));
       assert!(!result.contains("No database"));
   }
   
   #[tokio::test]
   async fn test_iteration_template_processing() {
       let template_content = r#"
       [dependencies]
       {{#each dependencies}}
       {{@key}} = "{{this}}"
       {{/each}}
       "#;
       
       let deps = HashMap::new();
       deps.insert("serde".to_string(), VariableValue::String("1.0".to_string()));
       deps.insert("tokio".to_string(), VariableValue::String("1.0".to_string()));
       
       let mut variables = HashMap::new();
       variables.insert("dependencies".to_string(), VariableValue::Object(deps));
       
       let engine = TemplateEngine::new();
       let result = engine.render(template_content, &Context::from(variables)).unwrap();
       
       assert!(result.contains("serde = \"1.0\""));
       assert!(result.contains("tokio = \"1.0\""));
   }
   ```

2. **CLI Integration Tests** (`tests/cli_variables.rs`)
   ```rust
   #[tokio::test]
   async fn test_variable_passing_via_cli() {
       let temp_dir = tempdir().unwrap();
       
       let result = run_cli_command(&[
           "new",
           "rust",
           "test-project",
           "--path", temp_dir.path().to_str().unwrap(),
           "--var", "use_database=true",
           "--var", "database_type=postgres",
           "--no-prompt"
       ]).await;
       
       assert!(result.is_ok());
       
       let cargo_toml = temp_dir.path().join("Cargo.toml");
       let content = std::fs::read_to_string(cargo_toml).unwrap();
       assert!(content.contains("postgres"));
   }
   ```

### Mock Requirements

1. **Git Repository Mocking**
   - Mock template repositories with various template configurations
   - Mock template files with different variable usage patterns
   - Mock partial templates and includes

2. **Interactive Input Mocking**
   - Mock stdin for testing interactive prompts
   - Mock different user input scenarios
   - Mock validation failures and recovery

## Edge Cases & Error Handling

### Template Syntax Errors
- **Invalid Syntax**: Clear error messages with line/column information
- **Unclosed Blocks**: Detect and report missing closing tags
- **Invalid Expressions**: Validate expression syntax and provide helpful suggestions
- **Circular Dependencies**: Detect and prevent infinite recursion in includes/inheritance

### Variable Resolution Errors
- **Missing Variables**: List required but undefined variables
- **Type Mismatches**: Clear errors when filters expect different types
- **Validation Failures**: Report custom validation rule violations
- **Circular References**: Prevent infinite loops in computed variables

### Template Processing Errors
- **File Access Issues**: Handle permission errors gracefully
- **Memory Limits**: Prevent excessive memory usage on large templates
- **Timeout Handling**: Limit processing time for complex templates
- **Partial Loading Failures**: Graceful degradation when includes fail

### Error Types

```rust
#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("Template syntax error at line {line}, column {column}: {message}")]
    SyntaxError { line: usize, column: usize, message: String },
    
    #[error("Variable '{name}' is not defined")]
    UndefinedVariable { name: String },
    
    #[error("Filter '{name}' failed: {reason}")]
    FilterError { name: String, reason: String },
    
    #[error("Template validation failed: {errors:?}")]
    ValidationError { errors: Vec<ValidationIssue> },
    
    #[error("Circular dependency detected: {cycle:?}")]
    CircularDependency { cycle: Vec<String> },
    
    #[error("Template processing timeout after {duration:?}")]
    ProcessingTimeout { duration: std::time::Duration },
}

#[derive(Debug, Clone)]
pub struct ValidationIssue {
    pub level: ValidationLevel,
    pub message: String,
    pub location: Option<SourceLocation>,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ValidationLevel {
    Error,
    Warning,
    Info,
}
```

## Dependencies

### New External Dependencies
- **handlebars**: Template engine (or similar like `tera` or `minijinja`)
- **chrono**: Date/time formatting for date filters
- **regex**: Pattern matching for advanced filters
- **indexmap**: Ordered maps for preserving variable order
- **inquire**: Interactive prompts for variable input

### Enhanced Dependencies
- **serde**: Extended serialization for complex variable types
- **toml**: Enhanced TOML parsing for template configurations
- **clap**: Extended CLI parsing for variable flags

### Version Requirements
```toml
[dependencies]
handlebars = "4.4"
chrono = { version = "0.4", features = ["serde"] }
regex = "1.9"
indexmap = { version = "2.0", features = ["serde"] }
inquire = "0.6"
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
```

## Configuration

### Template Configuration File

```toml
# .claudeforge.toml in template repository root
[template]
name = "Advanced Rust Template"
description = "Rust template with configurable features"
version = "1.0.0"
min_claudeforge_version = "0.2.0"

[variables]
# Simple variables
project_type = { type = "choice", choices = ["bin", "lib"], default = "bin" }
use_async = { type = "boolean", default = false }
license = { type = "choice", choices = ["MIT", "Apache-2.0", "GPL-3.0"], default = "MIT" }

# Complex variables
features = { type = "array", item_type = "string", default = ["cli"] }
dependencies = { type = "object", default = {} }

# Computed variables
[computed]
snake_case_name = "PROJECT_NAME | snake_case"
has_binary = "project_type == 'bin'"
cargo_bin_section = "if has_binary then '[bin]\\nname = \"' + snake_case_name + '\"' else ''"

# Interactive prompts
[[prompts]]
variable = "project_type"
message = "What type of Rust project?"
help = "Binary projects have a main function, libraries are reusable code"

[[prompts]]
variable = "use_async" 
message = "Use async/await?"
condition = "project_type == 'bin'"

[[prompts]]
variable = "features"
message = "Select features (space-separated)"
input_type = "multiselect"
options = ["cli", "logging", "config", "database", "web"]

# File processing rules
[[files]]
pattern = "**/*.rs"
engine = "handlebars"

[[files]]
pattern = "Cargo.toml"
engine = "handlebars"

[[files]]
pattern = "**/*.md"
engine = "handlebars"

[[files]]
pattern = "**/*.json"
skip_if = "false"  # Process all JSON files

# Include paths for partials
[includes]
paths = ["templates/partials", "includes"]

# Custom filters (optional)
[filters]
rust_case = { command = "rust-case-converter", args = ["--input", "{input}"] }
```

### User Configuration Extensions

```toml
# ~/.config/claudeforge/config.toml
[defaults]
author_name = "Jane Developer"
author_email = "jane@example.com"

[templates]
cache_directory = "~/.cache/claudeforge"
auto_update = true
update_interval_days = 7

# Template variable defaults
[template_variables]
license = "MIT"
features = ["cli", "logging"]
use_async = true

# Prompt behavior
[prompts]
skip_defaults = false  # Skip prompts when defaults exist
timeout_seconds = 300  # Timeout for interactive prompts
```

### Environment Variables
- `CLAUDEFORGE_TEMPLATE_TIMEOUT`: Template processing timeout
- `CLAUDEFORGE_TEMPLATE_ENGINE`: Override default template engine
- `CLAUDEFORGE_NO_PROMPT`: Skip all interactive prompts
- `CLAUDEFORGE_VARS_FILE`: Default variables file path

## Documentation

### User Documentation

#### Quick Start Guide
```markdown
## Template Variables

ClaudeForge templates support advanced variable replacement with conditional logic, filters, and interactive prompts.

### Basic Usage

```bash
# Use default variables
claudeforge new rust my-project

# Set specific variables
claudeforge new rust my-project --var use_database=true --var database_type=postgres

# Use variables file
claudeforge new rust my-project --vars-file my-vars.toml
```

### Variable Types

Templates can define different types of variables:

- **String**: Text values (`name = "my-project"`)
- **Boolean**: True/false values (`use_database = true`)
- **Number**: Numeric values (`port = 8080`)
- **Array**: Lists of values (`features = ["cli", "web"]`)
- **Object**: Key-value pairs (`dependencies = { serde = "1.0" }`)
- **Choice**: Select from predefined options (`license = "MIT"`)
```

#### Template Syntax Reference
```markdown
## Template Syntax

### Variables
```handlebars
{{PROJECT_NAME}}          <!-- Built-in variable -->
{{my_variable}}           <!-- Custom variable -->
{{config.database_type}}  <!-- Nested variable -->
```

### Conditionals
```handlebars
{{#if use_database}}
Database configuration here
{{/if}}

{{#unless is_library}}
Binary-specific code
{{/unless}}
```

### Loops
```handlebars
{{#each dependencies}}
{{@key}} = "{{this}}"
{{/each}}

{{#for i in 0..test_count}}
Test case {{i}}
{{/for}}
```

### Filters
```handlebars
{{PROJECT_NAME | snake_case}}     <!-- my_project -->
{{AUTHOR_NAME | uppercase}}       <!-- JANE DOE -->
{{CURRENT_DATE | format "%Y"}}    <!-- 2023 -->
```
```

### API Documentation

```rust
/// Advanced template engine with support for variables, conditionals, and filters.
///
/// The `TemplateEngine` processes templates with Handlebars-like syntax,
/// supporting complex logic and transformations for generating project files.
///
/// # Examples
///
/// ```rust
/// use claudeforge::template::engine::TemplateEngine;
/// use claudeforge::template::variables::{Context, VariableValue};
/// use std::collections::HashMap;
///
/// let engine = TemplateEngine::new();
/// 
/// let mut variables = HashMap::new();
/// variables.insert("name".to_string(), VariableValue::String("MyProject".to_string()));
/// variables.insert("use_db".to_string(), VariableValue::Boolean(true));
///
/// let context = Context::from(variables);
/// let template = r#"
/// # {{name}}
/// {{#if use_db}}
/// Database: enabled
/// {{/if}}
/// "#;
///
/// let result = engine.render(template, &context)?;
/// println!("{}", result);
/// ```
pub struct TemplateEngine {
    // ...
}

impl TemplateEngine {
    /// Creates a new template engine with default configuration.
    pub fn new() -> Self;
    
    /// Creates a template engine with custom variables.
    pub fn with_variables(variables: TemplateVariables) -> Self;
    
    /// Renders a template string with the given context.
    /// 
    /// # Arguments
    /// 
    /// * `template` - The template string to render
    /// * `context` - Variable context for template rendering
    /// 
    /// # Returns
    /// 
    /// Returns the rendered string or a template error.
    pub fn render(&self, template: &str, context: &Context) -> Result<String, TemplateError>;
}
```

## Example Usage

### CLI Usage Examples

```bash
# Basic project with custom variables
claudeforge new rust my-api \
  --var use_database=true \
  --var database_type=postgres \
  --var features='["cli","logging","database"]'

# Using variables file
cat > my-vars.toml << EOF
use_database = true
database_type = "postgres"
license = "MIT"
features = ["cli", "logging", "database", "web"]

[dependencies]
serde = "1.0"
tokio = "1.0"
sqlx = "0.7"
EOF

claudeforge new rust my-service --vars-file my-vars.toml

# Non-interactive mode with defaults
claudeforge new rust automated-project --no-prompt

# Preview variables before creation
claudeforge new rust my-project --dry-run --show-vars
```

### Template Examples

#### Cargo.toml with Variables
```handlebars
[package]
name = "{{PROJECT_NAME | snake_case}}"
version = "0.1.0"
edition = "2021"
authors = ["{{AUTHOR_NAME}} <{{AUTHOR_EMAIL}}>"]
license = "{{license}}"
description = "{{description | default 'A Rust project'}}"

{{#if (eq project_type "bin")}}
[[bin]]
name = "{{PROJECT_NAME | snake_case}}"
path = "src/main.rs"
{{/if}}

[dependencies]
{{#each dependencies}}
{{@key}} = "{{this}}"
{{/each}}

{{#if use_async}}
tokio = { version = "1.0", features = ["full"] }
{{/if}}

{{#if (contains features "cli")}}
clap = { version = "4.0", features = ["derive"] }
{{/if}}

{{#if (contains features "logging")}}
tracing = "0.1"
tracing-subscriber = "0.3"
{{/if}}

{{#if (contains features "config")}}
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
{{/if}}

[dev-dependencies]
{{#if use_async}}
tokio-test = "0.4"
{{/if}}
```

#### Conditional Source Files
```handlebars
{{!-- src/main.rs --}}
{{#if (contains features "logging")}}
use tracing::{info, error};
{{/if}}
{{#if (contains features "config")}}
use serde::{Serialize, Deserialize};
{{/if}}

{{#if (contains features "config")}}
#[derive(Serialize, Deserialize)]
struct Config {
    {{#if use_database}}
    database_url: String,
    {{/if}}
    {{#if (contains features "web")}}
    port: u16,
    {{/if}}
}
{{/if}}

{{#if use_async}}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
{{else}}
fn main() -> Result<(), Box<dyn std::error::Error>> {
{{/if}}
    {{#if (contains features "logging")}}
    tracing_subscriber::init();
    info!("Starting {{PROJECT_NAME}}");
    {{/if}}
    
    {{#if (contains features "config")}}
    let config = load_config()?;
    {{/if}}
    
    {{#if use_database}}
    let db = connect_database(&config.database_url){{#if use_async}}.await{{/if}}?;
    {{/if}}
    
    println!("Hello from {{PROJECT_NAME}}!");
    
    Ok(())
}

{{#if (contains features "config")}}
fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = std::fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}
{{/if}}

{{#if use_database}}
{{#if use_async}}
async fn connect_database(url: &str) -> Result<Database, Box<dyn std::error::Error>> {
{{else}}
fn connect_database(url: &str) -> Result<Database, Box<dyn std::error::Error>> {
{{/if}}
    // Database connection logic
    todo!("Implement database connection")
}
{{/if}}
```

#### README with Dynamic Content
```handlebars
# {{PROJECT_NAME}}

{{description | default "A Rust project created with ClaudeForge"}}

## Features

{{#each features}}
- {{this | title_case}}
{{/each}}

## Installation

```bash
cargo install --path .
```

## Usage

{{#if (eq project_type "bin")}}
```bash
./target/release/{{PROJECT_NAME | snake_case}}
```
{{else}}
Add this to your `Cargo.toml`:

```toml
[dependencies]
{{PROJECT_NAME | snake_case}} = "0.1.0"
```
{{/if}}

{{#if use_database}}
## Database Setup

This project uses {{database_type | title_case}}. Set up your database and configure the connection string in `config.toml`:

```toml
database_url = "{{#if (eq database_type "postgres")}}postgresql://{{else if (eq database_type "mysql")}}mysql://{{else}}sqlite://{{/if}}..."
```
{{/if}}

## Development

```bash
# Run tests
cargo test

{{#if use_async}}
# Run with async runtime
cargo run
{{else}}
# Run binary
cargo run
{{/if}}
```

## License

This project is licensed under the {{license}} license.
```

### Variables File Examples

```toml
# Basic web service configuration
project_type = "bin"
use_async = true
use_database = true
database_type = "postgres"
license = "MIT"
description = "A high-performance web API service"

features = [
    "cli", 
    "logging", 
    "config", 
    "database", 
    "web"
]

[dependencies]
axum = "0.7"
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls"] }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"

# Library configuration  
[library_config]
project_type = "lib"
use_async = false
use_database = false
license = "Apache-2.0"
description = "A utility library for data processing"

features = ["config"]

[library_config.dependencies]
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
```