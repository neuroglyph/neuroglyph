// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Tests for the list command behavior

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
fn list_shows_all_links_in_repository() {
    // Given: A gitmind repo with multiple links
    let temp_dir = setup_gitmind_repo();
    let repo_path = temp_dir.path();

    // Create files
    fs::write(repo_path.join("file1.md"), "# File 1").unwrap();
    fs::write(repo_path.join("file2.md"), "# File 2").unwrap();
    fs::write(repo_path.join("file3.md"), "# File 3").unwrap();

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

    // Create some links
    let app = App::new(repo_path);
    let result = app.link("file1.md", "file2.md", "CROSS_REF");
    assert_eq!(result.code, 0, "first link should succeed");

    let result = app.link("file2.md", "file3.md", "DEPENDS_ON");
    assert_eq!(result.code, 0, "second link should succeed");

    // When: Running gitmind list
    let result = app.list(None, None);

    // Then: All links are returned
    assert_eq!(result.code, 0, "list should succeed");
    let links = result.value.unwrap();
    assert_eq!(links.len(), 2, "should have two links");

    // Verify first link
    assert!(links.iter().any(|l| {
        l.source == "file1.md" && l.target == "file2.md" && l.link_type == "CROSS_REF"
    }));

    // Verify second link
    assert!(links.iter().any(|l| {
        l.source == "file2.md" && l.target == "file3.md" && l.link_type == "DEPENDS_ON"
    }));
}

#[test]
fn list_shows_empty_when_no_links() {
    // Given: A gitmind repo with no links
    let temp_dir = setup_gitmind_repo();
    let repo_path = temp_dir.path();

    // When: Running gitmind list
    let app = App::new(repo_path);
    let result = app.list(None, None);

    // Then: It returns empty list
    assert_eq!(result.code, 0, "list should succeed");
    let links = result.value.unwrap();
    assert_eq!(links.len(), 0, "should have no links");
}

#[test]
fn list_shows_link_timestamps() {
    // Given: A gitmind repo with a link
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
        .args(&["commit", "-m", "Add files"])
        .output()
        .unwrap();

    let app = App::new(repo_path);
    let result = app.link("a.md", "b.md", "CROSS_REF");
    assert_eq!(result.code, 0, "link should succeed");

    // When: Running gitmind list
    let result = app.list(None, None);

    // Then: Link includes timestamp
    assert_eq!(result.code, 0, "list should succeed");
    let links = result.value.unwrap();
    assert_eq!(links.len(), 1, "should have one link");
    assert!(links[0].timestamp > 0, "link should have timestamp");
}

#[test]
fn list_filters_by_source_file() {
    // Given: A gitmind repo with multiple links
    let temp_dir = setup_gitmind_repo();
    let repo_path = temp_dir.path();

    // Create files
    fs::write(repo_path.join("a.md"), "# A").unwrap();
    fs::write(repo_path.join("b.md"), "# B").unwrap();
    fs::write(repo_path.join("c.md"), "# C").unwrap();

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

    // Create links from different sources
    let app = App::new(repo_path);
    let result = app.link("a.md", "b.md", "CROSS_REF");
    assert_eq!(result.code, 0, "first link should succeed");

    let result = app.link("b.md", "c.md", "CROSS_REF");
    assert_eq!(result.code, 0, "second link should succeed");

    // When: Filtering by source
    let result = app.list(Some("a.md"), None);

    // Then: Only links from that source are shown
    assert_eq!(result.code, 0, "list should succeed");
    let links = result.value.unwrap();
    assert_eq!(links.len(), 1, "should have one link from source");
    assert_eq!(links[0].source, "a.md");
    assert_eq!(links[0].target, "b.md");
}

#[test]
fn list_filters_by_target_file() {
    // Given: A gitmind repo with multiple links
    let temp_dir = setup_gitmind_repo();
    let repo_path = temp_dir.path();

    // Create files
    fs::write(repo_path.join("a.md"), "# A").unwrap();
    fs::write(repo_path.join("b.md"), "# B").unwrap();
    fs::write(repo_path.join("c.md"), "# C").unwrap();

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

    // Create links to different targets
    let app = App::new(repo_path);
    let result = app.link("a.md", "b.md", "CROSS_REF");
    assert_eq!(result.code, 0, "first link should succeed");

    let result = app.link("a.md", "c.md", "CROSS_REF");
    assert_eq!(result.code, 0, "second link should succeed");

    // When: Filtering by target
    let result = app.list(None, Some("b.md"));

    // Then: Only links to that target are shown
    assert_eq!(result.code, 0, "list should succeed");
    let links = result.value.unwrap();
    assert_eq!(links.len(), 1, "should have one link to target");
    assert_eq!(links[0].source, "a.md");
    assert_eq!(links[0].target, "b.md");
}

#[test]
fn list_fails_if_gitmind_not_initialized() {
    // Given: A git repo without gitmind initialized
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    // When: Running gitmind list
    let app = App::new(repo_path);
    let result = app.list(None, None);

    // Then: It fails with NotInitialized exit code
    assert_eq!(result.code, 3, "list should fail with NotInitialized code");
    assert_eq!(result.value, None, "failed operation should have no value");
}
