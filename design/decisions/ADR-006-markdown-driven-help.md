# ADR-006: Markdown-Driven Help System

**Status:** Accepted  
**Date:** June 12, 2025  
**Deciders:** James (Project Lead), Claude (Technical Advisor)  

## Context

During sprint planning, James proposed that we:
1. Add a `--help` flag to the CLI
2. Document the CLI in Markdown
3. Have `--help` display the markdown documentation in the terminal

This sparked a discussion about help systems, man pages, and documentation best practices for CLI tools.

## Decision

We will implement a markdown-driven help system that serves as the single source of truth for all documentation formats.

### Approach

1. **Primary Source**: Markdown files in `docs/cli/`
2. **Build-Time Generation**: Man pages from markdown
3. **Runtime Display**: Rendered markdown for `--help`
4. **Future Web Docs**: Same source for website

### Implementation Strategy

```
Markdown files → ┬→ CLI --help (embedded + rendered)
                 ├→ Man pages (generated at build)
                 └→ Web docs (static site generation)
```

## Rationale

### Why Markdown?

1. **Developer Friendly**: Everyone knows markdown
2. **Rich Content**: Code examples, lists, emphasis
3. **Version Control**: Diff-friendly text format
4. **Tooling**: Excellent editor support

### Why Not Direct Man Pages?

1. **troff is arcane**: Few developers know the syntax
2. **Hard to maintain**: Errors are hard to spot
3. **Poor diffs**: Markup obscures content changes
4. **Limited reuse**: Can't easily generate other formats

### Why Include Man Pages At All?

1. **Unix convention**: Users expect `man gitmind`
2. **Offline access**: No internet required
3. **System integration**: Works with `apropos`, `whatis`
4. **Professional polish**: Shows attention to detail

### Why Embed in Binary?

1. **Single file distribution**: No separate doc files
2. **Version synchronization**: Docs match binary
3. **Works anywhere**: No path resolution issues
4. **Fast startup**: No file I/O for help

## Consequences

### Positive
- **Single source of truth**: All docs from one place
- **Easy updates**: Just edit markdown
- **Consistent**: Same content everywhere
- **Discoverable**: Both `--help` and `man` work
- **Future-proof**: Can generate more formats

### Negative
- **Build complexity**: Need doc generation step
- **Binary size**: Embedded docs add ~50KB
- **Dependencies**: Markdown parser needed
- **Testing**: Must test rendering output

### Neutral
- **Learning curve**: Contributors must follow doc standards
- **Tooling choices**: Must pick markdown→man converter
- **Style guide**: Need consistent markdown conventions

## Implementation Plan

### Phase 1: Markdown + Basic Help
1. Create `docs/cli/` structure
2. Write markdown for all commands
3. Embed with `include_str!`
4. Basic terminal rendering

### Phase 2: Man Page Generation  
1. Add `clap_mangen` to build deps
2. Generate at build time
3. Include in releases
4. Test on Linux/macOS

### Phase 3: Rich Terminal Rendering
1. Add terminal capability detection
2. Color support for capable terminals  
3. Pager integration for long help
4. Respect NO_COLOR, TERM settings

## Example Structure

```
docs/cli/
├── gitmind.md           # Main command
├── gitmind-init.md      # gitmind init
├── gitmind-link.md      # gitmind link  
├── gitmind-list.md      # gitmind list
├── gitmind-unlink.md    # gitmind unlink
├── gitmind-check.md     # gitmind check
└── gitmind-serve.md     # gitmind serve (future)
```

## Success Metrics
- Help available offline via `--help` and `man`
- Documentation stays in sync with implementation
- Users report help is actually helpful
- Contributors find docs easy to update

## References
- [Markdown-Driven Help Proposal](../proposals/markdown-driven-help.md)
- [F025: CLI Help System](../features/F025-cli-help-system.md) (to be created)
- Examples: Git, Cargo, Docker CLI documentation systems

---

*"Documentation is a love letter that you write to your future self." - Damian Conway*