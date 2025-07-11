# Template Splicing Feature Specification

## Feature Summary

The Template Splicing feature enables ClaudeForge to combine multiple Git repository templates when creating a new project. This allows users to compose projects from modular templates, such as combining a base language template with CI/CD configurations, documentation templates, or other auxiliary components stored in separate repositories.

### Problem Statement
Currently, ClaudeForge supports cloning a single template repository. Users who want to combine features from multiple templates must manually merge them after project creation. This is particularly common when users want to:
- Add CI/CD workflows to a base project template
- Combine language-specific templates with deployment configurations
- Mix organizational standards with open-source templates
- Layer security configurations onto existing templates

### Solution Overview
Implement a template splicing system that can intelligently merge multiple Git repositories into a single project, handling file conflicts, directory structures, and configuration merging according to user-defined rules.

## Goals & Requirements

### Functional Requirements
1. **Multiple Template Support**: Accept multiple template repository URLs during project creation
2. **Merge Strategies**: Provide configurable merge strategies for handling conflicts
3. **Order Control**: Allow users to specify the order in which templates are applied
4. **Selective Merging**: Enable users to include/exclude specific paths from templates
5. **Configuration Merging**: Intelligently merge configuration files (JSON, YAML, TOML)
6. **Dependency Resolution**: Combine dependency files (package.json, Cargo.toml, go.mod)
7. **Template Validation**: Verify template compatibility before merging

### Non-Functional Requirements
1. **Performance**: Template splicing should complete within reasonable time (< 30s for typical templates)
2. **Reliability**: Failed splicing should not leave partial projects
3. **Transparency**: Clear reporting of merge actions and conflicts
4. **Extensibility**: Plugin system for custom merge handlers
5. **Backward Compatibility**: Existing single-template functionality must continue working

### Success Criteria
- Users can create projects from 2+ templates with single command
- Common configuration files merge intelligently without manual intervention
- Conflict resolution follows predictable, documented rules
- Process provides clear feedback about actions taken
- Existing workflows remain unaffected

## API/Interface Design

### Command Line Interface
```bash
# Basic usage with multiple templates
claudeforge init myproject --template base=github.com/org/rust-base --template ci=github.com/org/rust-ci

# With merge strategy
claudeforge init myproject \
  --template base=github.com/org/rust-base \
  --template ci=github.com/org/rust-ci \
  --merge-strategy=overlay

# With path filtering
claudeforge init myproject \
  --template base=github.com/org/rust-base \
  --template ci=github.com/org/rust-ci:.github/workflows \
  --template docs=github.com/org/docs-template:docs/

# With priority ordering
claudeforge init myproject \
  --template base=github.com/org/rust-base:priority=1 \
  --template ci=github.com/org/rust-ci:priority=2
```

### Configuration File Support
```yaml
# .claudeforge.yml
templates:
  - name: base
    url: github.com/org/rust-base
    priority: 1
  - name: ci
    url: github.com/org/rust-ci
    priority: 2
    paths:
      include:
        - .github/workflows
        - .gitlab-ci.yml
  - name: docs
    url: github.com/org/docs-template
    paths:
      include:
        - docs/
        - README.md
    merge:
      strategy: overlay
      
merge:
  strategy: smart  # smart, overlay, underlay, fail
  configs:
    - pattern: "*.json"
      handler: json-merge
    - pattern: "*.yml"
      handler: yaml-merge
    - pattern: "Cargo.toml"
      handler: toml-merge
```

### Core Types and Interfaces
```rust
// Template source definition
pub struct TemplateSource {
    pub name: String,
    pub url: String,
    pub branch: Option<String>,
    pub priority: i32,
    pub paths: PathFilter,
    pub merge_config: MergeConfig,
}

// Path filtering
pub struct PathFilter {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

// Merge configuration
pub struct MergeConfig {
    pub strategy: MergeStrategy,
    pub file_handlers: Vec<FileHandler>,
}

pub enum MergeStrategy {
    Smart,      // Intelligent merging based on file type
    Overlay,    // Later templates overwrite earlier ones
    Underlay,   // Earlier templates take precedence
    Fail,       // Fail on any conflict
}

// File handler for specific patterns
pub struct FileHandler {
    pub pattern: String,
    pub handler: Box<dyn MergeHandler>,
}

// Merge handler trait
pub trait MergeHandler {
    fn can_handle(&self, path: &Path) -> bool;
    fn merge(&self, base: &[u8], overlay: &[u8]) -> Result<Vec<u8>, MergeError>;
}

// Template splicer main interface
pub struct TemplateSplicer {
    pub sources: Vec<TemplateSource>,
    pub target_dir: PathBuf,
    pub merge_config: MergeConfig,
}

impl TemplateSplicer {
    pub fn new(target_dir: PathBuf) -> Self;
    pub fn add_source(&mut self, source: TemplateSource);
    pub fn splice(&self) -> Result<SpliceReport, SpliceError>;
}

// Splice report
pub struct SpliceReport {
    pub merged_files: Vec<MergedFile>,
    pub conflicts: Vec<Conflict>,
    pub skipped_files: Vec<SkippedFile>,
}
```

## File and Package Structure

### Package Organization
```
internal/
├── splice/                    # Template splicing core
│   ├── splicer.rs            # Main splicing orchestrator
│   ├── source.rs             # Template source handling
│   ├── merger.rs             # File merging logic
│   ├── handlers/             # Merge handlers
│   │   ├── mod.rs
│   │   ├── json.rs           # JSON merge handler
│   │   ├── yaml.rs           # YAML merge handler
│   │   ├── toml.rs           # TOML merge handler
│   │   ├── text.rs           # Text file merge handler
│   │   └── binary.rs         # Binary file handler
│   ├── filter.rs             # Path filtering
│   ├── conflict.rs           # Conflict resolution
│   └── report.rs             # Splice reporting
├── template/                  # Existing template module
│   └── (existing files)      # Extend with splice support
└── config/                    # Configuration updates
    └── splice.rs             # Splice configuration

cmd/
└── claudeforge/
    └── commands/
        └── init.rs           # Update init command
```

### Test Structure
```
internal/splice/
├── splicer_test.rs
├── handlers/
│   ├── json_test.rs
│   ├── yaml_test.rs
│   └── toml_test.rs
└── testdata/
    ├── templates/
    │   ├── base/
    │   ├── ci/
    │   └── docs/
    └── expected/
        └── merged/
```

## Implementation Details

### Phase 1: Core Splicing Infrastructure
1. Create `TemplateSplicer` struct and basic interfaces
2. Implement template source resolution and cloning
3. Build file system traversal and comparison logic
4. Create basic overlay/underlay merge strategies

### Phase 2: Smart Merge Handlers
1. Implement JSON merge handler
   - Deep merge objects
   - Concatenate arrays with deduplication option
   - Handle version conflicts in package.json
2. Implement YAML merge handler
   - Preserve comments where possible
   - Handle multi-document YAML files
   - Merge sequences intelligently
3. Implement TOML merge handler
   - Merge tables and arrays
   - Handle Cargo.toml dependencies specially
4. Create text file merge handler
   - Line-based merging
   - Conflict markers for manual resolution

### Phase 3: CLI Integration
1. Extend `init` command with multiple `--template` flags
2. Parse template specifications with options
3. Add configuration file support
4. Implement progress reporting during splice

### Phase 4: Advanced Features
1. Template compatibility checking
2. Dependency conflict resolution
3. Post-splice hooks for cleanup
4. Template splice preview mode

### Key Algorithms

#### File Merging Decision Tree
```rust
fn should_merge(base_path: &Path, overlay_path: &Path, config: &MergeConfig) -> MergeAction {
    if !base_path.exists() {
        return MergeAction::Copy;
    }
    
    if let Some(handler) = config.find_handler(base_path) {
        if handler.can_handle(base_path) {
            return MergeAction::Merge(handler);
        }
    }
    
    match config.strategy {
        MergeStrategy::Overlay => MergeAction::Replace,
        MergeStrategy::Underlay => MergeAction::Skip,
        MergeStrategy::Fail => MergeAction::Conflict,
        MergeStrategy::Smart => determine_smart_action(base_path),
    }
}
```

#### Dependency Merging (Cargo.toml example)
```rust
fn merge_cargo_toml(base: &str, overlay: &str) -> Result<String> {
    let mut base_doc = base.parse::<Document>()?;
    let overlay_doc = overlay.parse::<Document>()?;
    
    // Merge dependencies
    if let Some(overlay_deps) = overlay_doc.get("dependencies") {
        let base_deps = base_doc.entry("dependencies").or_insert(table());
        merge_tables(base_deps, overlay_deps);
    }
    
    // Merge dev-dependencies
    if let Some(overlay_dev) = overlay_doc.get("dev-dependencies") {
        let base_dev = base_doc.entry("dev-dependencies").or_insert(table());
        merge_tables(base_dev, overlay_dev);
    }
    
    // Handle version conflicts
    resolve_version_conflicts(&mut base_doc)?;
    
    Ok(base_doc.to_string())
}
```

## Testing Strategy

### Unit Tests
1. **Splicer Core Tests**
   - Test template source parsing
   - Test priority ordering
   - Test path filtering
   - Test merge strategy selection

2. **Merge Handler Tests**
   - Test each handler with valid inputs
   - Test conflict scenarios
   - Test malformed file handling
   - Test edge cases (empty files, large files)

3. **Integration Tests**
   - Test full splice operations
   - Test multi-template scenarios
   - Test configuration file parsing
   - Test CLI argument parsing

### Test Scenarios
```rust
#[test]
fn test_basic_template_splice() {
    let temp_dir = tempdir().unwrap();
    let splicer = TemplateSplicer::new(temp_dir.path());
    
    splicer.add_source(TemplateSource {
        name: "base".to_string(),
        url: "testdata/templates/base".to_string(),
        priority: 1,
        ..Default::default()
    });
    
    splicer.add_source(TemplateSource {
        name: "ci".to_string(),
        url: "testdata/templates/ci".to_string(),
        priority: 2,
        ..Default::default()
    });
    
    let report = splicer.splice().unwrap();
    assert_eq!(report.conflicts.len(), 0);
    assert!(temp_dir.path().join(".github/workflows/rust.yml").exists());
}

#[test]
fn test_json_merge_handler() {
    let handler = JsonMergeHandler::new();
    let base = r#"{"name": "project", "version": "1.0.0"}"#;
    let overlay = r#"{"version": "2.0.0", "author": "user"}"#;
    
    let result = handler.merge(base.as_bytes(), overlay.as_bytes()).unwrap();
    let merged: Value = serde_json::from_slice(&result).unwrap();
    
    assert_eq!(merged["name"], "project");
    assert_eq!(merged["version"], "2.0.0");
    assert_eq!(merged["author"], "user");
}
```

## Edge Cases & Error Handling

### Edge Cases
1. **Circular Template Dependencies**: Detect and prevent infinite loops
2. **Large Files**: Stream processing for files > 100MB
3. **Binary Files**: Skip merging, use overlay strategy
4. **Symbolic Links**: Follow links with cycle detection
5. **Empty Templates**: Handle gracefully, warn user
6. **Network Failures**: Retry with exponential backoff
7. **Permission Issues**: Clear error messages for access denied

### Error Handling Patterns
```rust
pub enum SpliceError {
    TemplateNotFound { name: String, url: String },
    NetworkError { source: reqwest::Error },
    MergeConflict { path: PathBuf, details: String },
    InvalidConfiguration { message: String },
    IoError { source: io::Error },
}

impl TemplateSplicer {
    pub fn splice(&self) -> Result<SpliceReport, SpliceError> {
        // Validate all templates exist before starting
        self.validate_sources()?;
        
        // Create transaction-like behavior
        let temp_target = self.create_temp_target()?;
        
        match self.splice_to_temp(&temp_target) {
            Ok(report) => {
                self.commit_splice(temp_target)?;
                Ok(report)
            }
            Err(e) => {
                self.rollback_splice(temp_target)?;
                Err(e)
            }
        }
    }
}
```

## Dependencies

### External Crates
- `git2`: Git operations for cloning templates
- `serde_json`: JSON parsing and merging
- `serde_yaml`: YAML parsing and merging
- `toml_edit`: TOML parsing with format preservation
- `glob`: Path pattern matching
- `similar`: Text diffing for merge conflicts
- `tokio`: Async runtime for parallel operations
- `indicatif`: Progress bars for user feedback

### Internal Dependencies
- `internal/config`: Configuration management
- `internal/template`: Existing template functionality
- `internal/git`: Git operations wrapper
- `internal/logger`: Logging infrastructure

### Version Requirements
- Rust 1.70+ (for improved async traits)
- Compatible with existing ClaudeForge dependencies

## Configuration

### New Configuration Options
```rust
pub struct SpliceConfig {
    // Global merge strategy
    pub default_strategy: MergeStrategy,
    
    // Maximum template depth for recursive templates
    pub max_template_depth: usize,
    
    // Network timeout for template fetching
    pub fetch_timeout: Duration,
    
    // Enable parallel template fetching
    pub parallel_fetch: bool,
    
    // Cache directory for templates
    pub cache_dir: Option<PathBuf>,
    
    // File size limit for merge operations
    pub max_merge_file_size: usize,
}

impl Default for SpliceConfig {
    fn default() -> Self {
        Self {
            default_strategy: MergeStrategy::Smart,
            max_template_depth: 5,
            fetch_timeout: Duration::from_secs(300),
            parallel_fetch: true,
            cache_dir: None,
            max_merge_file_size: 10 * 1024 * 1024, // 10MB
        }
    }
}
```

### Environment Variables
- `CLAUDEFORGE_SPLICE_STRATEGY`: Override default merge strategy
- `CLAUDEFORGE_SPLICE_CACHE`: Template cache directory
- `CLAUDEFORGE_SPLICE_TIMEOUT`: Network timeout in seconds
- `CLAUDEFORGE_SPLICE_PARALLEL`: Enable/disable parallel fetching

## Documentation

### User Documentation
1. **Quick Start Guide**
   - Basic template splicing examples
   - Common use cases (base + CI, base + docs)
   - Troubleshooting guide

2. **Advanced Usage**
   - Custom merge handlers
   - Configuration file reference
   - Merge strategy details
   - Path filtering syntax

3. **Template Author Guide**
   - Best practices for splice-friendly templates
   - Metadata for template compatibility
   - Testing template combinations

### API Documentation
```rust
/// Splices multiple Git repository templates into a single project.
///
/// The `TemplateSplicer` combines templates in priority order, handling
/// file conflicts according to the configured merge strategy.
///
/// # Examples
///
/// ```
/// use claudeforge::splice::{TemplateSplicer, TemplateSource};
///
/// let mut splicer = TemplateSplicer::new("./myproject");
/// 
/// splicer.add_source(TemplateSource {
///     name: "base".to_string(),
///     url: "github.com/org/rust-base".to_string(),
///     priority: 1,
///     ..Default::default()
/// });
///
/// splicer.add_source(TemplateSource {
///     name: "ci".to_string(),
///     url: "github.com/org/rust-ci".to_string(),
///     priority: 2,
///     ..Default::default()
/// });
///
/// let report = splicer.splice()?;
/// println!("Merged {} files", report.merged_files.len());
/// ```
pub struct TemplateSplicer {
    // ...
}
```

### Example Usage Scenarios

#### Scenario 1: Rust Project with CI
```bash
claudeforge init my-rust-app \
  --template base=github.com/rust-templates/base \
  --template ci=github.com/rust-templates/github-actions
```

#### Scenario 2: Microservice with Deployment
```bash
claudeforge init my-service \
  --template app=github.com/company/go-service-template \
  --template k8s=github.com/company/k8s-configs:deploy/ \
  --template monitoring=github.com/company/observability
```

#### Scenario 3: Documentation Site with Blog
```yaml
# .claudeforge.yml
templates:
  - name: site
    url: github.com/templates/docusaurus
    priority: 1
  - name: blog
    url: github.com/templates/blog-addon
    priority: 2
    paths:
      include:
        - blog/
        - sidebars.js
  - name: theme
    url: github.com/company/brand-theme
    priority: 3
```

## Migration & Rollout

### Phase 1: Alpha Release
- Feature flag: `--enable-splice`
- Limited to 2 templates maximum
- Basic merge strategies only

### Phase 2: Beta Release
- Remove feature flag
- Support unlimited templates
- Full merge handler suite
- Configuration file support

### Phase 3: General Availability
- Default behavior for multiple templates
- Template marketplace integration
- Community merge handlers

### Backward Compatibility
- Single `--template` flag continues to work
- No changes to existing project structure
- Existing configuration files remain valid