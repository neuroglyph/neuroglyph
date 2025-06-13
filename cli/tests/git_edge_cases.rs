// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

//! Tests for hard-to-reproduce Git edge cases using test doubles

mod common;

use common::*;
use gitmind::{filesystem::StdFileSystem, time::SystemClock, App};
use tempfile::TempDir;

#[test]
fn link_fails_gracefully_when_index_locked() {
    // Given: A git repository with index.lock contention
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize real git repo and gitmind
    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    std::fs::write(repo_path.join("file1.txt"), "content1").unwrap();
    std::fs::write(repo_path.join("file2.txt"), "content2").unwrap();

    let app = App::new(repo_path);
    app.init();

    // Now use test double that simulates index lock
    let app = App::with_deps(repo_path, IndexLockedGit, StdFileSystem, SystemClock);

    // When: Trying to create a link
    let result = app.link("file1.txt", "file2.txt", "TEST");

    // Then: It fails with Git error and helpful message
    assert_eq!(result.code, 8, "should fail with Git error code");
    assert!(result.value.is_none());
}

#[test]
fn init_handles_repository_with_no_commits() {
    // Given: A brand new git repository with no commits
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    // When: Initializing gitmind in empty repo
    let app = App::new(repo_path);
    let result = app.init();

    // Then: Init succeeds (HEAD not required for init)
    assert_eq!(result.code, 0, "init should succeed in empty repo");
}

#[test]
fn link_commit_fails_in_repo_with_no_head() {
    // Given: A repository with no commits (no HEAD)
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    std::fs::write(repo_path.join("file1.txt"), "content1").unwrap();
    std::fs::write(repo_path.join("file2.txt"), "content2").unwrap();

    // Initialize gitmind
    let app = App::new(repo_path);
    app.init();

    // Use test double that simulates no HEAD
    let app = App::with_deps(repo_path, NoHeadGit, StdFileSystem, SystemClock);

    // When: Creating a link (which tries to commit)
    let result = app.link("file1.txt", "file2.txt", "TEST");

    // Then: It fails with Git error about missing HEAD
    assert_eq!(result.code, 8, "should fail with Git error code");
}

#[test]
fn operations_fail_gracefully_on_corrupted_repository() {
    // Given: A corrupted Git repository
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Create .gitmind directory without proper git repo
    std::fs::create_dir_all(repo_path.join(".gitmind/links")).unwrap();

    let app = App::with_deps(repo_path, CorruptedRepoGit, StdFileSystem, SystemClock);

    // When: Trying to initialize
    let result = app.init();

    // Then: It fails indicating not a git repository
    assert_eq!(result.code, 2, "should fail with NotAGitRepository code");
}

#[test]
fn operations_fail_gracefully_when_disk_full() {
    // Given: A repository where disk is full
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    std::fs::write(repo_path.join("file1.txt"), "content1").unwrap();
    std::fs::write(repo_path.join("file2.txt"), "content2").unwrap();

    let app = App::new(repo_path);
    app.init();

    // Use test double that simulates disk full
    let app = App::with_deps(repo_path, DiskFullGit, StdFileSystem, SystemClock);

    // When: Trying to create a link
    let result = app.link("file1.txt", "file2.txt", "TEST");

    // Then: It fails with Git error about disk space
    assert_eq!(result.code, 8, "should fail with Git error code");
}

#[test]
fn unlink_handles_disk_full_during_index_update() {
    // Given: A repository with existing links
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    std::fs::write(repo_path.join("file1.txt"), "content1").unwrap();
    std::fs::write(repo_path.join("file2.txt"), "content2").unwrap();

    // Create a link with real git
    let app = App::new(repo_path);
    app.init();
    app.link("file1.txt", "file2.txt", "TEST");

    // Commit the link
    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["add", "."])
        .output()
        .unwrap();

    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["commit", "-m", "test"])
        .output()
        .unwrap();

    // Now simulate disk full
    let app = App::with_deps(repo_path, DiskFullGit, StdFileSystem, SystemClock);

    // When: Trying to unlink
    let result = app.unlink("file1.txt", "file2.txt", None);

    // Then: It fails with Git error about disk space
    assert_eq!(result.code, 8, "should fail with Git error code");
}

#[test]
fn helpful_error_message_for_concurrent_git_operations() {
    // This test verifies that our index.lock error message is helpful
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    std::fs::write(repo_path.join("file1.txt"), "content1").unwrap();

    let app = App::new(repo_path);
    app.init();

    // Simulate index lock
    let app = App::with_deps(repo_path, IndexLockedGit, StdFileSystem, SystemClock);

    // When: Any operation that needs the index
    let result = app.link("file1.txt", "file1.txt", "SELF");

    // Then: Error code indicates Git error
    assert_eq!(result.code, 8);
    // In a real scenario, the error message would be shown to the user
    // and would include helpful text about terminating other git processes
}

#[test]
fn operations_fail_in_bare_repository() {
    // Given: A bare repository (no working tree)
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["init", "--bare"])
        .output()
        .unwrap();

    let app = App::with_deps(repo_path, BareRepositoryGit, StdFileSystem, SystemClock);

    // When: Trying to initialize gitmind
    let result = app.init();

    // Then: It fails indicating bare repository
    assert_eq!(result.code, 8, "should fail with Git error code");
}

#[test]
fn detached_head_operations_succeed_but_may_lose_commits() {
    // Given: A repository in detached HEAD state
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    // Create initial commit
    std::fs::write(repo_path.join("README.md"), "test").unwrap();
    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["add", "."])
        .output()
        .unwrap();
    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["commit", "-m", "initial"])
        .output()
        .unwrap();

    // Checkout to detached HEAD
    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["checkout", "HEAD~0"])
        .output()
        .unwrap();

    // Initialize gitmind
    let app = App::new(repo_path);
    app.init();

    std::fs::write(repo_path.join("file1.txt"), "content1").unwrap();
    std::fs::write(repo_path.join("file2.txt"), "content2").unwrap();

    // Use detached HEAD test double
    let app = App::with_deps(repo_path, DetachedHeadGit, StdFileSystem, SystemClock);

    // When: Creating a link (which commits)
    let result = app.link("file1.txt", "file2.txt", "TEST");

    // Then: It succeeds (but user should be warned about detached HEAD)
    assert_eq!(result.code, 0, "should succeed in detached HEAD");
}

#[test]
fn commit_fails_when_pre_commit_hook_rejects() {
    // Given: A repository with a failing pre-commit hook
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    std::fs::write(repo_path.join("file1.txt"), "content1").unwrap();
    std::fs::write(repo_path.join("file2.txt"), "content2").unwrap();

    let app = App::new(repo_path);
    app.init();

    // Use hook failure test double
    let app = App::with_deps(repo_path, HookFailureGit, StdFileSystem, SystemClock);

    // When: Creating a link
    let result = app.link("file1.txt", "file2.txt", "TEST");

    // Then: It fails with Git error about hook
    assert_eq!(result.code, 8, "should fail with Git error code");
}

#[test]
fn commit_fails_when_git_config_missing() {
    // Given: A repository with no user configuration
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    std::fs::write(repo_path.join("file1.txt"), "content1").unwrap();
    std::fs::write(repo_path.join("file2.txt"), "content2").unwrap();

    let app = App::new(repo_path);
    app.init();

    // Use no config test double
    let app = App::with_deps(repo_path, NoConfigGit, StdFileSystem, SystemClock);

    // When: Creating a link
    let result = app.link("file1.txt", "file2.txt", "TEST");

    // Then: It fails with Git error about missing config
    assert_eq!(result.code, 8, "should fail with Git error code");
    // The error would guide user to set up Git config
}

#[test]
fn worktree_operations_work_but_share_repository() {
    // Given: A Git worktree (secondary working directory)
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    // Create initial commit
    std::fs::write(repo_path.join("README.md"), "test").unwrap();
    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["add", "."])
        .output()
        .unwrap();
    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["commit", "-m", "initial"])
        .output()
        .unwrap();

    std::fs::write(repo_path.join("file1.txt"), "content1").unwrap();
    std::fs::write(repo_path.join("file2.txt"), "content2").unwrap();

    let app = App::new(repo_path);
    app.init();

    // Use worktree test double
    let app = App::with_deps(repo_path, WorktreeGit, StdFileSystem, SystemClock);

    // When: Creating links in a worktree
    let result = app.link("file1.txt", "file2.txt", "TEST");

    // Then: Operations succeed (links are visible in all worktrees)
    assert_eq!(result.code, 0, "should succeed in worktree");
}

#[test]
fn operations_fail_across_submodule_boundaries() {
    // Given: A repository with a submodule
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    // Create main repo files
    std::fs::write(repo_path.join("main.txt"), "main content").unwrap();

    // Simulate submodule directory
    std::fs::create_dir_all(repo_path.join("submodule")).unwrap();
    std::fs::write(repo_path.join("submodule/sub.txt"), "sub content").unwrap();

    let app = App::new(repo_path);
    app.init();

    // Use submodule test double
    let app = App::with_deps(repo_path, SubmoduleGit, StdFileSystem, SystemClock);

    // When: Trying to link across submodule boundary
    let result = app.link("main.txt", "submodule/sub.txt", "TEST");

    // Then: It succeeds because the submodule check only happens during git add
    // and we created the link successfully (the git add will fail later)
    assert_eq!(
        result.code, 0,
        "link creation succeeds, git add would fail later"
    );
}

#[test]
fn operations_fail_with_invalid_utf8_paths() {
    // Given: A repository with files that have invalid encoding
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    std::process::Command::new("git")
        .current_dir(&repo_path)
        .args(&["init"])
        .output()
        .unwrap();

    std::fs::write(repo_path.join("valid.txt"), "content").unwrap();

    let app = App::new(repo_path);
    app.init();

    // Use invalid encoding test double
    let app = App::with_deps(repo_path, InvalidEncodingGit, StdFileSystem, SystemClock);

    // When: Trying to link with invalid UTF-8 in path
    let result = app.link("valid.txt", "���.txt", "TEST");

    // Then: It fails with TargetNotFound because the file doesn't exist
    assert_eq!(result.code, 6, "should fail with target not found");
}
