// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Tests for the init command behavior

use gitmind::App;
use std::process::Command as StdCommand;
use tempfile::TempDir;

#[test]
fn init_creates_gitmind_directory_in_git_repo() {
    // Given: A git repository
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    // When: Running gitmind init
    let app = App::new(repo_path);
    let result = app.init();

    // Then: Command succeeds and directory exists
    assert_eq!(result.code, 0, "init should succeed");
    assert!(result.value.is_some(), "init should return a value");
    assert!(temp_dir.path().join(".gitmind").exists());
    assert!(temp_dir.path().join(".gitmind/links").exists());
}

#[test]
fn init_fails_outside_git_repository() {
    // Given: A directory that is not a git repository
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // When: Running gitmind init
    let app = App::new(repo_path);
    let result = app.init();

    // Then: It fails with NotAGitRepository exit code
    assert_eq!(
        result.code, 2,
        "init should fail with NotAGitRepository code"
    );
    assert_eq!(result.value, None, "failed operation should have no value");
}

#[test]
fn init_fails_if_already_initialized() {
    // Given: A git repository with gitmind already initialized
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    let app = App::new(repo_path);
    let result = app.init();
    assert_eq!(result.code, 0, "first init should succeed");

    // When: Running gitmind init again
    let result = app.init();

    // Then: It fails with AlreadyInitialized exit code
    assert_eq!(
        result.code, 4,
        "second init should fail with AlreadyInitialized code"
    );
    assert_eq!(result.value, None, "failed operation should have no value");
}
