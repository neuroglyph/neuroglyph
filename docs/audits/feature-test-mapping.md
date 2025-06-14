# Feature to Test Mapping

**Created:** June 14, 2025  
**Purpose:** Document which tests validate which features

## Test Suite Overview

The primary test suite is `c/tests/integration/test.sh` which runs 11 integration tests in an isolated Docker environment.

## Feature Coverage Map

### F001: Git Object Storage

| Test | What it Validates | Acceptance Criteria |
|------|-------------------|---------------------|
| Test 1 | `gitmind init` creates `.gitmind/links/` | Storage directory creation |
| Test 2 | Links stored as files | File-based storage model |
| Test 7 | SHA-based deduplication | Content-addressed storage |
| Test 8 | Git integration works | Links can be committed |

### F013: CLI Tools

| Test | What it Validates | Acceptance Criteria |
|------|-------------------|---------------------|
| Test 1 | `gitmind init` command | Initialize repository |
| Test 2 | `gitmind link` command | Create semantic links |
| Test 3 | `gitmind list` command | List all links |
| Test 4 | Multiple links supported | Handle many relationships |
| Test 5 | `--source` filtering | Query by source file |
| Test 6 | `gitmind unlink` command | Remove specific links |
| Test 9 | `gitmind status` command | Show repository stats |

### F016: Link Hygiene

| Test | What it Validates | Acceptance Criteria |
|------|-------------------|---------------------|
| Test 6 | `gitmind unlink` removes links | Manual link removal |
| Test 10 | `gitmind check` detects broken | Find missing targets |
| Test 11 | `gitmind check --fix` removes | Auto-cleanup broken links |

## Test Details

### Test 1: Init Command
```bash
gitmind init
# Validates: .gitmind/links directory created
```

### Test 2: Link Creation
```bash
gitmind link README.md docs/ARCHITECTURE.md --type IMPLEMENTS
# Validates: Link file created with SHA name
```

### Test 3: List Links
```bash
gitmind list
# Validates: Shows "IMPLEMENTS: README.md -> docs/ARCHITECTURE.md"
```

### Test 4: Multiple Links
```bash
gitmind link docs/ARCHITECTURE.md docs/api.md --type REFERENCES
# Validates: Can create and list multiple links
```

### Test 5: Filter by Source
```bash
gitmind list --source README.md
# Validates: Filtering works correctly
```

### Test 6: Unlink
```bash
gitmind unlink README.md docs/ARCHITECTURE.md
# Validates: Specific link removal
```

### Test 7: SHA Consistency
```bash
# Create same link twice
gitmind link README.md docs/api.md --type REFERENCES
# Validates: Deduplication via SHA naming
```

### Test 8: Git Integration
```bash
git add .gitmind && git commit -m "Add gitmind links"
# Validates: Links are Git-trackable
```

### Test 9: Status Command
```bash
gitmind status
# Validates: Shows "Total links: 2"
```

### Test 10: Check for Broken
```bash
rm docs/api.md && gitmind check
# Validates: Detects "Broken link"
```

### Test 11: Fix Broken Links
```bash
gitmind check --fix
# Validates: Removes broken links automatically
```

## Coverage Gaps

Features/criteria NOT covered by current tests:
- Link metadata beyond type and timestamp
- Performance with large numbers of links
- Edge cases (circular links, self-links)
- Error handling for permission issues
- Cross-platform compatibility

## Recommendations

1. Add performance benchmarks for 1000+ links
2. Test edge cases explicitly
3. Add unit tests for individual functions
4. Test error scenarios (no Git, bad permissions)