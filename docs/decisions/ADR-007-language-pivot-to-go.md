# ADR-007: Pivot from Rust to Go

**Status**: SUPERSEDED by [ADR-009](ADR-009-c.md)  
**Date**: June 13, 2025  
**Deciders**: James, Claude  
**Note**: We ultimately pivoted to pure C instead. See [ADR-009](ADR-009-c.md) for details.  

## Context

We've implemented ~500 lines of Rust code for GitMind MVP. However, we've hit significant friction:

1. **Git library pain**: Both gitoxide and git2-rs have major limitations
2. **Shelling out**: We're calling `git` CLI for basic operations (add, commit, rm)
3. **Overengineering**: Using Rust for a tool that creates text files and calls Git
4. **Slow velocity**: Fighting the borrow checker for no real benefit

## Decision

**Pivot to Go** before we reach MVP.

## Rationale

### Why Go Wins

1. **Single Binary Distribution** ✅
   - Just like Rust, but simpler
   - Cross-compile trivially: `GOOS=windows go build`

2. **Excellent Git Story** ✅
   - `go-git` is actually mature and works
   - Shelling out is idiomatic in Go anyway
   - Used by many Git tools (hub, gh, lazygit)

3. **3x Faster Development** ✅
   - No borrow checker fights
   - Straightforward error handling
   - Excellent standard library

4. **Future-Proof** ✅
   - Built-in HTTP server for web UI
   - Goroutines for file watching
   - Channels for daemon mode

5. **Boring Technology** ✅
   - Mature, stable, well-documented
   - Easy to onboard contributors
   - Huge ecosystem

### Code Comparison

**Rust (current)**:
```rust
fn add(&self, working_dir: &Path, file_path: &str) -> Result<()> {
    // We gave up and shell out anyway
    let output = std::process::Command::new("git")
        .current_dir(working_dir)
        .args(["add", file_path])
        .output()
        .map_err(|e| Error::Git(format!("Failed to run git add: {}", e)))?;
    
    if !output.status.success() {
        return Err(Error::Git(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    Ok(())
}
```

**Go (proposed)**:
```go
func (g *GitOps) Add(path string) error {
    cmd := exec.Command("git", "add", path)
    cmd.Dir = g.workDir
    return cmd.Run()
}

// Or use go-git:
func (g *GitOps) Add(path string) error {
    _, err := g.worktree.Add(path)
    return err
}
```

## Consequences

### Positive
- Ship MVP in 1 week instead of 3
- Easier to maintain and extend
- Better library ecosystem
- Simpler CI/CD
- Happier developers

### Negative  
- Throw away 500 lines of Rust (sunk cost)
- Learn Go idioms (minor)
- Not as "cool" as Rust (who cares)

## Migration Plan

1. **Day 1-2**: Port core types and CLI structure
2. **Day 3-4**: Port all commands (init, link, list, unlink, check)
3. **Day 5**: Port tests
4. **Day 6-7**: Polish, documentation, release

Total: **1 week to feature parity**

## Alternatives Considered

- **Keep Rust**: Only if we were building libgit3
- **Python**: Great for prototyping, distribution hell for CLI
- **Node.js**: Wrong tool for systems programming
- **Bash**: Windows support nightmare

## References

- [Language Pivot Analysis](../research/language-pivot-analysis.md)
- [Why Go's git story is good](https://github.com/go-git/go-git)
- [Boring Technology](http://boringtechnology.club/)

## Decision

**Go with Go.** 🚀

We're not building a Git implementation. We're building a tool that creates link files and commits them. Let's use the right tool for the job.