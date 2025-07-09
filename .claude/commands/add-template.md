# Add Template Command

Add a new template to the claudeforge registry.

## Usage
```
/add-template <language> <repo-url> <template-name> <description>
```

## Example
```
/add-template python https://github.com/iepathos/python-claude-code python-claude-code "Python project template optimized for Claude Code"
```

## Instructions

You need to:

1. **Add the language enum** (if it doesn't exist):
   - Add to `Language` enum in `src/cli.rs` if the language isn't already there
   - Add the corresponding `Display` implementation case
   - Use PascalCase for the enum variant (e.g., `Python`, `JavaScript`)

2. **Add the template entry** to `src/template/registry.rs`:
   - Insert a new template entry in the `load_template_registry()` function
   - Follow the existing pattern for other templates
   - Include appropriate `files_to_customize` entries for common files that need customization

3. **Build and test**:
   - Run `cargo build` to ensure it compiles
   - Run `./target/debug/claudeforge list` to verify the template appears

4. **Create git commit**:
   - Add all modified files to git
   - Create a commit with message: "Add {language} template: {template-name}"

## Template Structure Example

For the registry entry, follow this pattern:

```rust
// Language template
templates.insert(
    Language::LanguageName,
    Template {
        name: "template-name".to_string(),
        language: Language::LanguageName,
        repository: "repo-url".to_string(),
        description: "description".to_string(),
        files_to_customize: vec![
            FileCustomization {
                path: "common-file.ext".to_string(),
                replacements: vec![Replacement {
                    placeholder: "placeholder-text".to_string(),
                    value_type: ValueType::ProjectName,
                }],
            },
            // Add more file customizations as needed
        ],
    },
);
```

## Common File Customizations

Include customizations for typical files that need project-specific values:
- `README.md` - author name, project name
- Package files (`package.json`, `setup.py`, `Cargo.toml`, etc.) - project name, author
- License files - author name
- Configuration files - project-specific settings