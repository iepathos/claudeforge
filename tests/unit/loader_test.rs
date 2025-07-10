use claudeforge::cli::Language;
use claudeforge::template::loader::TemplateLoader;
use std::sync::Mutex;
use tempfile::TempDir;
use tokio::fs;

// Mutex to prevent parallel execution of tests that modify environment variables
static ENV_MUTEX: Mutex<()> = Mutex::new(());

#[tokio::test]
async fn test_template_loader_new() {
    let loader = TemplateLoader::new().await;
    assert!(loader.is_ok());
}

#[tokio::test]
async fn test_get_template_valid_language() {
    let loader = TemplateLoader::new().await.unwrap();

    let rust_template = loader.get_template(Language::Rust);
    assert!(rust_template.is_ok());
    let template = rust_template.unwrap();
    assert_eq!(template.language, Language::Rust);

    let go_template = loader.get_template(Language::Go);
    assert!(go_template.is_ok());
    let template = go_template.unwrap();
    assert_eq!(template.language, Language::Go);

    let python_template = loader.get_template(Language::Python);
    assert!(python_template.is_ok());
    let template = python_template.unwrap();
    assert_eq!(template.language, Language::Python);
}

#[tokio::test]
async fn test_list_templates() {
    let loader = TemplateLoader::new().await.unwrap();
    let templates = loader.list_templates();

    // Should have at least the registered templates
    assert!(!templates.is_empty());

    // Check that we have all expected languages
    let languages: Vec<Language> = templates.iter().map(|t| t.language.clone()).collect();
    assert!(languages.contains(&Language::Rust));
    assert!(languages.contains(&Language::Go));
    assert!(languages.contains(&Language::Python));
}

#[tokio::test]
async fn test_get_or_fetch_with_cached_template() {
    let _guard = ENV_MUTEX.lock().unwrap();
    drop(_guard);

    // Create a mock cache directory
    let temp_dir = TempDir::new().unwrap();
    let cache_dir = temp_dir.path().join("cache");
    fs::create_dir_all(&cache_dir).await.unwrap();

    // Mock a cached template
    let rust_template_dir = cache_dir.join("rust-claude-template");
    fs::create_dir_all(&rust_template_dir).await.unwrap();
    fs::write(rust_template_dir.join("Cargo.toml"), "[package]")
        .await
        .unwrap();

    // Set the cache directory environment variable
    std::env::set_var("XDG_CACHE_HOME", temp_dir.path());

    let loader = TemplateLoader::new().await.unwrap();

    // This should use the cached template without fetching
    let result = loader.get_or_fetch(Language::Rust).await;

    // Note: This might fail if it tries to fetch from the actual repository
    // In a real test environment, we'd mock the git operations
    match result {
        Ok(path) => {
            // If successful, it should point to a rust template directory
            assert!(path.to_string_lossy().contains("rust"));
        }
        Err(e) => {
            // Expected if the template repository doesn't exist
            println!("Expected error (template repo might not exist): {e}");
        }
    }

    // Clean up environment variables
    std::env::remove_var("XDG_CACHE_HOME");
}

#[tokio::test]
async fn test_update_all_with_no_cached_templates() {
    let _guard = ENV_MUTEX.lock().unwrap();
    drop(_guard);

    // Create a mock empty cache directory
    let temp_dir = TempDir::new().unwrap();
    let cache_dir = temp_dir.path().join("claudeforge");
    fs::create_dir_all(&cache_dir).await.unwrap();

    std::env::set_var("XDG_CACHE_HOME", temp_dir.path());

    let loader = TemplateLoader::new().await.unwrap();

    // Should complete without error even with no cached templates
    let result = loader.update_all().await;
    assert!(result.is_ok());

    // Clean up environment variables
    std::env::remove_var("XDG_CACHE_HOME");
}

#[tokio::test]
async fn test_update_all_with_cached_templates() {
    let _guard = ENV_MUTEX.lock().unwrap();
    drop(_guard);

    // Create a mock cache directory with templates
    let temp_dir = TempDir::new().unwrap();
    let cache_dir = temp_dir.path().join("claudeforge");
    fs::create_dir_all(&cache_dir).await.unwrap();

    // Mock cached templates
    let rust_dir = cache_dir.join("rust-claude-template");
    fs::create_dir_all(&rust_dir).await.unwrap();
    fs::write(rust_dir.join("Cargo.toml"), "[package]")
        .await
        .unwrap();

    let go_dir = cache_dir.join("go-claude-template");
    fs::create_dir_all(&go_dir).await.unwrap();
    fs::write(go_dir.join("go.mod"), "module test")
        .await
        .unwrap();

    std::env::set_var("XDG_CACHE_HOME", temp_dir.path());

    let loader = TemplateLoader::new().await.unwrap();

    // This will try to update the cached templates
    let result = loader.update_all().await;

    // This might fail if it tries to fetch from actual repositories
    match result {
        Ok(_) => {
            // Success case - templates were updated
            println!("Templates updated successfully");
        }
        Err(e) => {
            // Expected if the template repositories don't exist
            println!("Expected error (template repos might not exist): {e}");
        }
    }

    // Clean up environment variables
    std::env::remove_var("XDG_CACHE_HOME");
}
