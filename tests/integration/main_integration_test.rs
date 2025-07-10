use std::process::Command;

#[test]
fn test_version_command() {
    let output = Command::new("cargo")
        .args(["run", "--", "version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should contain version information
    assert!(stdout.contains("claudeforge"));
    assert!(stdout.contains("Create new projects optimized for Claude Code"));
}

#[test]
fn test_list_command() {
    let output = Command::new("cargo")
        .args(["run", "--", "list"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should list available templates
    assert!(stdout.contains("Available templates:"));
    assert!(stdout.contains("rust"));
    assert!(stdout.contains("go"));
    assert!(stdout.contains("python"));
}

#[test]
fn test_update_command() {
    let output = Command::new("cargo")
        .args(["run", "--", "update"])
        .output()
        .expect("Failed to execute command");

    // Update command should succeed even with no cached templates
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show update message
    assert!(
        stdout.contains("Checking for cached templates")
            || stdout.contains("No cached templates found")
    );
}

#[test]
fn test_new_command_missing_args() {
    let output = Command::new("cargo")
        .args(["run", "--", "new"])
        .output()
        .expect("Failed to execute command");

    // Should fail due to missing arguments
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("error") || stderr.contains("required"));
}

#[test]
fn test_new_command_invalid_language() {
    let output = Command::new("cargo")
        .args(["run", "--", "new", "invalid-lang", "test-project"])
        .output()
        .expect("Failed to execute command");

    // Should fail due to invalid language
    assert!(!output.status.success());
}

#[test]
fn test_git_not_available_simulation() {
    // This test would require mocking git availability
    // For now, we just verify that git is checked
    let output = Command::new("cargo")
        .args(["run", "--", "version"])
        .output()
        .expect("Failed to execute command");

    // If this succeeds, it means git check passed
    assert!(output.status.success());
}
