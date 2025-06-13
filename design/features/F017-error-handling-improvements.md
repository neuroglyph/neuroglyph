# F017: Error Handling Improvements

**Status:** PLANNED  
**Created:** June 12, 2025  
**Priority:** High (Pre-MVP)  

## Summary
Transform cryptic Git errors into helpful, actionable guidance that educates users and provides clear recovery paths.

## User Story
As a gitmind user, I want clear, helpful error messages when something goes wrong, so that I can understand what happened and fix it quickly without needing to search for solutions.

## Background
Current error handling passes through raw Git errors which are often cryptic and unhelpful. Users encountering errors like "fatal: Unable to create '.git/index.lock': File exists" have no guidance on resolution. This feature implements the comprehensive error handling system proposed in `design/proposals/error-handling-improvement.md`.

## Acceptance Criteria

### 1. Error Classification System
- [ ] All Git errors are classified into categories (Transient, Configuration, Corruption, Permissions, Concurrency, UserMistake)
- [ ] Each error has a severity level (Recoverable, Warning, Fatal)
- [ ] Error patterns are matched to known issues

### 2. Context-Aware Error Messages
- [ ] Error messages include what the user was trying to do
- [ ] Plain English explanation of why the error occurred
- [ ] Actionable solutions with specific commands when applicable
- [ ] Progressive disclosure with --verbose flag for expert details

### 3. Common Error Scenarios

#### Index Lock (Concurrency)
- [ ] Detect "index.lock exists" errors
- [ ] Provide clear explanation about concurrent Git processes
- [ ] Offer solutions: close tools, remove lock file, wait and retry
- [ ] Include safety warnings about removing lock files

#### No HEAD Reference (Empty Repo)
- [ ] Detect operations in repos with no commits
- [ ] Explain that GitMind needs at least one commit
- [ ] Provide exact command to create initial commit
- [ ] Guide user to retry their operation

#### Disk Full
- [ ] Detect "No space left on device" errors
- [ ] Show required vs available space
- [ ] Suggest specific cleanup commands (npm cache, brew cleanup, etc.)
- [ ] Indicate which operation failed due to space

#### Repository Not Found
- [ ] Detect "not a git repository" errors
- [ ] Suggest running `git init` first
- [ ] Explain GitMind requires a Git repository
- [ ] Check if user is in wrong directory

#### Permission Denied
- [ ] Detect permission errors on .git directory
- [ ] Explain permission requirements
- [ ] Suggest checking file ownership
- [ ] Provide commands to fix permissions

#### Missing Git Configuration
- [ ] Detect missing user.name/email
- [ ] Provide exact git config commands
- [ ] Explain why Git needs this information
- [ ] Offer to use defaults with warning

### 4. Error Recovery Mechanisms
- [ ] Implement automatic retry for transient errors
- [ ] Add `--retry` flag for manual retry with backoff
- [ ] Safe automatic recovery for specific scenarios
- [ ] Preserve operation context for retry

### 5. Help System Integration
- [ ] Each error has a unique code (e.g., E001)
- [ ] `gitmind help error:E001` shows detailed help
- [ ] Help includes extended explanation and examples
- [ ] Links to online documentation when available

## Technical Design

### New Dependencies
```toml
# Cargo.toml additions
colored = "2.0"  # For colored terminal output
backoff = "0.4"  # For retry mechanisms
```

### Core Components

#### 1. Error Classifier
```rust
pub enum ErrorCategory {
    Transient,      // Temporary issues (lock files, network)
    Configuration,  // Setup issues (no repo, no config)
    Corruption,     // Data integrity issues
    Permissions,    // Access control issues
    Concurrency,    // Multiple process conflicts
    UserMistake,    // Invalid operations
}

pub struct ErrorClassifier {
    patterns: HashMap<&'static str, KnownError>,
}
```

#### 2. Context Tracker
```rust
pub struct OperationContext {
    command: String,
    source_file: Option<String>,
    target_file: Option<String>,
    link_type: Option<String>,
}

pub struct ContextStack {
    operations: Vec<OperationContext>,
}
```

#### 3. Solution Generator
```rust
pub struct Solution {
    description: String,
    command: Option<String>,
    risk_level: RiskLevel,
    can_automate: bool,
}

pub trait SolutionProvider {
    fn get_solutions(&self, error: &KnownError, context: &OperationContext) -> Vec<Solution>;
}
```

### Error Message Format
```
❌ Cannot {operation}: {brief description}

{detailed explanation}

{contextual information}

Solutions:
1. {primary solution}
2. {alternative solution}
[...]

{additional help reference}
```

## Test Requirements

### Unit Tests
- [ ] Error classifier correctly categorizes all known patterns
- [ ] Context tracker maintains operation stack
- [ ] Solution generator provides appropriate fixes
- [ ] Error formatter produces expected output

### Integration Tests
- [ ] Each error scenario produces correct user message
- [ ] Retry mechanisms work for transient errors
- [ ] Help system displays error-specific guidance
- [ ] Colored output works across terminals

### Edge Case Tests
Using our test doubles, verify behavior for:

1. **Concurrent Operations**
   - [ ] Multiple gitmind instances detect lock conflicts
   - [ ] Clear message about which process holds lock
   - [ ] Safe cleanup after crashed processes

2. **Filesystem Issues**
   - [ ] Disk full during various operations
   - [ ] Permission denied on different paths
   - [ ] Read-only filesystem handling

3. **Git State Issues**
   - [ ] Operations in bare repositories
   - [ ] Detached HEAD warnings
   - [ ] Shallow clone limitations
   - [ ] Corrupted repository recovery

4. **Configuration Issues**
   - [ ] Missing git config with helpful setup
   - [ ] Invalid config values
   - [ ] Repository not initialized

5. **Network Issues** (future)
   - [ ] Remote repository unreachable
   - [ ] Authentication failures
   - [ ] Partial transfers

## Error Behavior Specifications

### Index Lock Exists
**Trigger**: Another process has `.git/index.lock`
**Behavior**: 
1. Detect lock file existence
2. Check if lock is stale (> 10 minutes old)
3. If stale, offer automatic removal
4. If fresh, list common culprits and wait/retry options
5. Exit code: 8 (Git error)

### No HEAD Reference  
**Trigger**: Repository has no commits
**Behavior**:
1. Detect missing HEAD reference
2. Explain need for initial commit
3. Provide exact command to create one
4. Suggest retry after commit
5. Exit code: 8 (Git error)

### Disk Full
**Trigger**: Write operations fail with ENOSPC
**Behavior**:
1. Detect disk space error
2. Calculate space needed (if possible)
3. Show available space on partition
4. Provide cleanup suggestions
5. Exit code: 9 (IO error)

### Permission Denied
**Trigger**: Cannot read/write Git files
**Behavior**:
1. Identify which file/directory has permission issues
2. Show current permissions
3. Suggest ownership/permission fixes
4. Warn if running as wrong user
5. Exit code: 9 (IO error)

## Implementation Plan

### Phase 1: Core Error System
1. Implement error classifier with pattern matching
2. Create context tracking system
3. Build solution generator for top 5 errors
4. Add basic error formatting

### Phase 2: Enhanced Messages
1. Implement all error message transformations
2. Add colored output for better readability
3. Create help system integration
4. Add progress indicators for long operations

### Phase 3: Recovery Mechanisms
1. Implement retry logic for transient errors
2. Add automatic recovery for safe operations
3. Create operation replay for failed commands
4. Add telemetry for error patterns

## Success Metrics
- 90% of errors provide actionable solutions
- Average time to resolve errors reduced by 50%
- User feedback indicates errors are helpful
- Support requests for errors reduced by 75%

## Related Documents
- [ADR-004: Error Handling Improvements](../decisions/ADR-004-error-handling-improvements.md) - Architecture decision
- [Error Handling Improvement Proposal](../proposals/error-handling-improvement.md) - Detailed proposal
- [Git Edge Cases Audit](../audits/git-edge-cases-audit.md) - Edge cases to handle
- [Test Double Audit](../audits/test-double-audit.md) - Testing approach