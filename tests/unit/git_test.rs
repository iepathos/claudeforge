use claudeforge::git::{add_all_and_commit, clone_repository, init_repository, is_git_available};
use tempfile::TempDir;
use std::fs;

#[test]
fn test_git_clone_with_invalid_url() {
    let temp_dir = TempDir::new().unwrap();
    let target = temp_dir.path().join("clone");
    
    // Try to clone from an invalid URL
    let result = clone_repository("https://invalid-url-that-does-not-exist.com/repo.git", &target);
    
    assert!(result.is_err());
}

#[test]
fn test_init_and_commit_workflow() {
    // Only run if git is available
    if !is_git_available() {
        println!("Skipping test - git not available");
        return;
    }
    
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path().join("test-repo");
    fs::create_dir_all(&repo_path).unwrap();
    
    // Initialize repository
    let init_result = init_repository(&repo_path);
    assert!(init_result.is_ok());
    
    // Create a test file
    fs::write(repo_path.join("test.txt"), "test content").unwrap();
    
    // Add and commit
    let commit_result = add_all_and_commit(&repo_path, "Test commit");
    assert!(commit_result.is_ok());
    
    // Verify .git directory exists
    assert!(repo_path.join(".git").exists());
}

#[test]
fn test_add_all_and_commit_empty_repo() {
    if !is_git_available() {
        println!("Skipping test - git not available");
        return;
    }
    
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path().join("empty-repo");
    fs::create_dir_all(&repo_path).unwrap();
    
    // Initialize repository
    init_repository(&repo_path).unwrap();
    
    // Try to commit without any files
    let result = add_all_and_commit(&repo_path, "Empty commit");
    
    // This might fail because there's nothing to commit
    match result {
        Ok(_) => println!("Empty commit succeeded"),
        Err(e) => println!("Expected error for empty commit: {}", e),
    }
}

#[test]
fn test_clone_repository_target_exists() {
    let temp_dir = TempDir::new().unwrap();
    let target = temp_dir.path().join("existing");
    
    // Create the target directory
    fs::create_dir_all(&target).unwrap();
    fs::write(target.join("existing.txt"), "content").unwrap();
    
    // Try to clone to existing directory
    let result = clone_repository("https://github.com/example/repo.git", &target);
    
    // Should fail because target exists
    assert!(result.is_err());
}