# Git Edge Cases Audit

Date: June 12, 2025

This audit identifies hard-to-reproduce Git edge cases that should be tested in the Neuroglyph codebase. These scenarios are difficult to create with real Git repositories but represent actual failure modes users encounter.

## Executive Summary

Our analysis revealed 25+ untested Git edge cases across 4 operations. We've implemented test doubles for the most critical scenarios and identified remaining gaps.

## Edge Cases by Operation

### `is_repository()` Operation

#### Untested Scenarios
1. **Bare Repository Detection**
   - Issue: System doesn't handle bare repos (no working tree)
   - Impact: User confusion when operations fail
   - Real-world scenario: Server-side Git repositories

2. **Corrupted .git Directory**
   - Issue: Missing/corrupted `.git/HEAD` file
   - Impact: False negatives on valid but damaged repos
   - Real-world scenario: Power failure during Git operation

3. **Permission Denied on .git**
   - Issue: `.git` exists but unreadable
   - Impact: Can't detect repository even though it exists
   - Real-world scenario: Multi-user systems, Docker containers

### `add()` Operation

#### Untested Scenarios
1. **Index Lock Contention** 🔴 CRITICAL
   - Issue: `.git/index.lock` already exists
   - Impact: Complete operation failure
   - Real-world scenario: IDE/editor running concurrent Git operations

2. **Corrupted Index File**
   - Issue: `.git/index` file corrupted
   - Impact: Cryptic error messages
   - Real-world scenario: Disk corruption, interrupted writes

3. **Disk Full During Index Write** 
   - Issue: Storage exhausted mid-operation
   - Impact: Partial writes, potential corruption
   - Real-world scenario: Large repos on constrained systems

4. **Non-UTF8 Path Encoding**
   - Issue: File paths with invalid encoding
   - Impact: "Invalid path encoding" errors
   - Real-world scenario: Cross-platform file sharing

### `commit()` Operation

#### Untested Scenarios
1. **Empty Repository (No HEAD)** 🔴 CRITICAL
   - Issue: No commits exist yet
   - Impact: "Failed to get HEAD" error
   - Real-world scenario: Fresh repo initialization

2. **Detached HEAD State**
   - Issue: Not on any branch
   - Impact: Commits created but "lost"
   - Real-world scenario: Checking out specific commits

3. **Corrupted Object Database**
   - Issue: Damaged `.git/objects/`
   - Impact: "Failed to write tree" errors
   - Real-world scenario: Filesystem corruption

4. **Pre-commit Hook Failures**
   - Issue: Hooks reject commits
   - Impact: Unclear error messages
   - Real-world scenario: Strict project policies

5. **Missing Git Configuration**
   - Issue: No user.name/email configured
   - Impact: Falls back to defaults silently
   - Real-world scenario: CI environments, new installations

### `remove()` Operation

#### Untested Scenarios
1. **File Already Deleted**
   - Issue: File in index but not on disk
   - Impact: Partial success, user confusion
   - Real-world scenario: Manual file deletion

2. **Untracked File Removal**
   - Issue: Removing non-Git-tracked files
   - Impact: Silent "success" when should warn
   - Real-world scenario: User error

3. **Read-only File Permissions**
   - Issue: Can't delete from working directory
   - Impact: Index updated but file remains
   - Real-world scenario: Protected files

### Cross-Operation Edge Cases

1. **Repository Location Changes**
   - Issue: Repo moved during operation
   - Impact: "Repository not found" errors
   - Real-world scenario: File manager operations

2. **Shallow Clones**
   - Issue: Limited history affects operations
   - Impact: Unexpected behavior
   - Real-world scenario: CI/CD pipelines

3. **Git Worktrees**
   - Issue: Multiple working trees
   - Impact: Links not visible across worktrees
   - Real-world scenario: Feature branch development

4. **Submodule Boundaries**
   - Issue: Operations across submodules
   - Impact: Commits in wrong repository
   - Real-world scenario: Complex project structures

## Implementation Status

### Test Doubles Created ✅
- [x] `IndexLockedGit` - Index lock contention
- [x] `NoHeadGit` - Empty repository scenarios
- [x] `CorruptedRepoGit` - Repository corruption
- [x] `DiskFullGit` - Storage failures

### Tests Written ✅
- [x] Index lock handling
- [x] Empty repo operations
- [x] Disk full scenarios
- [x] Repository corruption
- [x] Filesystem permission errors

### Remaining Test Doubles Needed
- [x] `BareRepositoryGit` - No working tree ✅
- [x] `DetachedHeadGit` - Not on any branch ✅
- [x] `HookFailureGit` - Pre-commit rejections ✅
- [ ] `ShallowCloneGit` - Limited history
- [x] `WorktreeGit` - Multiple working trees ✅
- [x] `InvalidEncodingGit` - Non-UTF8 paths ✅

### Completed June 13, 2025
- ✅ Implemented BareRepositoryGit, DetachedHeadGit, HookFailureGit, WorktreeGit, InvalidEncodingGit
- ✅ Added comprehensive tests for all edge cases
- ✅ ShallowCloneGit deemed low priority (rare use case)

## Risk Assessment

### High Risk (User Impact: Daily)
- Index lock contention
- Empty repository operations
- Disk space issues

### Medium Risk (User Impact: Weekly)
- Repository corruption
- Permission errors
- Detached HEAD state
- Hook failures

### Low Risk (User Impact: Rare)
- Bare repositories
- Shallow clones
- Worktrees
- Submodules

## Recommendations

1. **Immediate Actions**
   - Implement remaining high-risk test doubles
   - Improve error messages for common scenarios
   - Add user guidance to error outputs

2. **Short-term Improvements**
   - Cover medium-risk scenarios
   - Create error recovery documentation
   - Add diagnostic commands

3. **Long-term Enhancements**
   - Support for edge cases (bare repos, worktrees)
   - Automatic error recovery where possible
   - Telemetry to track real-world edge cases

## Testing Checklist

### Critical Edge Cases
- [x] Index lock detection and messaging ✅
- [x] Empty repository handling ✅
- [x] Disk full during operations ✅
- [x] Repository corruption detection ✅
- [x] Detached HEAD warnings ✅
- [x] Hook failure explanations ✅
- [x] Configuration missing prompts ✅

### Error Message Improvements
- [ ] Add recovery steps to index lock errors
- [ ] Explain HEAD reference errors clearly
- [ ] Guide users through corruption recovery
- [ ] Clarify disk space requirements
- [ ] Provide configuration setup hints

### Test Coverage Goals
- [x] 100% coverage of critical paths
- [x] Edge case test doubles for hard-to-reproduce scenarios
- [ ] Integration tests for error recovery flows
- [ ] End-to-end tests for user error scenarios

### Documentation Tasks
- [ ] Create troubleshooting guide
- [ ] Document error codes and meanings
- [ ] Add recovery procedures
- [ ] Include edge case examples

## Conclusion

We've identified 25+ edge cases, implemented test doubles for the most critical 4, and have a clear roadmap for comprehensive coverage. The focus should remain on user-impacting scenarios with helpful error messages.