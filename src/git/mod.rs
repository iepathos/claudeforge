use anyhow::{Context, Result};
use git2::{Repository, Signature};
use std::path::Path;
use tracing::{debug, info};

/// Clone a repository to a target path
pub fn clone_repository(repo_url: &str, target_path: &Path) -> Result<()> {
    debug!("Cloning repository: {} to {:?}", repo_url, target_path);

    Repository::clone(repo_url, target_path)
        .with_context(|| format!("Failed to clone repository: {repo_url}"))?;

    info!("Successfully cloned repository to {:?}", target_path);
    Ok(())
}

/// Initialize a new git repository
pub fn init_repository(path: &Path) -> Result<()> {
    debug!("Initializing git repository at {:?}", path);

    Repository::init(path)
        .with_context(|| format!("Failed to initialize git repository at {path:?}"))?;

    info!("Successfully initialized git repository at {:?}", path);
    Ok(())
}

/// Add all files and create initial commit
pub fn add_all_and_commit(repo_path: &Path, message: &str) -> Result<()> {
    let repo = Repository::open(repo_path)
        .with_context(|| format!("Failed to open git repository at {repo_path:?}"))?;

    let mut index = repo.index()?;

    // Add all files to index
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    // Create tree from index
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    // Get default signature
    let signature = get_signature(&repo)?;

    // Create initial commit
    repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[])?;

    info!("Successfully created initial commit: {}", message);
    Ok(())
}

/// Get git signature from config or use default
fn get_signature(repo: &Repository) -> Result<Signature> {
    let config = repo.config()?;

    let name = config
        .get_string("user.name")
        .unwrap_or_else(|_| "ClaudeForge User".to_string());

    let email = config
        .get_string("user.email")
        .unwrap_or_else(|_| "user@example.com".to_string());

    Ok(Signature::now(&name, &email)?)
}

/// Check if git is available on the system
pub fn is_git_available() -> bool {
    std::process::Command::new("git")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_git_available() {
        // This test depends on git being installed
        assert!(is_git_available());
    }

    #[test]
    fn test_init_repository() {
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();

        init_repository(repo_path).unwrap();

        assert!(repo_path.join(".git").exists());
    }
}
