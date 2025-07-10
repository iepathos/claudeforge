use claudeforge::config::Config;

#[tokio::test]
async fn test_toml_missing_defaults_section() {
    // Test config missing defaults section
    let toml_missing_defaults = r#"
[templates]
auto_update = true
update_interval_days = 7
"#;

    let result: Result<Config, _> = toml::from_str(toml_missing_defaults);
    match result {
        Ok(config) => {
            println!("✅ Parsing succeeded");
            // Should use default values for missing defaults section
            assert!(config.defaults.author_name.is_none());
            assert!(config.defaults.author_email.is_none());
            assert!(config.defaults.default_directory.is_none());
        }
        Err(e) => {
            println!("❌ Parsing failed: {}", e);
            panic!("Should parse successfully with missing defaults section");
        }
    }
}

#[tokio::test]
async fn test_toml_empty_defaults_section() {
    // Test config with empty defaults section
    let toml_empty_defaults = r#"
[defaults]

[templates]
auto_update = true
update_interval_days = 7
"#;

    let result: Result<Config, _> = toml::from_str(toml_empty_defaults);
    match result {
        Ok(config) => {
            println!("✅ Empty defaults parsing succeeded");
            assert!(config.defaults.author_name.is_none());
            assert!(config.defaults.author_email.is_none());
            assert!(config.defaults.default_directory.is_none());
        }
        Err(e) => {
            println!("❌ Empty defaults parsing failed: {}", e);
            panic!("Empty defaults section should parse successfully");
        }
    }
}