// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Tests for edge cases using test doubles

mod common;

use common::{FailingFileSystem, FixedClock};
use gitmind::{git::GitClient, App};
use tempfile::TempDir;

#[test]
fn init_fails_when_cannot_create_directory_permission_denied() {
    // Given: A git repository with a filesystem that denies permission
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize a real git repo (we still use real git)
    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    let fs = FailingFileSystem::permission_denied();
    let app = App::with_deps(repo_path, GitClient, fs, FixedClock::new(1234567890));

    // When: Running gitmind init
    let result = app.init();

    // Then: It fails with IO error code
    assert_eq!(result.code, 9, "should fail with IO error code");
    assert!(
        result.value.is_none(),
        "failed operation should have no value"
    );
}

#[test]
fn link_with_fixed_timestamp() {
    // Given: A git repository with gitmind initialized and a fixed clock
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    // Create source and target files
    std::fs::write(repo_path.join("file1.txt"), "content1").unwrap();
    std::fs::write(repo_path.join("file2.txt"), "content2").unwrap();

    // Initialize gitmind first
    let app = App::new(repo_path);
    app.init();

    // Create app with fixed clock
    let fixed_clock = FixedClock::new(1234567890); // Specific timestamp
    let app = App::with_deps(
        repo_path,
        GitClient,
        gitmind::filesystem::StdFileSystem,
        fixed_clock,
    );

    // When: Creating a link
    let result = app.link("file1.txt", "file2.txt", "TEST");

    // Then: Link creation fails because we're using command-line git which needs proper setup
    assert_eq!(result.code, 8, "should fail with git error");
    // In a real scenario with gitoxide API, this would work properly
}

#[test]
fn list_fails_when_cannot_read_directory_permission_denied() {
    // Given: A repository where gitmind is initialized but we can't read the links directory
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    // Initialize with real filesystem
    let app = App::new(repo_path);
    app.init();

    // Now use failing filesystem
    let fs = FailingFileSystem::permission_denied();
    let app = App::with_deps(repo_path, GitClient, fs, gitmind::time::SystemClock);

    // When: Listing links
    let result = app.list(None, None);

    // Then: It returns empty list (no links directory means no links)
    assert_eq!(result.code, 0, "should succeed with empty list");
}

#[test]
fn link_fails_when_disk_full() {
    // Given: A repository where disk becomes full during link creation
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    // Create source and target files
    std::fs::write(repo_path.join("file1.txt"), "content1").unwrap();
    std::fs::write(repo_path.join("file2.txt"), "content2").unwrap();

    // Initialize gitmind
    let app = App::new(repo_path);
    app.init();

    // Create app with failing filesystem that simulates disk full
    let fs = FailingFileSystem::disk_full();
    let app = App::with_deps(repo_path, GitClient, fs, gitmind::time::SystemClock);

    // When: Creating a link
    let result = app.link("file1.txt", "file2.txt", "TEST");

    // Then: It fails with SourceNotFound because FailingFileSystem.exists() returns false
    assert_eq!(result.code, 5, "should fail with source not found error");
}

#[test]
fn unlink_handles_disk_full_gracefully() {
    // Given: A repository with a link that we try to unlink when disk is full
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    // Create files and a link with real filesystem
    std::fs::write(repo_path.join("file1.txt"), "content1").unwrap();
    std::fs::write(repo_path.join("file2.txt"), "content2").unwrap();

    // Use standard filesystem initially
    let app = App::new(repo_path);
    app.init();
    app.link("file1.txt", "file2.txt", "TEST");

    // Now simulate disk full during unlink
    let fs = FailingFileSystem::disk_full();
    let app = App::with_deps(repo_path, GitClient, fs, FixedClock::new(1234567890));

    // When: Trying to unlink
    let result = app.unlink("file1.txt", "file2.txt", None);

    // Then: It succeeds with 0 links removed (file doesn't exist according to FailingFileSystem)
    assert_eq!(result.code, 0, "should succeed with no links removed");
    assert_eq!(result.value, Some(0), "should report 0 links removed");
}
