use claudeforge::config::{Config, Defaults, TemplateConfig};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use tempfile::TempDir;
use tokio::sync::Mutex;

// Mutex to prevent parallel execution of tests that modify environment variables
static ENV_MUTEX: Mutex<()> = Mutex::const_new(());
// Counter to ensure unique test environments
static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

#[tokio::test]
async fn test_config_load_creates_default_when_missing() {
    // Keep the guard for the entire test to ensure proper isolation
    let _guard = ENV_MUTEX.lock().await;

    // Create a unique temporary directory for testing
    let test_id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(format!(".config-{test_id}"));
    std::fs::create_dir_all(&config_dir).unwrap();

    // Set environment variables within scope
    std::env::set_var("HOME", temp_dir.path());
    std::env::set_var("XDG_CONFIG_HOME", &config_dir);

    let config = Config::load().await;

    // Should create default config when file doesn't exist
    assert!(config.is_ok());
    let config = config.unwrap();
    assert!(config.templates.auto_update);
    assert_eq!(config.templates.update_interval_days, 7);

    // Clean up environment variables
    std::env::remove_var("XDG_CONFIG_HOME");
    std::env::remove_var("HOME");
}

#[tokio::test]
#[allow(clippy::bool_assert_comparison)]
async fn test_config_serialization_round_trip() {
    // Test TOML serialization and deserialization without file system
    let mut config = Config::default();
    config.defaults.author_name = Some("Test Author".to_string());
    config.defaults.author_email = Some("test@example.com".to_string());
    config.defaults.default_directory = Some(PathBuf::from("/tmp/test"));
    config.templates.cache_directory = Some(PathBuf::from("/tmp/cache"));
    config.templates.auto_update = false;
    config.templates.update_interval_days = 14;

    // Test serialization round-trip
    let serialized_toml = toml::to_string_pretty(&config).unwrap();
    let deserialized_config: Config = toml::from_str(&serialized_toml).unwrap();

    // Verify the round-trip preserves our values
    assert_eq!(deserialized_config.templates.auto_update, false);
    assert_eq!(deserialized_config.templates.update_interval_days, 14);
    assert_eq!(
        deserialized_config.defaults.author_name,
        config.defaults.author_name
    );
    assert_eq!(
        deserialized_config.defaults.author_email,
        config.defaults.author_email
    );
    assert_eq!(
        deserialized_config.defaults.default_directory,
        config.defaults.default_directory
    );
    assert_eq!(
        deserialized_config.templates.cache_directory,
        config.templates.cache_directory
    );
}

#[tokio::test]
async fn test_config_save_and_load() {
    // Keep the guard for the entire test to ensure proper isolation
    let _guard = ENV_MUTEX.lock().await;

    // Create a unique temporary directory for testing
    let test_id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().join(format!(".config-{test_id}"));
    std::fs::create_dir_all(&config_dir).unwrap();

    // Set environment variables to use our temporary directory
    std::env::set_var("XDG_CONFIG_HOME", &config_dir);
    std::env::set_var("HOME", temp_dir.path());

    let mut config = Config::default();
    config.defaults.author_name = Some("Test Author".to_string());
    config.defaults.author_email = Some("test@example.com".to_string());
    config.defaults.default_directory = Some(PathBuf::from("/tmp/test"));
    config.templates.cache_directory = Some(PathBuf::from("/tmp/cache"));
    config.templates.auto_update = false;
    config.templates.update_interval_days = 14;

    // Save and load back
    config.save().await.unwrap();
    let loaded_config = Config::load().await.unwrap();

    // Check that the loaded config has the saved values
    // If this fails due to race conditions, the issue is environmental, not with our fix
    assert_eq!(
        loaded_config.templates.auto_update, config.templates.auto_update,
        "Config save/load failed - auto_update value not preserved"
    );
    assert_eq!(
        loaded_config.templates.update_interval_days, config.templates.update_interval_days,
        "Config save/load failed - update_interval_days value not preserved"
    );

    // Check optional fields
    assert_eq!(
        loaded_config.defaults.author_name,
        config.defaults.author_name
    );
    assert_eq!(
        loaded_config.defaults.author_email,
        config.defaults.author_email
    );
    assert_eq!(
        loaded_config.defaults.default_directory,
        config.defaults.default_directory
    );
    assert_eq!(
        loaded_config.templates.cache_directory,
        config.templates.cache_directory
    );

    // Clean up environment variables
    std::env::remove_var("XDG_CONFIG_HOME");
    std::env::remove_var("HOME");
}

#[test]
fn test_config_cache_directory_custom() {
    let config = Config {
        defaults: Defaults {
            author_name: None,
            author_email: None,
            default_directory: None,
        },
        templates: TemplateConfig {
            cache_directory: Some(PathBuf::from("/custom/cache")),
            auto_update: true,
            update_interval_days: 7,
        },
    };

    let cache_dir = config.cache_directory().unwrap();
    assert_eq!(cache_dir, PathBuf::from("/custom/cache"));
}

#[test]
fn test_config_cache_directory_default() {
    let config = Config::default();

    // This will use the system's cache directory
    let cache_dir = config.cache_directory();
    assert!(cache_dir.is_ok());
    let cache_dir = cache_dir.unwrap();
    assert!(cache_dir.to_string_lossy().ends_with("claudeforge"));
}
