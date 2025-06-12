// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Tests for the link command behavior

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
fn link_creates_semantic_link_between_files() {
    // Given: A gitmind repo with two files
    let temp_dir = setup_gitmind_repo();
    fs::write(temp_dir.path().join("notes.md"), "# Notes").unwrap();
    fs::write(temp_dir.path().join("ideas.md"), "# Ideas").unwrap();

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

    // When: Creating a link between them
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .args(&["link", "notes.md", "ideas.md"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Created link"));

    // Then: A link file is created with correct content
    let links_dir = temp_dir.path().join(".gitmind/links");
    let entries: Vec<_> = fs::read_dir(&links_dir).unwrap().collect();
    assert_eq!(entries.len(), 1);

    let link_file = entries[0].as_ref().unwrap().path();
    let content = fs::read_to_string(&link_file).unwrap();
    assert!(content.contains("CROSS_REF: notes.md -> ideas.md"));
}

#[test]
fn link_supports_custom_link_types() {
    // Given: A gitmind repo with spec and implementation files
    let temp_dir = setup_gitmind_repo();
    fs::write(temp_dir.path().join("spec.md"), "# Spec").unwrap();
    fs::write(temp_dir.path().join("impl.rs"), "// Code").unwrap();

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

    // When: Creating a link with custom type
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .args(&["link", "spec.md", "impl.rs", "--type", "IMPLEMENTS"])
        .assert()
        .success();

    // Then: The link file contains the custom type
    let links_dir = temp_dir.path().join(".gitmind/links");
    let entries: Vec<_> = fs::read_dir(&links_dir).unwrap().collect();
    let link_file = entries[0].as_ref().unwrap().path();
    let content = fs::read_to_string(&link_file).unwrap();
    assert!(content.contains("IMPLEMENTS: spec.md -> impl.rs"));
}

#[test]
fn link_fails_when_source_file_missing() {
    // Given: A gitmind repo without the source file
    let temp_dir = setup_gitmind_repo();

    // When: Trying to create a link with missing source
    // Then: It fails with appropriate error
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .args(&["link", "missing.md", "target.md"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Source file not found"));
}

#[test]
fn link_fails_when_target_file_missing() {
    // Given: A gitmind repo with only source file
    let temp_dir = setup_gitmind_repo();
    fs::write(temp_dir.path().join("source.md"), "# Source").unwrap();

    Command::new("git")
        .current_dir(&temp_dir)
        .args(&["add", "."])
        .assert()
        .success();

    Command::new("git")
        .current_dir(&temp_dir)
        .args(&["commit", "-m", "Add source"])
        .assert()
        .success();

    // When: Trying to create a link with missing target
    // Then: It fails with appropriate error
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .args(&["link", "source.md", "missing.md"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Target file not found"));
}

#[test]
fn link_automatically_commits_to_git() {
    // Given: A gitmind repo with two files
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
        .args(&["commit", "-m", "Initial files"])
        .assert()
        .success();

    // When: Creating a link
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .args(&["link", "a.md", "b.md"])
        .assert()
        .success();

    // Then: The link is committed to git
    let output = Command::new("git")
        .current_dir(&temp_dir)
        .args(&["log", "--oneline", "-1"])
        .output()
        .unwrap();

    let log = String::from_utf8(output.stdout).unwrap();
    assert!(log.contains("link(F001): a.md -> b.md"));
}

#[test]
fn link_deduplicates_identical_links() {
    // Given: A gitmind repo with two files and an existing link
    let temp_dir = setup_gitmind_repo();
    fs::write(temp_dir.path().join("x.md"), "# X").unwrap();
    fs::write(temp_dir.path().join("y.md"), "# Y").unwrap();

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
        .args(&["link", "x.md", "y.md"])
        .assert()
        .success();

    // When: Creating the same link again
    Command::cargo_bin("gitmind")
        .unwrap()
        .current_dir(&temp_dir)
        .args(&["link", "x.md", "y.md"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Link already exists"));

    // Then: Only one link file exists
    let links_dir = temp_dir.path().join(".gitmind/links");
    let entries: Vec<_> = fs::read_dir(&links_dir).unwrap().collect();
    assert_eq!(entries.len(), 1);
}
