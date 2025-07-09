use anyhow::{Context, Result};
use std::path::Path;
use tokio::fs;

#[cfg(windows)]
use tokio::time::{sleep, Duration};

/// Recursively copy a directory, optionally excluding certain directories
pub async fn copy_dir_recursive(src: &Path, dst: &Path, exclude: Option<&[&str]>) -> Result<()> {
    Box::pin(copy_dir_recursive_inner(src, dst, exclude)).await
}

async fn copy_dir_recursive_inner(src: &Path, dst: &Path, exclude: Option<&[&str]>) -> Result<()> {
    let exclude_set = exclude.unwrap_or(&[]);

    if !dst.exists() {
        fs::create_dir_all(dst)
            .await
            .with_context(|| format!("Failed to create directory: {dst:?}"))?;
    }

    let mut entries = fs::read_dir(src)
        .await
        .with_context(|| format!("Failed to read directory: {src:?}"))?;

    while let Some(entry) = entries.next_entry().await? {
        let entry_path = entry.path();
        let entry_name = entry.file_name();
        let entry_name_str = entry_name.to_string_lossy();

        // Skip excluded directories
        if exclude_set.contains(&entry_name_str.as_ref()) {
            continue;
        }

        let dst_path = dst.join(&entry_name);

        if entry_path.is_dir() {
            Box::pin(copy_dir_recursive_inner(&entry_path, &dst_path, exclude)).await?;
        } else {
            fs::copy(&entry_path, &dst_path)
                .await
                .with_context(|| format!("Failed to copy file: {entry_path:?} to {dst_path:?}"))?;
        }
    }

    Ok(())
}

/// Check if a directory is empty
pub async fn is_dir_empty(path: &Path) -> Result<bool> {
    let mut entries = fs::read_dir(path).await?;
    Ok(entries.next_entry().await?.is_none())
}

/// Robustly remove a directory, handling Windows permission issues
pub async fn remove_dir_all_robust(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    // On Windows, try to handle permission issues with retries
    #[cfg(windows)]
    {
        const MAX_RETRIES: u32 = 5;
        const RETRY_DELAY: Duration = Duration::from_millis(100);

        // Try to remove read-only attributes before removal
        if let Err(_) = remove_readonly_attributes(path).await {
            // Ignore errors during attribute removal
        }

        let mut last_error = None;
        for attempt in 0..MAX_RETRIES {
            match fs::remove_dir_all(path).await {
                Ok(()) => return Ok(()),
                Err(e) => {
                    last_error = Some(e);

                    // Check if this is a Windows permission error
                    if let Some(os_error) = last_error.as_ref().unwrap().raw_os_error() {
                        if os_error == 5 {
                            // ERROR_ACCESS_DENIED
                            if attempt < MAX_RETRIES - 1 {
                                // Wait a bit and retry with exponential backoff
                                sleep(RETRY_DELAY * (attempt + 1)).await;
                                continue;
                            }
                        }
                    }

                    // For other errors on Windows, retry anyway
                    if attempt < MAX_RETRIES - 1 {
                        sleep(RETRY_DELAY * (attempt + 1)).await;
                    }
                }
            }
        }

        Err(anyhow::Error::from(last_error.unwrap())).with_context(|| {
            format!(
                "Failed to remove directory after {} attempts: {path:?}",
                MAX_RETRIES
            )
        })
    }

    // On non-Windows platforms, just use the standard removal
    #[cfg(not(windows))]
    {
        fs::remove_dir_all(path)
            .await
            .with_context(|| format!("Failed to remove directory: {path:?}"))
    }
}

#[cfg(windows)]
async fn remove_readonly_attributes(path: &Path) -> Result<()> {
    use std::process::Command;

    // Convert path to owned string to avoid lifetime issues
    let path_str = format!("{}\\*", path.display());

    // Use attrib command to remove read-only attributes recursively
    let output = tokio::task::spawn_blocking(move || {
        Command::new("attrib")
            .args(["-R", &path_str, "/S"])
            .output()
    })
    .await?;

    match output {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_copy_dir_recursive() {
        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("src");
        let dst_dir = temp_dir.path().join("dst");

        // Create source directory with files
        fs::create_dir_all(&src_dir).await.unwrap();
        fs::write(src_dir.join("file1.txt"), "content1")
            .await
            .unwrap();
        fs::write(src_dir.join("file2.txt"), "content2")
            .await
            .unwrap();

        // Create subdirectory
        let sub_dir = src_dir.join("sub");
        fs::create_dir_all(&sub_dir).await.unwrap();
        fs::write(sub_dir.join("file3.txt"), "content3")
            .await
            .unwrap();

        // Copy directory
        copy_dir_recursive(&src_dir, &dst_dir, None).await.unwrap();

        // Verify files were copied
        assert!(dst_dir.join("file1.txt").exists());
        assert!(dst_dir.join("file2.txt").exists());
        assert!(dst_dir.join("sub").join("file3.txt").exists());

        // Verify content
        let content = fs::read_to_string(dst_dir.join("file1.txt")).await.unwrap();
        assert_eq!(content, "content1");
    }

    #[tokio::test]
    async fn test_copy_dir_with_exclusions() {
        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("src");
        let dst_dir = temp_dir.path().join("dst");

        // Create source directory with files
        fs::create_dir_all(&src_dir).await.unwrap();
        fs::write(src_dir.join("file1.txt"), "content1")
            .await
            .unwrap();

        // Create excluded directory
        let excluded_dir = src_dir.join(".git");
        fs::create_dir_all(&excluded_dir).await.unwrap();
        fs::write(excluded_dir.join("config"), "git config")
            .await
            .unwrap();

        // Copy directory with exclusions
        copy_dir_recursive(&src_dir, &dst_dir, Some(&[".git"]))
            .await
            .unwrap();

        // Verify normal file was copied
        assert!(dst_dir.join("file1.txt").exists());

        // Verify excluded directory was not copied
        assert!(!dst_dir.join(".git").exists());
    }

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
        fs::write(non_empty_dir.join("file.txt"), "content")
            .await
            .unwrap();

        let result = is_dir_empty(&non_empty_dir).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_is_dir_empty_with_nonexistent_directory() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent_dir = temp_dir.path().join("nonexistent");

        let result = is_dir_empty(&nonexistent_dir).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_copy_dir_recursive_with_nested_directories() {
        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("src");
        let dst_dir = temp_dir.path().join("dst");

        // Create nested directory structure
        let nested_dir = src_dir.join("level1").join("level2");
        fs::create_dir_all(&nested_dir).await.unwrap();
        fs::write(nested_dir.join("deep_file.txt"), "deep content")
            .await
            .unwrap();

        // Copy directory
        copy_dir_recursive(&src_dir, &dst_dir, None).await.unwrap();

        // Verify nested structure was copied
        assert!(dst_dir
            .join("level1")
            .join("level2")
            .join("deep_file.txt")
            .exists());

        let content =
            fs::read_to_string(dst_dir.join("level1").join("level2").join("deep_file.txt"))
                .await
                .unwrap();
        assert_eq!(content, "deep content");
    }

    #[tokio::test]
    async fn test_copy_dir_recursive_destination_exists() {
        let temp_dir = TempDir::new().unwrap();
        let src_dir = temp_dir.path().join("src");
        let dst_dir = temp_dir.path().join("dst");

        // Create source directory with files
        fs::create_dir_all(&src_dir).await.unwrap();
        fs::write(src_dir.join("file1.txt"), "content1")
            .await
            .unwrap();

        // Create destination directory
        fs::create_dir_all(&dst_dir).await.unwrap();
        fs::write(dst_dir.join("existing_file.txt"), "existing")
            .await
            .unwrap();

        // Copy directory to existing destination
        copy_dir_recursive(&src_dir, &dst_dir, None).await.unwrap();

        // Verify both files exist
        assert!(dst_dir.join("file1.txt").exists());
        assert!(dst_dir.join("existing_file.txt").exists());
    }

    #[tokio::test]
    async fn test_remove_dir_all_robust() {
        let temp_dir = TempDir::new().unwrap();
        let test_dir = temp_dir.path().join("test_remove");

        // Create a directory with files
        fs::create_dir_all(&test_dir).await.unwrap();
        fs::write(test_dir.join("file1.txt"), "content1")
            .await
            .unwrap();

        let sub_dir = test_dir.join("subdir");
        fs::create_dir_all(&sub_dir).await.unwrap();
        fs::write(sub_dir.join("file2.txt"), "content2")
            .await
            .unwrap();

        // Verify directory exists
        assert!(test_dir.exists());

        // Remove directory robustly
        remove_dir_all_robust(&test_dir).await.unwrap();

        // Verify directory is gone
        assert!(!test_dir.exists());
    }

    #[tokio::test]
    async fn test_remove_dir_all_robust_nonexistent() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent_dir = temp_dir.path().join("nonexistent");

        // Should not error when removing nonexistent directory
        remove_dir_all_robust(&nonexistent_dir).await.unwrap();
    }
}
