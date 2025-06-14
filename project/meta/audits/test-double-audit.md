# Test Double Friendliness Audit

This audit identifies areas in the Neuroglyph codebase where test doubles could improve testability, while respecting the project's philosophy of using real Git repos for testing.

## Summary of Findings

### 1. **Time Generation in LinkCommand** ⚠️ HIGH PRIORITY
**File**: `cli/src/commands/link.rs:39`
```rust
let timestamp = chrono::Utc::now().timestamp();
```
**Issue**: Hardcoded time generation makes it impossible to test time-dependent behavior or create deterministic tests.
**Test Double Beneficial?**: YES - for testing edge cases like:
- Links created at specific times (e.g., Y2K, epoch time)
- Ordering behavior when links have specific timestamps
- Time zone edge cases
**Suggested Improvement**: 
```rust
pub trait Clock {
    fn now(&self) -> i64;
}

pub struct SystemClock;
impl Clock for SystemClock {
    fn now(&self) -> i64 {
        chrono::Utc::now().timestamp()
    }
}

pub struct LinkCommand<G: GitOperations, F: FileSystem, C: Clock> {
    context: GitMindContext,
    git: G,
    fs: F,
    clock: C,
}
```

### 2. **Environment Variable Access in main.rs** ✅ ACCEPTABLE
**File**: `cli/src/main.rs:76`
```rust
let working_dir = match std::env::current_dir() {
```
**Issue**: Direct access to environment
**Test Double Beneficial?**: NO - This is in main.rs which is the boundary layer. Tests should use App::new() directly with a known path.
**Suggested Improvement**: None needed - this is appropriate for the CLI entry point.

### 3. **Dependency Creation in lib.rs** ⚠️ MEDIUM PRIORITY
**File**: `cli/src/lib.rs:34-107`
**Issue**: App methods create their own dependencies (GitClient, StdFileSystem) internally
**Test Double Beneficial?**: MAYBE - Currently tests can't inject custom implementations
**Current State**:
```rust
pub fn init(&self) -> CommandResult<()> {
    let git = GitClient;
    let fs = StdFileSystem;
    let cmd = commands::InitCommand::new(&self.working_dir, git, fs);
    cmd.execute().into_command_result()
}
```
**Suggested Improvement**: 
```rust
pub struct App<G: GitOperations, F: FileSystem, C: Clock> {
    working_dir: PathBuf,
    git: G,
    fs: F,
    clock: C,
}

impl App<GitClient, StdFileSystem, SystemClock> {
    pub fn new(working_dir: impl AsRef<Path>) -> Self {
        Self {
            working_dir: working_dir.as_ref().to_path_buf(),
            git: GitClient,
            fs: StdFileSystem,
            clock: SystemClock,
        }
    }
}
```

### 4. **SHA Generation in Link** ✅ GOOD
**File**: `cli/src/link.rs:38-47`
**Issue**: None - SHA generation is deterministic based on link content
**Test Double Beneficial?**: NO - This is pure computation with no external dependencies

### 5. **Git and FileSystem Traits** ✅ EXCELLENT
**Files**: `cli/src/git.rs`, `cli/src/filesystem.rs`
**Issue**: None - Already using dependency injection pattern correctly
**Test Double Beneficial?**: Already implemented and used appropriately

## Test File Analysis

### Current Test Practices ✅ GOOD
All test files correctly:
- Use real Git repositories (via TempDir)
- Test behavior, not implementation
- Don't test stdout/stderr (following CommandResult pattern)
- Create isolated environments for each test

### Areas Where Test Doubles Would Help

1. **Testing Time-Dependent Behavior**
   - Currently impossible to test specific timestamp scenarios
   - Can't verify link ordering with controlled timestamps
   - Can't test edge cases like epoch boundaries

2. **Testing Error Scenarios**
   - While FileSystem trait allows error injection, it's not used in tests
   - Could test disk full, permission denied, etc.

## Recommendations

### High Priority
1. **Add Clock trait for time injection**
   - Enables deterministic testing
   - Allows testing time-dependent edge cases
   - Maintains real behavior by default

### Medium Priority
2. **Consider dependency injection at App level**
   - Would allow tests to inject custom implementations
   - Could enable more complex integration testing scenarios
   - Maintains backward compatibility with default implementations

### Low Priority
3. **Add test doubles for specific error scenarios**
   - Create test-specific FileSystem implementations that fail in specific ways
   - Only for edge cases that can't be triggered with real implementations

## What NOT to Change

Per CLAUDE.md guidelines:
- Keep using real Git repos for all Git operations
- Don't mock Git - it's the core of the system
- Continue testing behavior, not implementation
- Don't add test doubles unless they enable testing specific edge cases

## Example Implementation

Here's how to make LinkCommand test-double friendly for time:

```rust
// In cli/src/time.rs
pub trait Clock {
    fn now(&self) -> i64;
}

pub struct SystemClock;
impl Clock for SystemClock {
    fn now(&self) -> i64 {
        chrono::Utc::now().timestamp()
    }
}

#[cfg(test)]
pub struct FixedClock {
    timestamp: i64,
}

#[cfg(test)]
impl FixedClock {
    pub fn new(timestamp: i64) -> Self {
        Self { timestamp }
    }
}

#[cfg(test)]
impl Clock for FixedClock {
    fn now(&self) -> i64 {
        self.timestamp
    }
}
```

This maintains the project philosophy while enabling edge case testing.

## Implementation Status

### Issues Fixed

1. **✅ Time Generation in LinkCommand** 
   - Created `Clock` trait in `cli/src/time.rs`
   - Updated `LinkCommand` to accept `Clock` as dependency
   - Created `FixedClock` test double for deterministic testing
   - Added test demonstrating fixed timestamp usage

2. **✅ Dependency Creation in lib.rs**
   - Made `App` generic over `GitOperations`, `FileSystem`, and `Clock`
   - Added `App::with_deps()` for dependency injection
   - Maintained backward compatibility with `App::new()`
   - All commands now receive injected dependencies

3. **✅ FileSystem Test Doubles Created**
   - Created `FailingFileSystem` in `cli/src/filesystem.rs`
   - Supports permission denied, disk full, and file not found scenarios
   - Added comprehensive tests in `cli/tests/edge_cases.rs`

4. **✅ Git Test Doubles Created**
   - Created `IndexLockedGit`, `NoHeadGit`, `CorruptedRepoGit`, `DiskFullGit`
   - Added tests in `cli/tests/git_edge_cases.rs`
   - Covers most common Git edge cases

## Audit Checklist

### Architecture Improvements
- [x] Create Clock trait for time abstraction
- [x] Update LinkCommand to use Clock trait
- [x] Make App generic over dependencies
- [x] Maintain backward compatibility
- [x] Update all commands to use injected dependencies

### Test Double Implementations
- [x] FixedClock for deterministic timestamps
- [x] FailingFileSystem for I/O error scenarios
- [x] IndexLockedGit for concurrent access errors
- [x] NoHeadGit for empty repository scenarios
- [x] CorruptedRepoGit for repository corruption
- [x] DiskFullGit for storage failures

### Test Coverage
- [x] Test for permission denied on filesystem operations
- [x] Test for disk full scenarios
- [x] Test for fixed timestamp behavior
- [x] Test for index lock contention
- [x] Test for operations in empty repositories
- [x] Test for corrupted repository handling
- [ ] Test for bare repository detection
- [ ] Test for detached HEAD scenarios
- [ ] Test for Git hook failures
- [ ] Test for worktree operations
- [ ] Test for submodule boundaries

### Documentation
- [x] Updated CLAUDE_DEV_THOUGHTS.md with implementation notes
- [x] Created gitoxide-test-double-analysis.md
- [x] Documented all test doubles with clear purpose
- [ ] Update user-facing error messages based on test findings

### Remaining Work
- [x] Add test doubles for remaining Git edge cases
- [ ] Create error handling improvement proposal
- [ ] Update error messages to be more helpful
- [x] Add tests for non-UTF8 path handling

### Completed June 13, 2025
- ✅ Added WorktreeGit, SubmoduleGit, InvalidEncodingGit test doubles
- ✅ Fixed check command to use `git rm` instead of filesystem removal + staging
- ✅ All 44 tests now passing
- ✅ Test doubles moved to common module for integration tests