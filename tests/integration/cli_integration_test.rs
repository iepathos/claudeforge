use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_cli_version_output() {
    let mut cmd = Command::cargo_bin("claudeforge").unwrap();
    cmd.arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("claudeforge"))
        .stdout(predicate::str::contains("Create new projects optimized for Claude Code"))
        .stdout(predicate::str::contains("Repository:"))
        .stdout(predicate::str::contains("Authors:"));
}

#[test]
fn test_cli_help_output() {
    let mut cmd = Command::cargo_bin("claudeforge").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Create new projects optimized for Claude Code"))
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("Commands:"));
}

#[test]
fn test_cli_list_templates() {
    let mut cmd = Command::cargo_bin("claudeforge").unwrap();
    cmd.arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Available templates"));
}

#[test]
fn test_cli_update_command() {
    // Use a temporary directory for cache to ensure clean state
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("claudeforge").unwrap();
    
    // Set XDG_CACHE_HOME to temporary directory to isolate the test
    cmd.env("XDG_CACHE_HOME", temp_dir.path())
        .arg("update")
        .assert()
        .success();
}

#[test]
fn test_cli_new_without_arguments() {
    let mut cmd = Command::cargo_bin("claudeforge").unwrap();
    cmd.arg("new")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_cli_new_with_invalid_template() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("claudeforge").unwrap();
    
    cmd.arg("new")
        .arg("invalid-template")
        .arg("test-project")
        .arg("--directory")
        .arg(temp_dir.path())
        .arg("--yes")
        .assert()
        .failure();
}

#[test]
fn test_cli_new_rust_project() {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("claudeforge").unwrap();
    
    cmd.arg("new")
        .arg("rust")
        .arg("test-rust-project")
        .arg("--directory")
        .arg(temp_dir.path())
        .arg("--yes")
        .assert()
        .success()
        .stdout(predicate::str::contains("Project 'test-rust-project' created successfully"));
}

#[test]
fn test_cli_new_project_existing_directory() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("existing-project");
    std::fs::create_dir_all(&project_dir).unwrap();
    
    let mut cmd = Command::cargo_bin("claudeforge").unwrap();
    cmd.arg("new")
        .arg("rust")
        .arg("existing-project")
        .arg("--directory")
        .arg(temp_dir.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
}

#[test]
fn test_cli_new_project_existing_directory_with_yes() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("existing-project");
    std::fs::create_dir_all(&project_dir).unwrap();
    
    let mut cmd = Command::cargo_bin("claudeforge").unwrap();
    cmd.arg("new")
        .arg("rust")
        .arg("existing-project")
        .arg("--directory")
        .arg(temp_dir.path())
        .arg("--yes")
        .assert()
        .success()
        .stdout(predicate::str::contains("overwriting due to --yes flag"));
}

#[test]
fn test_cli_invalid_command() {
    let mut cmd = Command::cargo_bin("claudeforge").unwrap();
    cmd.arg("invalid-command")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand 'invalid-command'"));
}

#[test]
fn test_cli_version_short_flag() {
    let mut cmd = Command::cargo_bin("claudeforge").unwrap();
    cmd.arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains("claudeforge"));
}

#[test]
fn test_cli_help_short_flag() {
    let mut cmd = Command::cargo_bin("claudeforge").unwrap();
    cmd.arg("-h")
        .assert()
        .success()
        .stdout(predicate::str::contains("Create new projects optimized for Claude Code"));
}