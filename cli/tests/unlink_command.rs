// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::process::Command as StdCommand;
use tempfile::TempDir;

#[test]
fn test_unlink_removes_specific_link() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize git repo
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["init"])
        .output()
        .unwrap();

    // Configure git user
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.email", "test@example.com"])
        .output()
        .unwrap();
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.name", "Test User"])
        .output()
        .unwrap();

    // Initialize gitmind
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["init"])
        .assert()
        .success();

    // Create test files
    let source = "source.md";
    let target = "target.md";
    fs::write(repo_path.join(source), "# Source").unwrap();
    fs::write(repo_path.join(target), "# Target").unwrap();

    // Create a link
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["link", source, target, "--type", "CROSS_REF"])
        .assert()
        .success();

    // Verify link exists by running list command
    let output = Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["list"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CROSS_REF: source.md -> target.md"));

    // Unlink the files
    let mut cmd = Command::cargo_bin("gitmind").unwrap();
    cmd.current_dir(repo_path)
        .args(["unlink", source, target])
        .assert()
        .success();

    // Verify link is removed
    let output = Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["list"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "list command failed with stderr: {}",
        stderr
    );
    assert!(
        stdout.contains("No links found"),
        "Expected 'No links found' but got: '{}'",
        stdout
    );

    // Verify Git commit was made
    let output = StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["log", "--oneline"])
        .output()
        .unwrap();
    let log = String::from_utf8_lossy(&output.stdout);
    assert!(log.contains("unlink(F016):"));
    assert!(log.contains(&format!("{} -/-> {}", source, target)));
}

#[test]
fn test_unlink_with_type_removes_only_matching_type() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize git repo
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["init"])
        .output()
        .unwrap();

    // Configure git user
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.email", "test@example.com"])
        .output()
        .unwrap();
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.name", "Test User"])
        .output()
        .unwrap();

    // Initialize gitmind
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["init"])
        .assert()
        .success();

    // Create test files
    let source = "source.md";
    let target = "target.md";
    fs::write(repo_path.join(source), "# Source").unwrap();
    fs::write(repo_path.join(target), "# Target").unwrap();

    // Create two links with different types
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["link", source, target, "--type", "CROSS_REF"])
        .assert()
        .success();

    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["link", source, target, "--type", "DEPENDS_ON"])
        .assert()
        .success();

    // Verify both links exist
    let output = Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["list"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CROSS_REF: source.md -> target.md"));
    assert!(stdout.contains("DEPENDS_ON: source.md -> target.md"));

    // Unlink only CROSS_REF type
    let mut cmd = Command::cargo_bin("gitmind").unwrap();
    cmd.current_dir(repo_path)
        .args(["unlink", source, target, "--type", "CROSS_REF"])
        .assert()
        .success();

    // Verify only one link remains
    let output = Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["list"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("CROSS_REF: source.md -> target.md"));
    assert!(stdout.contains("DEPENDS_ON: source.md -> target.md"));
}

#[test]
fn test_unlink_all_from_source() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize git repo
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["init"])
        .output()
        .unwrap();

    // Configure git user
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.email", "test@example.com"])
        .output()
        .unwrap();
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.name", "Test User"])
        .output()
        .unwrap();

    // Initialize gitmind
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["init"])
        .assert()
        .success();

    // Create test files
    let source = "source.md";
    let target1 = "target1.md";
    let target2 = "target2.md";
    fs::write(repo_path.join(source), "# Source").unwrap();
    fs::write(repo_path.join(target1), "# Target 1").unwrap();
    fs::write(repo_path.join(target2), "# Target 2").unwrap();

    // Create multiple links from source
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["link", source, target1, "--type", "CROSS_REF"])
        .assert()
        .success();

    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["link", source, target2, "--type", "DEPENDS_ON"])
        .assert()
        .success();

    // Verify links exist
    let output = Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["list"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CROSS_REF: source.md -> target1.md"));
    assert!(stdout.contains("DEPENDS_ON: source.md -> target2.md"));

    // Unlink all from source
    let mut cmd = Command::cargo_bin("gitmind").unwrap();
    cmd.current_dir(repo_path)
        .args(["unlink", source, "--all"])
        .assert()
        .success();

    // Verify all links from source are removed
    let output = Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["list"])
        .output()
        .unwrap();
    assert!(output.status.success(), "list command should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("No links found"),
        "Expected 'No links found' but got: '{}'",
        stdout
    );
}

#[test]
fn test_unlink_all_to_target() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize git repo
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["init"])
        .output()
        .unwrap();

    // Configure git user
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.email", "test@example.com"])
        .output()
        .unwrap();
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.name", "Test User"])
        .output()
        .unwrap();

    // Initialize gitmind
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["init"])
        .assert()
        .success();

    // Create test files
    let source1 = "source1.md";
    let source2 = "source2.md";
    let target = "target.md";
    fs::write(repo_path.join(source1), "# Source 1").unwrap();
    fs::write(repo_path.join(source2), "# Source 2").unwrap();
    fs::write(repo_path.join(target), "# Target").unwrap();

    // Create multiple links to target
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["link", source1, target, "--type", "CROSS_REF"])
        .assert()
        .success();

    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["link", source2, target, "--type", "DEPENDS_ON"])
        .assert()
        .success();

    // Verify links exist
    let output = Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["list"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CROSS_REF: source1.md -> target.md"));
    assert!(stdout.contains("DEPENDS_ON: source2.md -> target.md"));

    // Unlink all to target
    let mut cmd = Command::cargo_bin("gitmind").unwrap();
    cmd.current_dir(repo_path)
        .args(["unlink", "--to", target])
        .assert()
        .success();

    // Verify all links to target are removed
    let output = Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["list"])
        .output()
        .unwrap();
    assert!(output.status.success(), "list command should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("No links found"),
        "Expected 'No links found' but got: '{}'",
        stdout
    );
}

#[test]
fn test_unlink_non_existent_link_succeeds_with_message() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize git repo
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["init"])
        .output()
        .unwrap();

    // Configure git user
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.email", "test@example.com"])
        .output()
        .unwrap();
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.name", "Test User"])
        .output()
        .unwrap();

    // Initialize gitmind
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&repo_path)
        .args(["init"])
        .assert()
        .success();

    // Create test files but no link
    let source = "source.md";
    let target = "target.md";
    fs::write(repo_path.join(source), "# Source").unwrap();
    fs::write(repo_path.join(target), "# Target").unwrap();

    // Try to unlink non-existent link
    let mut cmd = Command::cargo_bin("gitmind").unwrap();
    cmd.current_dir(repo_path)
        .args(["unlink", source, target])
        .assert()
        .success()
        .stdout(predicate::str::contains("No matching links found"));
}

#[test]
fn test_unlink_requires_gitmind_init() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize only git repo (not gitmind)
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["init"])
        .output()
        .unwrap();

    // Try to unlink
    let mut cmd = Command::cargo_bin("gitmind").unwrap();
    cmd.current_dir(repo_path)
        .args(["unlink", "a.md", "b.md"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("gitmind not initialized"));
}
