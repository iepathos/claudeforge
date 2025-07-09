# Spec 040: Test Coverage Improvement

## Feature Summary

This spec outlines a comprehensive plan to improve test coverage for the claudeforge project from the current 61.14% to at least 80%. The improvement will focus on adding tests for uncovered code paths in critical modules, particularly `src/main.rs`, `src/config/mod.rs`, `src/template/loader.rs`, and `src/utils/fs.rs`, while maintaining existing test quality and performance.

## Goals & Requirements

### Functional Requirements
- Increase overall test coverage from 61.14% to at least 80%
- Add comprehensive tests for `src/main.rs` (currently 0/45 lines covered)
- Improve coverage for `src/config/mod.rs` from 3/29 to at least 23/29 lines
- Increase coverage for `src/template/loader.rs` from 21/45 to at least 36/45 lines
- Enhance coverage for `src/template/processor.rs` from 60/80 to at least 70/80 lines
- Improve coverage for `src/utils/fs.rs` from 17/27 to at least 22/27 lines

### Non-Functional Requirements
- Tests must run quickly (total test suite under 1 second)
- Use property-based testing where appropriate
- Maintain clear test organization and naming
- Ensure tests are deterministic and repeatable
- Mock external dependencies appropriately

### Success Criteria
- Cargo tarpaulin reports at least 80% overall coverage
- All new tests pass consistently
- No degradation in test execution time
- Tests cover critical error paths and edge cases

## API/Interface Design

### Test Module Organization

```rust
// src/main.rs
#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use tempfile::TempDir;
    
    #[test]
    fn test_cli_parsing() {
        // Test command parsing
    }
    
    #[test]
    fn test_init_command() {
        // Test init command execution
    }
    
    #[test]
    fn test_new_command() {
        // Test new command execution
    }
}
```

### Mock Traits for Testing

```rust
// src/config/mod.rs
#[cfg(test)]
pub trait MockConfigLoader {
    fn load_config(&self, path: &Path) -> Result<Config>;
    fn save_config(&self, config: &Config, path: &Path) -> Result<()>;
}

// src/template/loader.rs
#[cfg(test)]
pub trait MockTemplateLoader {
    fn load_template(&self, path: &Path) -> Result<Template>;
    fn validate_template(&self, template: &Template) -> Result<()>;
}
```

## File and Package Structure

### Test Files Organization
```
src/
├── main.rs (add #[cfg(test)] module)
├── config/
│   ├── mod.rs (add comprehensive tests)
│   └── tests.rs (integration tests if needed)
├── template/
│   ├── loader.rs (expand test coverage)
│   ├── processor.rs (add edge case tests)
│   └── registry.rs (maintain current coverage)
├── utils/
│   └── fs.rs (add error path tests)
└── git/
    └── mod.rs (improve error handling tests)

tests/
├── integration_tests.rs (new integration test suite)
└── cli_tests.rs (CLI integration tests)
```

## Implementation Details

### Phase 1: Main Module Tests (Priority: Critical)

```rust
// src/main.rs - Test implementation
#[cfg(test)]
mod tests {
    use super::*;
    use assert_cmd::Command;
    use predicates::prelude::*;
    use tempfile::TempDir;

    #[test]
    fn test_init_command_creates_config() {
        let temp_dir = TempDir::new().unwrap();
        let mut cmd = Command::cargo_bin("claudeforge").unwrap();
        
        cmd.arg("init")
           .current_dir(&temp_dir)
           .assert()
           .success();
        
        assert!(temp_dir.path().join(".claudeforge/config.toml").exists());
    }

    #[test]
    fn test_new_command_with_template() {
        let temp_dir = TempDir::new().unwrap();
        let mut cmd = Command::cargo_bin("claudeforge").unwrap();
        
        cmd.arg("new")
           .arg("my-project")
           .arg("--template")
           .arg("rust")
           .current_dir(&temp_dir)
           .assert()
           .success();
        
        assert!(temp_dir.path().join("my-project").exists());
    }

    #[test]
    fn test_update_command() {
        let mut cmd = Command::cargo_bin("claudeforge").unwrap();
        
        cmd.arg("update")
           .assert()
           .success();
    }

    #[test]
    fn test_list_command() {
        let mut cmd = Command::cargo_bin("claudeforge").unwrap();
        
        cmd.arg("list")
           .assert()
           .success()
           .stdout(predicate::str::contains("Available templates"));
    }
}
```

### Phase 2: Config Module Tests

```rust
// src/config/mod.rs - Additional tests
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_config_load_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        
        let config_content = r#"
        [settings]
        author = "Test Author"
        email = "test@example.com"
        
        [templates]
        rust = { repo = "https://github.com/example/rust-template" }
        "#;
        
        fs::write(&config_path, config_content).unwrap();
        
        let config = Config::load_from_path(&config_path).unwrap();
        assert_eq!(config.settings.author, Some("Test Author".to_string()));
        assert_eq!(config.settings.email, Some("test@example.com".to_string()));
    }

    #[test]
    fn test_config_load_missing_file() {
        let result = Config::load_from_path(Path::new("/nonexistent/config.toml"));
        assert!(result.is_err());
    }

    #[test]
    fn test_config_save() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        
        let config = Config {
            settings: Settings {
                author: Some("Test Author".to_string()),
                email: Some("test@example.com".to_string()),
                ..Default::default()
            },
            templates: HashMap::new(),
        };
        
        config.save_to_path(&config_path).unwrap();
        assert!(config_path.exists());
        
        let loaded = Config::load_from_path(&config_path).unwrap();
        assert_eq!(loaded.settings.author, config.settings.author);
    }

    #[test]
    fn test_config_merge() {
        let mut base = Config::default();
        let override_config = Config {
            settings: Settings {
                author: Some("New Author".to_string()),
                ..Default::default()
            },
            templates: HashMap::new(),
        };
        
        base.merge(override_config);
        assert_eq!(base.settings.author, Some("New Author".to_string()));
    }
}
```

### Phase 3: Template Loader Tests

```rust
// src/template/loader.rs - Additional tests
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_load_template_from_directory() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("template");
        fs::create_dir(&template_path).unwrap();
        
        let manifest_content = r#"
        name = "test-template"
        version = "1.0.0"
        description = "Test template"
        "#;
        
        fs::write(template_path.join("template.toml"), manifest_content).unwrap();
        fs::write(template_path.join("README.md"), "# Test Template").unwrap();
        
        let loader = TemplateLoader::new();
        let template = loader.load_from_path(&template_path).unwrap();
        
        assert_eq!(template.name, "test-template");
        assert_eq!(template.version, "1.0.0");
    }

    #[test]
    fn test_load_template_missing_manifest() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("template");
        fs::create_dir(&template_path).unwrap();
        
        let loader = TemplateLoader::new();
        let result = loader.load_from_path(&template_path);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("manifest"));
    }

    #[test]
    fn test_validate_template_structure() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("template");
        fs::create_dir(&template_path).unwrap();
        fs::create_dir(template_path.join("src")).unwrap();
        
        let loader = TemplateLoader::new();
        let validation = loader.validate_structure(&template_path);
        
        assert!(validation.is_ok());
    }

    #[test]
    fn test_load_template_with_variables() {
        let temp_dir = TempDir::new().unwrap();
        let template_path = temp_dir.path().join("template");
        fs::create_dir(&template_path).unwrap();
        
        let manifest_content = r#"
        name = "test-template"
        version = "1.0.0"
        
        [variables]
        project_name = { prompt = "Project name?", default = "my-project" }
        author = { prompt = "Author name?" }
        "#;
        
        fs::write(template_path.join("template.toml"), manifest_content).unwrap();
        
        let loader = TemplateLoader::new();
        let template = loader.load_from_path(&template_path).unwrap();
        
        assert_eq!(template.variables.len(), 2);
        assert!(template.variables.contains_key("project_name"));
        assert!(template.variables.contains_key("author"));
    }
}
```

### Phase 4: File System Utils Tests

```rust
// src/utils/fs.rs - Additional tests
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn test_copy_dir_recursive() {
        let temp_dir = TempDir::new().unwrap();
        let src = temp_dir.path().join("source");
        let dst = temp_dir.path().join("destination");
        
        fs::create_dir(&src).unwrap();
        fs::create_dir(src.join("subdir")).unwrap();
        fs::write(src.join("file.txt"), "content").unwrap();
        fs::write(src.join("subdir/nested.txt"), "nested content").unwrap();
        
        copy_dir_recursive(&src, &dst).unwrap();
        
        assert!(dst.exists());
        assert!(dst.join("file.txt").exists());
        assert!(dst.join("subdir/nested.txt").exists());
    }

    #[test]
    fn test_copy_dir_permission_error() {
        let temp_dir = TempDir::new().unwrap();
        let src = temp_dir.path().join("source");
        let dst = temp_dir.path().join("destination");
        
        fs::create_dir(&src).unwrap();
        fs::write(src.join("file.txt"), "content").unwrap();
        
        // Make destination directory read-only
        fs::create_dir(&dst).unwrap();
        fs::set_permissions(&dst, fs::Permissions::from_mode(0o444)).unwrap();
        
        let result = copy_dir_recursive(&src, &dst);
        assert!(result.is_err());
    }

    #[test]
    fn test_ensure_dir_exists() {
        let temp_dir = TempDir::new().unwrap();
        let nested = temp_dir.path().join("a/b/c/d");
        
        ensure_dir_exists(&nested).unwrap();
        assert!(nested.exists());
        assert!(nested.is_dir());
    }

    #[test]
    fn test_clean_empty_dirs() {
        let temp_dir = TempDir::new().unwrap();
        let nested = temp_dir.path().join("a/b/c");
        fs::create_dir_all(&nested).unwrap();
        
        clean_empty_dirs(temp_dir.path()).unwrap();
        assert!(!nested.exists());
        assert!(!temp_dir.path().join("a").exists());
    }
}
```

### Phase 5: Template Processor Edge Cases

```rust
// src/template/processor.rs - Additional tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_process_nested_variables() {
        let processor = TemplateProcessor::new();
        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "Test".to_string());
        vars.insert("version".to_string(), "1.0.0".to_string());
        
        let input = "Project {{name}} version {{version}} - {{name}}-{{version}}";
        let result = processor.process_string(input, &vars).unwrap();
        
        assert_eq!(result, "Project Test version 1.0.0 - Test-1.0.0");
    }

    #[test]
    fn test_process_escaped_brackets() {
        let processor = TemplateProcessor::new();
        let vars = HashMap::new();
        
        let input = "Use \\{{variable\\}} to reference variables";
        let result = processor.process_string(input, &vars).unwrap();
        
        assert_eq!(result, "Use {{variable}} to reference variables");
    }

    #[test]
    fn test_process_missing_variable() {
        let processor = TemplateProcessor::new();
        let vars = HashMap::new();
        
        let input = "Hello {{name}}!";
        let result = processor.process_string(input, &vars);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("name"));
    }

    #[test]
    fn test_process_conditional_blocks() {
        let processor = TemplateProcessor::new();
        let mut vars = HashMap::new();
        vars.insert("include_tests".to_string(), "true".to_string());
        
        let input = r#"
        fn main() {}
        {{#if include_tests}}
        #[cfg(test)]
        mod tests {
            #[test]
            fn test() {}
        }
        {{/if}}
        "#;
        
        let result = processor.process_string(input, &vars).unwrap();
        assert!(result.contains("mod tests"));
    }
}
```

## Testing Strategy

### Unit Test Requirements
- Each public function must have at least one test
- Error paths must be explicitly tested
- Edge cases (empty inputs, large inputs, special characters) must be covered
- Use property-based testing for string processing functions

### Integration Test Scenarios
1. Complete CLI workflow: init → new → build
2. Template loading and processing pipeline
3. Configuration loading and merging
4. Git operations with error recovery

### Test File Structure
```
tests/
├── integration_tests.rs
│   ├── test_full_project_creation
│   ├── test_template_update_workflow
│   └── test_error_recovery
└── cli_tests.rs
    ├── test_all_commands
    ├── test_help_output
    └── test_version_info
```

### Mock Requirements
- File system operations for permission errors
- Network operations for template downloads
- Git operations for repository interactions
- User input for interactive prompts

## Edge Cases & Error Handling

### Critical Edge Cases
1. **File System**
   - No write permissions
   - Disk full scenarios
   - Symlink handling
   - Unicode filenames

2. **Configuration**
   - Malformed TOML files
   - Missing required fields
   - Type mismatches
   - Circular references

3. **Templates**
   - Missing manifest files
   - Invalid variable syntax
   - Recursive template includes
   - Binary file handling

4. **CLI**
   - Invalid command combinations
   - Missing required arguments
   - Interrupted operations
   - Signal handling

### Error Recovery
- All errors should provide actionable messages
- Partial operations should be rolled back
- Temporary files should be cleaned up
- User should be informed of recovery options

## Dependencies

### Testing Dependencies
```toml
[dev-dependencies]
# Testing frameworks
proptest = "1.0"
mockall = "0.11"
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.0"

# Coverage tools
cargo-tarpaulin = "0.27"

# Benchmarking
criterion = "0.5"
```

### Platform-Specific Testing
- Use `cfg(test)` for test-only code
- Mock platform-specific operations
- Test on Windows, macOS, and Linux

## Configuration

### Test Configuration
```toml
# .cargo/config.toml
[alias]
test-coverage = "tarpaulin --out Html --output-dir target/coverage"
test-all = "test --all-features --workspace"
test-integration = "test --test integration_tests"
```

### Environment Variables
```bash
# For testing
CLAUDEFORGE_TEST_MODE=1
CLAUDEFORGE_TEST_CACHE_DIR=/tmp/claudeforge-test
CLAUDEFORGE_TEST_TIMEOUT=30
```

## Documentation

### Test Documentation Requirements
- Each test module must have a module-level doc comment
- Complex tests need inline explanations
- Test data files must be documented
- Coverage reports should be included in CI

### Example Documentation
```rust
//! Tests for the template processing module.
//!
//! These tests cover:
//! - Variable substitution
//! - Conditional blocks
//! - Error handling
//! - Edge cases

/// Tests that nested variables are properly replaced
/// and that multiple occurrences are handled correctly.
#[test]
fn test_nested_variable_substitution() {
    // Test implementation
}
```

## Implementation Timeline

1. **Week 1**: Main module and CLI tests (Priority: Critical)
2. **Week 2**: Config and template loader tests
3. **Week 3**: File system utils and error path tests
4. **Week 4**: Integration tests and coverage verification

## Success Metrics

- Overall coverage reaches 80% or higher
- All critical paths have test coverage
- Test execution time remains under 1 second
- No flaky tests in CI
- Clear test output and error messages