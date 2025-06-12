<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- © 2025 J. Kirby Ross / Neuroglyph Collective -->

# ADR-001: Link Storage Model

**Status:** Accepted  
**Date:** June 2025  
**Decision:** Option B - Tracked files in `.gitmind/links/`

## Context

Neuroglyph needs to store semantic links between files in a Git-native way. We evaluated three options for F001 (Git Object Storage):

### Option A: Pure Git Objects (git hash-object)
- Store relationships as Git blobs using `git hash-object`
- Reference them with lightweight refs in `refs/semlinks/`
- Pros: Truly distributed, no working directory needed
- Cons: Complex querying, requires Git plumbing knowledge, harder to debug

### Option B: Tracked Files in `.gitmind/links/` ✅ CHOSEN
- Store each link as a file: `.gitmind/links/<sha>.link`
- Track these files normally in Git
- Pros: Human-readable, easy grep/search, simple implementation, visible in all Git tools
- Cons: Requires working directory, potential merge conflicts

### Option C: Hybrid (cache + git objects)
- Use Option B as primary storage
- Also create git objects for advanced use cases
- Pros: Best of both worlds
- Cons: More complex, two sources of truth

## Decision

We chose **Option B** for the MVP because:

1. **Simplicity**: Straightforward to implement and understand
2. **Debuggability**: Can browse links with standard tools (ls, cat, grep)
3. **Git-native**: Works with all existing Git workflows
4. **Migration path**: Easy to add Option C features later

## Implementation

Link format (one line per file):
```
LINK_TYPE: source_path -> target_path  # ts:timestamp [metadata]
```

Example:
```
CROSS_REF: README.md -> docs/architecture.md  # ts:1736637876
```

Filename is SHA-1 hash of the content for deterministic naming.

## Consequences

### Positive
- Quick to implement
- Easy to debug and inspect
- Works with all Git tools
- Simple merge conflict resolution

### Negative
- Requires checked-out working directory
- Potential for many small files
- Not as "pure" as git objects approach

### Mitigation
- Can evolve to Option C (hybrid) without breaking changes
- Small files are what Git excels at
- Working directory requirement aligns with CLI usage patterns