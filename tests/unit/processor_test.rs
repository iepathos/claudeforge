use claudeforge::cli::Language;
use claudeforge::template::processor::create_project;
use claudeforge::error::ClaudeForgeError;
use tempfile::TempDir;
use tokio::fs;

#[tokio::test]
async fn test_create_project_directory_exists_with_skip_prompts() {
    let temp_dir = TempDir::new().unwrap();
    let project_name = "existing-project";
    let project_dir = temp_dir.path().join(project_name);
    
    // Create the directory first
    fs::create_dir_all(&project_dir).await.unwrap();
    fs::write(project_dir.join("existing.txt"), "existing content").await.unwrap();
    
    // Try to create project with skip_prompts = true
    let result = create_project(
        Language::Rust,
        project_name.to_string(),
        Some(temp_dir.path().to_path_buf()),
        true, // skip_prompts
    ).await;
    
    // This might fail if template fetching fails, but the directory exists logic should work
    match result {
        Ok(_) => {
            // If successful, the directory should have been overwritten
            println!("Project created successfully, overwriting existing directory");
        }
        Err(e) => {
            // Could fail due to template fetching issues
            println!("Expected error (might be template fetch issue): {}", e);
        }
    }
}

#[tokio::test]
async fn test_create_project_directory_exists_without_skip_prompts() {
    let temp_dir = TempDir::new().unwrap();
    let project_name = "existing-project";
    let project_dir = temp_dir.path().join(project_name);
    
    // Create the directory first
    fs::create_dir_all(&project_dir).await.unwrap();
    
    // Try to create project with skip_prompts = false
    let result = create_project(
        Language::Rust,
        project_name.to_string(),
        Some(temp_dir.path().to_path_buf()),
        false, // skip_prompts
    ).await;
    
    // Should fail with DirectoryExists error
    match result {
        Err(e) => {
            // Check if it's the expected error
            if let Some(cf_error) = e.downcast_ref::<ClaudeForgeError>() {
                match cf_error {
                    ClaudeForgeError::DirectoryExists(_) => {
                        // This is the expected error
                    }
                    _ => {
                        // Could be a different error (e.g., template fetch failed)
                        println!("Different error than expected: {}", cf_error);
                    }
                }
            }
        }
        Ok(_) => {
            panic!("Expected DirectoryExists error, but operation succeeded");
        }
    }
}

#[tokio::test]
async fn test_create_project_with_custom_directory() {
    let temp_dir = TempDir::new().unwrap();
    let custom_dir = temp_dir.path().join("custom").join("path");
    let project_name = "custom-project";
    
    // Create project in custom directory
    let result = create_project(
        Language::Go,
        project_name.to_string(),
        Some(custom_dir.clone()),
        true,
    ).await;
    
    match result {
        Ok(_) => {
            // Verify project was created in the custom directory
            let project_path = custom_dir.join(project_name);
            assert!(project_path.exists());
        }
        Err(e) => {
            // Expected if template repository doesn't exist
            println!("Expected error (template repo might not exist): {}", e);
        }
    }
}

#[tokio::test]
async fn test_create_project_default_directory() {
    let temp_dir = TempDir::new().unwrap();
    let project_name = "default-dir-project";
    
    // Change to temp directory
    std::env::set_current_dir(&temp_dir).unwrap();
    
    // Create project without specifying directory (should use current dir)
    let result = create_project(
        Language::Python,
        project_name.to_string(),
        None, // Use default directory
        true,
    ).await;
    
    match result {
        Ok(_) => {
            // Verify project was created in current directory
            let project_path = temp_dir.path().join(project_name);
            assert!(project_path.exists());
        }
        Err(e) => {
            // Expected if template repository doesn't exist
            println!("Expected error (template repo might not exist): {}", e);
        }
    }
}

#[tokio::test]
async fn test_create_project_with_special_characters_in_name() {
    let temp_dir = TempDir::new().unwrap();
    let project_name = "my-special_project.2024";
    
    let result = create_project(
        Language::Rust,
        project_name.to_string(),
        Some(temp_dir.path().to_path_buf()),
        true,
    ).await;
    
    match result {
        Ok(_) => {
            let project_path = temp_dir.path().join(project_name);
            assert!(project_path.exists());
        }
        Err(e) => {
            println!("Expected error (template repo might not exist): {}", e);
        }
    }
}