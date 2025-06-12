// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective

use gitmind::App;
use std::fs;
use std::process::Command as StdCommand;
use tempfile::TempDir;

#[test]
fn test_unlink_removes_specific_link() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize git repo
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["init"])
        .output()
        .unwrap();

    // Configure git user
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.email", "test@example.com"])
        .output()
        .unwrap();
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.name", "Test User"])
        .output()
        .unwrap();

    // Initialize gitmind
    let app = App::new(repo_path);
    let result = app.init();
    assert_eq!(result.code, 0, "init should succeed");

    // Create test files
    let source = "source.md";
    let target = "target.md";
    fs::write(repo_path.join(source), "# Source").unwrap();
    fs::write(repo_path.join(target), "# Target").unwrap();

    // Create a link
    let result = app.link(source, target, "CROSS_REF");
    assert_eq!(result.code, 0, "link should succeed");
    assert!(result.value.is_some(), "link should return SHA");

    // Verify link exists
    let result = app.list(None, None);
    assert_eq!(result.code, 0, "list should succeed");
    let links = result.value.unwrap();
    assert_eq!(links.len(), 1, "should have one link");
    assert_eq!(links[0].source, source);
    assert_eq!(links[0].target, target);
    assert_eq!(links[0].link_type, "CROSS_REF");

    // Unlink the files
    let result = app.unlink(source, target, None);
    assert_eq!(result.code, 0, "unlink should succeed");
    assert_eq!(result.value, Some(1), "should remove one link");

    // Verify link is removed
    let result = app.list(None, None);
    assert_eq!(result.code, 0, "list should succeed");
    let links = result.value.unwrap();
    assert_eq!(links.len(), 0, "should have no links");

    // Verify Git commit was made
    let output = StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["log", "--oneline"])
        .output()
        .unwrap();
    let log = String::from_utf8_lossy(&output.stdout);
    assert!(log.contains("unlink(F016):"));
    assert!(log.contains(&format!("{} -/-> {}", source, target)));
}

#[test]
fn test_unlink_with_type_removes_only_matching_type() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize git repo
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["init"])
        .output()
        .unwrap();

    // Configure git user
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.email", "test@example.com"])
        .output()
        .unwrap();
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.name", "Test User"])
        .output()
        .unwrap();

    // Initialize gitmind
    let app = App::new(repo_path);
    let result = app.init();
    assert_eq!(result.code, 0, "init should succeed");

    // Create test files
    let source = "source.md";
    let target = "target.md";
    fs::write(repo_path.join(source), "# Source").unwrap();
    fs::write(repo_path.join(target), "# Target").unwrap();

    // Create two links with different types
    let result = app.link(source, target, "CROSS_REF");
    assert_eq!(result.code, 0, "first link should succeed");
    assert!(result.value.is_some());

    let result = app.link(source, target, "DEPENDS_ON");
    assert_eq!(result.code, 0, "second link should succeed");
    assert!(result.value.is_some());

    // Verify both links exist
    let result = app.list(None, None);
    assert_eq!(result.code, 0, "list should succeed");
    let links = result.value.unwrap();
    assert_eq!(links.len(), 2, "should have two links");
    assert!(links
        .iter()
        .any(|l| l.link_type == "CROSS_REF" && l.source == source && l.target == target));
    assert!(links
        .iter()
        .any(|l| l.link_type == "DEPENDS_ON" && l.source == source && l.target == target));

    // Unlink only CROSS_REF type
    let result = app.unlink(source, target, Some("CROSS_REF"));
    assert_eq!(result.code, 0, "unlink should succeed");
    assert_eq!(result.value, Some(1), "should remove one link");

    // Verify only one link remains
    let result = app.list(None, None);
    assert_eq!(result.code, 0, "list should succeed");
    let links = result.value.unwrap();
    assert_eq!(links.len(), 1, "should have one link remaining");
    assert_eq!(links[0].link_type, "DEPENDS_ON");
    assert_eq!(links[0].source, source);
    assert_eq!(links[0].target, target);
}

#[test]
fn test_unlink_all_from_source() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize git repo
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["init"])
        .output()
        .unwrap();

    // Configure git user
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.email", "test@example.com"])
        .output()
        .unwrap();
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.name", "Test User"])
        .output()
        .unwrap();

    // Initialize gitmind
    let app = App::new(repo_path);
    let result = app.init();
    assert_eq!(result.code, 0, "init should succeed");

    // Create test files
    let source = "source.md";
    let target1 = "target1.md";
    let target2 = "target2.md";
    fs::write(repo_path.join(source), "# Source").unwrap();
    fs::write(repo_path.join(target1), "# Target 1").unwrap();
    fs::write(repo_path.join(target2), "# Target 2").unwrap();

    // Create multiple links from source
    let result = app.link(source, target1, "CROSS_REF");
    assert_eq!(result.code, 0, "first link should succeed");
    assert!(result.value.is_some());

    let result = app.link(source, target2, "DEPENDS_ON");
    assert_eq!(result.code, 0, "second link should succeed");
    assert!(result.value.is_some());

    // Verify links exist
    let result = app.list(None, None);
    assert_eq!(result.code, 0, "list should succeed");
    let links = result.value.unwrap();
    assert_eq!(links.len(), 2, "should have two links");
    assert!(links
        .iter()
        .any(|l| l.link_type == "CROSS_REF" && l.source == source && l.target == target1));
    assert!(links
        .iter()
        .any(|l| l.link_type == "DEPENDS_ON" && l.source == source && l.target == target2));

    // Unlink all from source
    let result = app.unlink_all_from(source, None);
    assert_eq!(result.code, 0, "unlink all should succeed");
    assert_eq!(result.value, Some(2), "should remove two links");

    // Verify all links from source are removed
    let result = app.list(None, None);
    assert_eq!(result.code, 0, "list should succeed");
    let links = result.value.unwrap();
    assert_eq!(links.len(), 0, "should have no links");
}

#[test]
fn test_unlink_all_to_target() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize git repo
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["init"])
        .output()
        .unwrap();

    // Configure git user
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.email", "test@example.com"])
        .output()
        .unwrap();
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.name", "Test User"])
        .output()
        .unwrap();

    // Initialize gitmind
    let app = App::new(repo_path);
    let result = app.init();
    assert_eq!(result.code, 0, "init should succeed");

    // Create test files
    let source1 = "source1.md";
    let source2 = "source2.md";
    let target = "target.md";
    fs::write(repo_path.join(source1), "# Source 1").unwrap();
    fs::write(repo_path.join(source2), "# Source 2").unwrap();
    fs::write(repo_path.join(target), "# Target").unwrap();

    // Create multiple links to target
    let result = app.link(source1, target, "CROSS_REF");
    assert_eq!(result.code, 0, "first link should succeed");
    assert!(result.value.is_some());

    let result = app.link(source2, target, "DEPENDS_ON");
    assert_eq!(result.code, 0, "second link should succeed");
    assert!(result.value.is_some());

    // Verify links exist
    let result = app.list(None, None);
    assert_eq!(result.code, 0, "list should succeed");
    let links = result.value.unwrap();
    assert_eq!(links.len(), 2, "should have two links");
    assert!(links
        .iter()
        .any(|l| l.link_type == "CROSS_REF" && l.source == source1 && l.target == target));
    assert!(links
        .iter()
        .any(|l| l.link_type == "DEPENDS_ON" && l.source == source2 && l.target == target));

    // Unlink all to target
    let result = app.unlink_to(target, None);
    assert_eq!(result.code, 0, "unlink to target should succeed");
    assert_eq!(result.value, Some(2), "should remove two links");

    // Verify all links to target are removed
    let result = app.list(None, None);
    assert_eq!(result.code, 0, "list should succeed");
    let links = result.value.unwrap();
    assert_eq!(links.len(), 0, "should have no links");
}

#[test]
fn test_unlink_non_existent_link_succeeds_with_message() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize git repo
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["init"])
        .output()
        .unwrap();

    // Configure git user
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.email", "test@example.com"])
        .output()
        .unwrap();
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["config", "user.name", "Test User"])
        .output()
        .unwrap();

    // Initialize gitmind
    let app = App::new(repo_path);
    let result = app.init();
    assert_eq!(result.code, 0, "init should succeed");

    // Create test files but no link
    let source = "source.md";
    let target = "target.md";
    fs::write(repo_path.join(source), "# Source").unwrap();
    fs::write(repo_path.join(target), "# Target").unwrap();

    // Try to unlink non-existent link
    let result = app.unlink(source, target, None);
    assert_eq!(
        result.code, 0,
        "unlink should succeed even when no links exist"
    );
    assert_eq!(result.value, Some(0), "should remove zero links");
}

#[test]
fn test_unlink_requires_gitmind_init() {
    let temp_dir = TempDir::new().unwrap();
    let repo_path = temp_dir.path();

    // Initialize only git repo (not gitmind)
    StdCommand::new("git")
        .current_dir(&repo_path)
        .args(["init"])
        .output()
        .unwrap();

    // Try to unlink
    let app = App::new(repo_path);
    let result = app.unlink("a.md", "b.md", None);
    assert_eq!(
        result.code, 3,
        "unlink should fail with NotInitialized code"
    );
    assert_eq!(result.value, None, "failed operation should have no value");
}
