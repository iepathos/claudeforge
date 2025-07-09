use claudeforge::utils::fs::{copy_dir_recursive, is_dir_empty};
use tempfile::TempDir;
use tokio::fs;

#[tokio::test]
async fn test_is_dir_empty_with_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let empty_dir = temp_dir.path().join("empty");
    fs::create_dir_all(&empty_dir).await.unwrap();

    let result = is_dir_empty(&empty_dir).await.unwrap();
    assert!(result);
}

#[tokio::test]
async fn test_is_dir_empty_with_non_empty_directory() {
    let temp_dir = TempDir::new().unwrap();
    let non_empty_dir = temp_dir.path().join("non_empty");
    fs::create_dir_all(&non_empty_dir).await.unwrap();

    // Create a file in the directory
    fs::write(non_empty_dir.join("file.txt"), "content")
        .await
        .unwrap();

    let result = is_dir_empty(&non_empty_dir).await.unwrap();
    assert!(!result);
}

#[tokio::test]
async fn test_is_dir_empty_with_subdirectory() {
    let temp_dir = TempDir::new().unwrap();
    let dir_with_subdir = temp_dir.path().join("with_subdir");
    fs::create_dir_all(&dir_with_subdir).await.unwrap();

    // Create a subdirectory
    fs::create_dir_all(dir_with_subdir.join("subdir"))
        .await
        .unwrap();

    let result = is_dir_empty(&dir_with_subdir).await.unwrap();
    assert!(!result);
}

#[tokio::test]
async fn test_copy_dir_recursive_handles_symlinks() {
    // Note: This test only runs on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;

        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("src");
        let dst_dir = temp_dir.path().join("dst");

        // Create source directory with a file and a symlink
        fs::create_dir_all(&src_dir).await.unwrap();
        fs::write(src_dir.join("file.txt"), "content")
            .await
            .unwrap();

        // Create a symlink
        symlink("file.txt", src_dir.join("link.txt")).unwrap();

        // Copy directory
        copy_dir_recursive(&src_dir, &dst_dir, None).await.unwrap();

        // Verify both file and symlink were copied
        assert!(dst_dir.join("file.txt").exists());
        assert!(dst_dir.join("link.txt").exists());
    }
}

#[tokio::test]
async fn test_copy_dir_recursive_with_nested_exclusions() {
    let temp_dir = TempDir::new().unwrap();
    let src_dir = temp_dir.path().join("src");
    let dst_dir = temp_dir.path().join("dst");

    // Create source directory with nested structure
    fs::create_dir_all(&src_dir).await.unwrap();
    fs::write(src_dir.join("keep.txt"), "keep").await.unwrap();

    // Create excluded directory with content
    let excluded = src_dir.join("node_modules");
    fs::create_dir_all(&excluded).await.unwrap();
    fs::write(excluded.join("package.json"), "{}")
        .await
        .unwrap();

    // Create another excluded directory
    let git_dir = src_dir.join(".git");
    fs::create_dir_all(&git_dir).await.unwrap();
    fs::write(git_dir.join("config"), "git config")
        .await
        .unwrap();

    // Copy with multiple exclusions
    copy_dir_recursive(&src_dir, &dst_dir, Some(&[".git", "node_modules"]))
        .await
        .unwrap();

    // Verify normal file was copied
    assert!(dst_dir.join("keep.txt").exists());

    // Verify excluded directories were not copied
    assert!(!dst_dir.join(".git").exists());
    assert!(!dst_dir.join("node_modules").exists());
}
