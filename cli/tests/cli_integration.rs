// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn test_init_creates_gitmind_directory() {
    // Create isolated temp directory for test
    let temp_dir = TempDir::new().unwrap();

    // Initialize git repo in temp dir, NOT in working repo
    Command::new("git")
        .current_dir(&temp_dir)
        .args(&["init"])
        .assert()
        .success();

    // Run gitmind init
    let mut cmd = Command::cargo_bin("gitmind").unwrap();
    cmd.current_dir(&temp_dir)
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialized gitmind"));

    // Verify .gitmind/links/ was created
    assert!(temp_dir.path().join(".gitmind").exists());
    assert!(temp_dir.path().join(".gitmind/links").exists());
}

#[test]
fn test_init_fails_without_git_repo() {
    // Create temp directory WITHOUT git repo
    let temp_dir = TempDir::new().unwrap();

    // Run gitmind init - should fail
    let mut cmd = Command::cargo_bin("gitmind").unwrap();
    cmd.current_dir(&temp_dir)
        .arg("init")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not a git repository"));
}
