// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Tests for the check command behavior

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
fn check_finds_broken_links_when_files_deleted() {
    // Given: A repo with links where target files are deleted
    let temp_dir = setup_gitmind_repo();
    let repo_path = temp_dir.path();

    // Create files and links
    fs::write(repo_path.join("doc.md"), "# Documentation").unwrap();
    fs::write(repo_path.join("code.rs"), "// Code").unwrap();
    fs::write(repo_path.join("test.rs"), "// Test").unwrap();

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
    app.link("doc.md", "code.rs", "DOCUMENTS");
    app.link("code.rs", "test.rs", "TESTED_BY");

    // Delete a target file
    fs::remove_file(repo_path.join("code.rs")).unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["rm", "code.rs"])
        .output()
        .unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["commit", "-m", "Remove code.rs"])
        .output()
        .unwrap();

    // When: Running check
    let result = app.check(false, false);

    // Then: It finds 2 broken links (doc->code and code->test)
    assert_eq!(result.code, 0, "check should succeed");
    let broken_links = result.value.unwrap();
    assert_eq!(broken_links.len(), 2, "should find 2 broken links");

    // Verify the broken links
    assert!(broken_links
        .iter()
        .any(|l| l.source == "doc.md" && l.target == "code.rs"));
    assert!(broken_links
        .iter()
        .any(|l| l.source == "code.rs" && l.target == "test.rs"));
}

#[test]
fn check_returns_empty_when_no_broken_links() {
    // Given: A repo with valid links
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
    app.link("a.md", "b.md", "CROSS_REF");

    // When: Running check
    let result = app.check(false, false);

    // Then: No broken links found
    assert_eq!(result.code, 0, "check should succeed");
    let broken_links = result.value.unwrap();
    assert_eq!(broken_links.len(), 0, "should find no broken links");
}

#[test]
fn check_with_fix_removes_broken_links() {
    // Given: A repo with broken links
    let temp_dir = setup_gitmind_repo();
    let repo_path = temp_dir.path();

    fs::write(repo_path.join("exists.md"), "# Exists").unwrap();
    fs::write(repo_path.join("deleted.md"), "# Deleted").unwrap();
    fs::write(repo_path.join("also_deleted.md"), "# Also Deleted").unwrap();

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
    app.link("exists.md", "deleted.md", "CROSS_REF");
    app.link("deleted.md", "also_deleted.md", "DEPENDS_ON");

    // Delete files
    fs::remove_file(repo_path.join("deleted.md")).unwrap();
    fs::remove_file(repo_path.join("also_deleted.md")).unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["rm", "deleted.md", "also_deleted.md"])
        .output()
        .unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["commit", "-m", "Remove files"])
        .output()
        .unwrap();

    // When: Running check with --fix
    let result = app.check(true, false);

    // Then: Broken links are removed
    assert_eq!(result.code, 0, "check --fix should succeed");
    let fixed_links = result.value.unwrap();
    assert_eq!(fixed_links.len(), 2, "should fix 2 broken links");

    // Verify links are actually removed
    let list_result = app.list(None, None);
    let remaining_links = list_result.value.unwrap();
    assert_eq!(remaining_links.len(), 0, "all links should be removed");

    // Verify git commit was made
    let output = StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["log", "--oneline", "-1"])
        .output()
        .unwrap();
    let log = String::from_utf8(output.stdout).unwrap();
    assert!(log.contains("check(F016): removed 2 broken links"));
}

#[test]
fn check_with_dry_run_does_not_modify_links() {
    // Given: A repo with broken links
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
    app.link("a.md", "b.md", "CROSS_REF");

    // Delete target file
    fs::remove_file(repo_path.join("b.md")).unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["rm", "b.md"])
        .output()
        .unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["commit", "-m", "Remove b.md"])
        .output()
        .unwrap();

    // Count commits before dry run
    let output = StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["rev-list", "--count", "HEAD"])
        .output()
        .unwrap();
    let commit_count_before = String::from_utf8(output.stdout).unwrap().trim().to_string();

    // When: Running check with --dry-run
    let result = app.check(true, true);

    // Then: Reports what would be fixed but doesn't modify
    assert_eq!(result.code, 0, "check --dry-run should succeed");
    let would_fix = result.value.unwrap();
    assert_eq!(would_fix.len(), 1, "should report 1 link would be fixed");

    // Verify link still exists
    let list_result = app.list(None, None);
    let links = list_result.value.unwrap();
    assert_eq!(links.len(), 1, "link should still exist");

    // Verify no new commit was made
    let output = StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["rev-list", "--count", "HEAD"])
        .output()
        .unwrap();
    let commit_count_after = String::from_utf8(output.stdout).unwrap().trim().to_string();
    assert_eq!(
        commit_count_before, commit_count_after,
        "no commit should be made"
    );
}

#[test]
fn check_fails_if_gitmind_not_initialized() {
    // Given: A git repo without gitmind initialized
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    // When: Running check
    let app = App::new(repo_path);
    let result = app.check(false, false);

    // Then: It fails with NotInitialized exit code
    assert_eq!(result.code, 3, "check should fail with NotInitialized code");
    assert_eq!(result.value, None, "failed operation should have no value");
}

#[test]
fn check_handles_links_where_source_is_also_deleted() {
    // Given: A repo where both source and target of a link are deleted
    let temp_dir = setup_gitmind_repo();
    let repo_path = temp_dir.path();

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

    let app = App::new(repo_path);
    app.link("a.md", "b.md", "CROSS_REF");
    app.link("b.md", "c.md", "CROSS_REF");

    // Delete both a.md and b.md
    fs::remove_file(repo_path.join("a.md")).unwrap();
    fs::remove_file(repo_path.join("b.md")).unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["rm", "a.md", "b.md"])
        .output()
        .unwrap();

    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(&["commit", "-m", "Remove files"])
        .output()
        .unwrap();

    // When: Running check
    let result = app.check(false, false);

    // Then: Both links are identified as broken
    assert_eq!(result.code, 0, "check should succeed");
    let broken_links = result.value.unwrap();
    assert_eq!(broken_links.len(), 2, "should find 2 broken links");
}
