// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Tests for the list command behavior

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper to set up a git repo with gitmind initialized
fn setup_gitmind_repo() -> TempDir {
    let temp_dir = TempDir::new().unwrap();

    // Initialize git
    Command::new("git")
        .current_dir(&temp_dir)
        .args(&["init"])
        .assert()
        .success();

    // Configure git user
    Command::new("git")
        .current_dir(&temp_dir)
        .args(&["config", "user.name", "Test User"])
        .assert()
        .success();

    Command::new("git")
        .current_dir(&temp_dir)
        .args(&["config", "user.email", "test@example.com"])
        .assert()
        .success();

    // Initialize gitmind
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .arg("init")
        .assert()
        .success();

    temp_dir
}

#[test]
fn list_shows_all_links_in_repository() {
    // Given: A gitmind repo with multiple links
    let temp_dir = setup_gitmind_repo();

    // Create files
    fs::write(temp_dir.path().join("file1.md"), "# File 1").unwrap();
    fs::write(temp_dir.path().join("file2.md"), "# File 2").unwrap();
    fs::write(temp_dir.path().join("file3.md"), "# File 3").unwrap();

    Command::new("git")
        .current_dir(&temp_dir)
        .args(&["add", "."])
        .assert()
        .success();

    Command::new("git")
        .current_dir(&temp_dir)
        .args(&["commit", "-m", "Add files"])
        .assert()
        .success();

    // Create some links
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .args(&["link", "file1.md", "file2.md"])
        .assert()
        .success();

    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .args(&["link", "file2.md", "file3.md", "--type", "DEPENDS_ON"])
        .assert()
        .success();

    // When: Running gitmind list
    let output = Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .arg("list")
        .assert()
        .success();

    // Then: All links are displayed
    output
        .stdout(predicate::str::contains("file1.md -> file2.md"))
        .stdout(predicate::str::contains("CROSS_REF"))
        .stdout(predicate::str::contains("file2.md -> file3.md"))
        .stdout(predicate::str::contains("DEPENDS_ON"));
}

#[test]
fn list_shows_empty_when_no_links() {
    // Given: A gitmind repo with no links
    let temp_dir = setup_gitmind_repo();

    // When: Running gitmind list
    let output = Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .arg("list")
        .assert()
        .success();

    // Then: It shows no links message
    output.stdout(predicate::str::contains("No links found"));
}

#[test]
fn list_shows_link_timestamps() {
    // Given: A gitmind repo with a link
    let temp_dir = setup_gitmind_repo();

    fs::write(temp_dir.path().join("a.md"), "# A").unwrap();
    fs::write(temp_dir.path().join("b.md"), "# B").unwrap();

    Command::new("git")
        .current_dir(&temp_dir)
        .args(&["add", "."])
        .assert()
        .success();

    Command::new("git")
        .current_dir(&temp_dir)
        .args(&["commit", "-m", "Add files"])
        .assert()
        .success();

    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .args(&["link", "a.md", "b.md"])
        .assert()
        .success();

    // When: Running gitmind list
    let output = Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .arg("list")
        .assert()
        .success();

    // Then: Link includes timestamp
    output.stdout(predicate::str::contains("ts:"));
}

#[test]
fn list_filters_by_source_file() {
    // Given: A gitmind repo with multiple links
    let temp_dir = setup_gitmind_repo();

    // Create files
    fs::write(temp_dir.path().join("a.md"), "# A").unwrap();
    fs::write(temp_dir.path().join("b.md"), "# B").unwrap();
    fs::write(temp_dir.path().join("c.md"), "# C").unwrap();

    Command::new("git")
        .current_dir(&temp_dir)
        .args(&["add", "."])
        .assert()
        .success();

    Command::new("git")
        .current_dir(&temp_dir)
        .args(&["commit", "-m", "Add files"])
        .assert()
        .success();

    // Create links from different sources
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .args(&["link", "a.md", "b.md"])
        .assert()
        .success();

    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .args(&["link", "b.md", "c.md"])
        .assert()
        .success();

    // When: Filtering by source
    let output = Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .args(&["list", "--source", "a.md"])
        .assert()
        .success();

    // Then: Only links from that source are shown
    output
        .stdout(predicate::str::contains("a.md -> b.md"))
        .stdout(predicate::str::contains("b.md -> c.md").not());
}

#[test]
fn list_filters_by_target_file() {
    // Given: A gitmind repo with multiple links
    let temp_dir = setup_gitmind_repo();

    // Create files
    fs::write(temp_dir.path().join("a.md"), "# A").unwrap();
    fs::write(temp_dir.path().join("b.md"), "# B").unwrap();
    fs::write(temp_dir.path().join("c.md"), "# C").unwrap();

    Command::new("git")
        .current_dir(&temp_dir)
        .args(&["add", "."])
        .assert()
        .success();

    Command::new("git")
        .current_dir(&temp_dir)
        .args(&["commit", "-m", "Add files"])
        .assert()
        .success();

    // Create links to different targets
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .args(&["link", "a.md", "b.md"])
        .assert()
        .success();

    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .args(&["link", "a.md", "c.md"])
        .assert()
        .success();

    // When: Filtering by target
    let output = Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .args(&["list", "--target", "b.md"])
        .assert()
        .success();

    // Then: Only links to that target are shown
    output
        .stdout(predicate::str::contains("a.md -> b.md"))
        .stdout(predicate::str::contains("a.md -> c.md").not());
}

#[test]
fn list_fails_if_gitmind_not_initialized() {
    // Given: A git repo without gitmind initialized
    let temp_dir = TempDir::new().unwrap();
    Command::new("git")
        .current_dir(&temp_dir)
        .args(&["init"])
        .assert()
        .success();

    // When: Running gitmind list
    // Then: It fails with appropriate error
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .arg("list")
        .assert()
        .failure()
        .stderr(predicate::str::contains("gitmind not initialized"));
}
