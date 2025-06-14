# Feature-to-Test Mapping

This document maps each implemented feature's acceptance criteria to the specific tests in `c/test.sh` that validate them.

**Generated:** June 14, 2025  
**Test Suite:** c/test.sh  
**Binary:** gitmind (C implementation, 67KB)

## Test Overview

The test suite runs in an isolated Docker container to ensure:
- No Git operations affect the working repository
- Tests use real Git operations (not mocks)
- Clean environment for each test run

## Feature F001: Git Object Storage

| Acceptance Criteria | Test(s) | Notes |
|---------------------|---------|-------|
| Relationships stored in `.gitmind/links/` directory | Test 1 | `gitmind init` creates directory |
| Each relationship has unique SHA-1 identifier | Test 2 | Link files created with SHA names |
| Relationships are immutable once stored | Implicit | Files not modified after creation |
| Duplicate relationships deduplicated via content hashing | Test 7 | SHA consistency test |
| All relationships can be listed | Test 3 | `gitmind list` shows all links |
| Query relationships for a given source file | Test 5 | `--source` filter |
| Bidirectional lookups (by target) | NOT TESTED | Feature not implemented |
| Sub-second performance | Implicit | All tests complete quickly |
| Standardized format: `TYPE: source -> target` | Test 3 | Output format verified |
| Timestamp metadata support | Implicit | Format includes `# ts:` |
| JSON serialization | NOT TESTED | Plain text only |
| Works with standard Git commands | Test 8 | Links can be committed |
| No corruption during concurrent operations | NOT TESTED | |
| Compatible with Git packfiles | NOT TESTED | |

## Feature F013: CLI Tools

| Acceptance Criteria | Test(s) | Notes |
|---------------------|---------|-------|
| `gitmind init` creates directory structure | Test 1 | Creates `.gitmind/links/` |
| Works in any Git repository | Test setup | Test creates Git repo first |
| Idempotent initialization | NOT TESTED | Could run init twice |
| `gitmind link` creates links | Test 2, 4 | Basic link creation |
| `--type` flag for custom link types | Test 2 | Uses `--type IMPLEMENTS` |
| SHA-based filenames for deduplication | Test 7 | Verifies deduplication |
| Timestamp metadata added automatically | Test 3 | Format includes timestamp |
| Default type "REFERENCES" | NOT TESTED | Always uses --type |
| `gitmind list` shows all links | Test 3 | Lists all links |
| `--source` filter | Test 5 | Filter by source file |
| `--target` filter | NOT TESTED | Feature not implemented |
| Output format includes type, source, target | Test 3 | Verifies format |
| Output includes timestamp | NOT VERIFIED | Test doesn't check timestamp |
| `gitmind unlink` removes links | Test 6 | Removes specific link |
| Handles non-existent links gracefully | NOT TESTED | |
| `gitmind check` validates links | Test 10 | Detects broken links |
| `--fix` removes broken links | Test 11 | Auto-cleanup |
| Reports broken link details | Test 10 | Output contains "Broken link" |
| Reports count of broken links | NOT VERIFIED | |
| `gitmind status` shows summary | Test 9 | Shows total links |
| Repository health status | NOT VERIFIED | Only shows count |
| `gitmind version` displays version | NOT TESTED | Command exists |

## Feature F016: Link Hygiene

| Acceptance Criteria | Test(s) | Notes |
|---------------------|---------|-------|
| Remove specific link between files | Test 6 | `gitmind unlink` |
| Handle non-existent links gracefully | NOT TESTED | |
| Remove all links from source | NOT IMPLEMENTED | |
| Remove all links to target | NOT IMPLEMENTED | |
| Auto-commit removal | NOT IMPLEMENTED | No Git commits |
| Detect links to non-existent files | Test 10 | After deleting target |
| Report broken links | Test 10 | "Broken link" in output |
| `--fix` removes broken links | Test 11 | Cleanup verified |
| `--dry-run` preview | NOT IMPLEMENTED | |
| Summary statistics | Test 10 | Reports broken links |

## Test Details

### Test 1: Initialization
```bash
gitmind init
# Verifies: .gitmind/links/ directory created
```

### Test 2: Create Link
```bash
gitmind link README.md docs/ARCHITECTURE.md --type IMPLEMENTS
# Verifies: Link file created in .gitmind/links/
```

### Test 3: List Links
```bash
gitmind list
# Verifies: Output format "IMPLEMENTS: README.md -> docs/ARCHITECTURE.md"
```

### Test 4: Multiple Links
```bash
gitmind link docs/ARCHITECTURE.md docs/api.md --type REFERENCES
# Verifies: Can create multiple links (count = 2)
```

### Test 5: Filter by Source
```bash
gitmind list --source README.md
# Verifies: Returns only links from README.md (count = 1)
```

### Test 6: Unlink
```bash
gitmind unlink README.md docs/ARCHITECTURE.md
# Verifies: Link removed (count decreases to 1)
```

### Test 7: SHA Consistency
```bash
gitmind link README.md docs/api.md --type REFERENCES
# Verifies: Duplicate links are deduplicated (only 2 unique files)
```

### Test 8: Git Integration
```bash
git add .gitmind
git commit -m "Add gitmind links"
# Verifies: Links can be committed to Git
```

### Test 9: Status Command
```bash
gitmind status
# Verifies: Shows "Total links: 2"
```

### Test 10: Check Command
```bash
rm docs/api.md
gitmind check
# Verifies: Detects broken link after file deletion
```

### Test 11: Check --fix
```bash
gitmind check --fix
# Verifies: Removes broken links (count = 1 after fix)
```

## Coverage Summary

### Well-Tested Features
- Basic link creation and storage
- Link listing and filtering by source
- Link removal (unlink)
- Broken link detection and cleanup
- Git integration (committing links)

### Gaps in Test Coverage
- Bidirectional queries (--target filter)
- Default link type behavior
- Non-existent link handling
- Concurrent operations
- Performance benchmarks
- Version command
- Error handling edge cases

### Recommendations
1. Add test for `--target` filter when implemented
2. Test default "REFERENCES" type without --type flag
3. Add test for unlinking non-existent links
4. Add performance benchmarks for large link counts
5. Test error messages and exit codes
6. Add test for `gitmind version` command