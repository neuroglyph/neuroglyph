// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Tests for the link command behavior

use gitmind::App;
use std::fs;
use std::process::Command as StdCommand;
use tempfile::TempDir;

/// Helper to set up a git repo with gitmind initialized
fn setup_gitmind_repo() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize git
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    // Configure git user
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["config", "user.name", "Test User"])
        .output()
        .unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["config", "user.email", "test@example.com"])
        .output()
        .unwrap();

    // Initialize gitmind
    let app = App::new(repo_path);
    let result = app.init();
    assert_eq!(result.code, 0, "init should succeed");

    temp_dir
}

#[test]
fn link_creates_semantic_link_between_files() {
    // Given: A gitmind repo with two files
    let temp_dir = setup_gitmind_repo();
    let repo_path = temp_dir.path();
    fs::write(repo_path.join("notes.md"), "# Notes").unwrap();
    fs::write(repo_path.join("ideas.md"), "# Ideas").unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["add", "."])
        .output()
        .unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["commit", "-m", "Add files"])
        .output()
        .unwrap();

    // When: Creating a link between them
    let app = App::new(repo_path);
    let result = app.link("notes.md", "ideas.md", "CROSS_REF");

    // Then: Link creation succeeds and SHA is returned
    assert_eq!(result.code, 0, "link should succeed");
    assert!(result.value.is_some(), "link should return SHA");

    // And: A link file is created with correct content
    let links_dir = repo_path.join(".gitmind/links");
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
    let repo_path = temp_dir.path();
    fs::write(repo_path.join("spec.md"), "# Spec").unwrap();
    fs::write(repo_path.join("impl.rs"), "// Code").unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["add", "."])
        .output()
        .unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["commit", "-m", "Add files"])
        .output()
        .unwrap();

    // When: Creating a link with custom type
    let app = App::new(repo_path);
    let result = app.link("spec.md", "impl.rs", "IMPLEMENTS");

    // Then: Link creation succeeds
    assert_eq!(result.code, 0, "link should succeed");
    assert!(result.value.is_some(), "link should return SHA");

    // And: The link file contains the custom type
    let links_dir = repo_path.join(".gitmind/links");
    let entries: Vec<_> = fs::read_dir(&links_dir).unwrap().collect();
    let link_file = entries[0].as_ref().unwrap().path();
    let content = fs::read_to_string(&link_file).unwrap();
    assert!(content.contains("IMPLEMENTS: spec.md -> impl.rs"));
}

#[test]
fn link_fails_when_source_file_missing() {
    // Given: A gitmind repo without the source file
    let temp_dir = setup_gitmind_repo();
    let repo_path = temp_dir.path();

    // When: Trying to create a link with missing source
    let app = App::new(repo_path);
    let result = app.link("missing.md", "target.md", "CROSS_REF");

    // Then: It fails with SourceNotFound exit code
    assert_eq!(result.code, 5, "link should fail with SourceNotFound code");
    assert_eq!(result.value, None, "failed operation should have no value");
}

#[test]
fn link_fails_when_target_file_missing() {
    // Given: A gitmind repo with only source file
    let temp_dir = setup_gitmind_repo();
    let repo_path = temp_dir.path();
    fs::write(repo_path.join("source.md"), "# Source").unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["add", "."])
        .output()
        .unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["commit", "-m", "Add source"])
        .output()
        .unwrap();

    // When: Trying to create a link with missing target
    let app = App::new(repo_path);
    let result = app.link("source.md", "missing.md", "CROSS_REF");

    // Then: It fails with TargetNotFound exit code
    assert_eq!(result.code, 6, "link should fail with TargetNotFound code");
    assert_eq!(result.value, None, "failed operation should have no value");
}

#[test]
fn link_automatically_commits_to_git() {
    // Given: A gitmind repo with two files
    let temp_dir = setup_gitmind_repo();
    let repo_path = temp_dir.path();
    fs::write(repo_path.join("a.md"), "# A").unwrap();
    fs::write(repo_path.join("b.md"), "# B").unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["add", "."])
        .output()
        .unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["commit", "-m", "Initial files"])
        .output()
        .unwrap();

    // When: Creating a link
    let app = App::new(repo_path);
    let result = app.link("a.md", "b.md", "CROSS_REF");
    assert_eq!(result.code, 0, "link should succeed");

    // Then: The link is committed to git
    let output = StdCommand::new("git")
        .current_dir(&repo_path)
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
    let repo_path = temp_dir.path();
    fs::write(repo_path.join("x.md"), "# X").unwrap();
    fs::write(repo_path.join("y.md"), "# Y").unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["add", "."])
        .output()
        .unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["commit", "-m", "Add files"])
        .output()
        .unwrap();

    let app = App::new(repo_path);
    let result = app.link("x.md", "y.md", "CROSS_REF");
    assert_eq!(result.code, 0, "first link should succeed");

    // When: Creating the same link again
    let result = app.link("x.md", "y.md", "CROSS_REF");

    // Then: It fails with LinkAlreadyExists exit code
    assert_eq!(
        result.code, 7,
        "duplicate link should fail with LinkAlreadyExists code"
    );
    assert_eq!(result.value, None, "failed operation should have no value");

    // And: Only one link file exists
    let links_dir = repo_path.join(".gitmind/links");
    let entries: Vec<_> = fs::read_dir(&links_dir).unwrap().collect();
    assert_eq!(entries.len(), 1);
}
