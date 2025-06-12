// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Tests for the init command behavior

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

#[test]
fn init_creates_gitmind_directory_in_git_repo() {
    // Given: A git repository
    let temp_dir = TempDir::new().unwrap();
    Command::new("git")
        .current_dir(&temp_dir)
        .args(&["init"])
        .assert()
        .success();

    // When: Running gitmind init
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialized gitmind"));

    // Then: .gitmind/links/ directory exists
    assert!(temp_dir.path().join(".gitmind").exists());
    assert!(temp_dir.path().join(".gitmind/links").exists());
}

#[test]
fn init_fails_outside_git_repository() {
    // Given: A directory that is not a git repository
    let temp_dir = TempDir::new().unwrap();

    // When: Running gitmind init
    // Then: It fails with appropriate error message
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .arg("init")
        .assert()
        .failure()
        .stderr(predicate::str::contains("not a git repository"));
}

#[test]
fn init_fails_if_already_initialized() {
    // Given: A git repository with gitmind already initialized
    let temp_dir = TempDir::new().unwrap();
    Command::new("git")
        .current_dir(&temp_dir)
        .args(&["init"])
        .assert()
        .success();

    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .arg("init")
        .assert()
        .success();

    // When: Running gitmind init again
    // Then: It fails with appropriate error message
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .arg("init")
        .assert()
        .failure()
        .stderr(predicate::str::contains("already initialized"));
}
