<!-- OBSOLETE as of June 13, 2025 -->
<!-- We pivoted to pure C implementation instead. See ADR-009-c.md -->

# GitMind Rust → Go Migration Plan

**Status**: READY TO EXECUTE  
**Estimated Time**: 2 weeks  
**Goal**: Working Go MVP with same features as Rust prototype  

## Current State

- ✅ Rust prototype working (all 44 tests passing)
- ✅ Clear architecture (dependency injection, clean layers)
- ✅ Well-defined commands: init, link, list, unlink, check
- ✅ Test doubles for edge cases
- ⚠️  Git operations shell out to CLI (perfect - Go can do the same)

## Migration Principles

1. **Keep what works**: Architecture, commands, test cases
2. **Simplify what doesn't**: No more fighting gitoxide
3. **Ship incrementally**: Each day produces working code
4. **Document everything**: This is a learning opportunity

## Week 1: Core Implementation

### Day 1-2: Project Setup & Foundation
```bash
mkdir go
cd go
go mod init github.com/neuroglyph/gitmind
```

**Deliverables**:
- [ ] Basic project structure
- [ ] Core types: `Link`, `Error`, `Result`
- [ ] `init` command working
- [ ] Makefile for builds

### Day 3-4: Link Operations
**Deliverables**:
- [ ] `link` command with go-git or shell
- [ ] `list` command reading `.gitmind/links/`
- [ ] SHA calculation matching Rust
- [ ] Basic tests

### Day 5-6: Unlink & Check
**Deliverables**:
- [ ] `unlink` command with all variations
- [ ] `check` command with --fix flag
- [ ] JSON output mode
- [ ] Cross-platform builds

### Day 7: Testing & Polish
**Deliverables**:
- [ ] Port key test cases from Rust
- [ ] GitHub Actions workflow
- [ ] Binary releases for Linux/macOS/Windows
- [ ] Installation instructions

## Week 2: Extensions & Documentation

### Day 8-9: Python SDK Proof of Concept
```python
# gitmind-py/gitmind.py
import subprocess
import json

class GitMind:
    def list_links(self):
        result = subprocess.run(
            ['gitmind', 'list', '--json'],
            capture_output=True
        )
        return json.loads(result.stdout)
```

### Day 10-11: Documentation & Examples
- [ ] Migration story blog post
- [ ] Comparison: Rust vs Go implementation
- [ ] Extension API documentation
- [ ] Demo video with Go version

### Day 12-14: Future Architecture
- [ ] Design GraphQL/REST API spec
- [ ] Prototype Gonzai chaos module (Python)
- [ ] Document Rust optimization points
- [ ] Community call for contributors

## Code Migration Examples

### Rust → Go: Link Type
```rust
// Rust
pub struct Link {
    pub link_type: String,
    pub source: String,
    pub target: String,
    pub timestamp: i64,
}
```

```go
// Go
type Link struct {
    LinkType  string `json:"link_type"`
    Source    string `json:"source"`
    Target    string `json:"target"`
    Timestamp int64  `json:"timestamp"`
}
```

### Rust → Go: Error Handling
```rust
// Rust
cmd.execute().into_command_result()
```

```go
// Go
err := cmd.Execute()
if err != nil {
    return err
}
```

### Git Operations (Same Approach!)
```go
// Go - Just like our Rust "workaround"
cmd := exec.Command("git", "add", path)
cmd.Dir = workDir
return cmd.Run()

// Or use go-git when it makes sense
wt, _ := repo.Worktree()
_, err := wt.Add(path)
```

## Success Metrics

### Week 1
- [ ] All 5 commands working in Go
- [ ] Binaries under 10MB
- [ ] Tests passing on Windows/Mac/Linux
- [ ] Installation takes <30 seconds

### Week 2  
- [ ] Python can read GitMind graphs
- [ ] Community excited about polyglot approach
- [ ] Clear path for contributions
- [ ] Performance baseline established

## What We're NOT Doing

- ❌ Rewriting everything at once
- ❌ Adding new features during migration
- ❌ Optimizing prematurely
- ❌ Making it perfect

## What We ARE Doing

- ✅ Shipping working software
- ✅ Making it approachable
- ✅ Setting up for extensions
- ✅ Building momentum

## The Moment of Truth

```bash
# Day 7 smoke test
$ go build -o gitmind cmd/gitmind/main.go
$ ./gitmind init
$ ./gitmind link README.md docs/setup.md REFERENCES
$ ./gitmind list
REFERENCES: README.md -> docs/setup.md (2025-06-20 15:30:45)
```

If this works on Windows, macOS, and Linux by Day 7, we've won.

## Rally Cry

We're not abandoning Rust. We're being pragmatic. The core engine in Go gets us to users faster. When we need microsecond performance for the chaos engine, Rust will be waiting.

**Ship the tool. Build the platform. Make the statement.**

Let's go! 🚀