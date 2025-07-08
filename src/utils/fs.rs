use anyhow::{Context, Result};
use std::path::Path;
use tokio::fs;

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
}
