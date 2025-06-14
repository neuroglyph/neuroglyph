#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# © 2025 J. Kirby Ross / Neuroglyph Collective

set -uo pipefail  # Remove -e to allow tests to fail without exiting

# Source Docker guard - will exit if not in Docker
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../docker-guard.sh"

# Get to the C implementation root
cd "$(dirname "$0")/../.."

echo "=== Path Validation Security Tests ==="
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

FAILED=0
PASSED=0

# Test function
test_path() {
    local path="$1"
    local expected="$2"
    local description="$3"
    
    # Create a simple test program
    cat > test_path.c << 'EOF'
#include <stdio.h>
#include <string.h>
#include "gitmind.h"

// Test wrapper
int main(int argc, char** argv) {
    if (argc != 2) {
        fprintf(stderr, "Usage: %s <path>\n", argv[0]);
        return 1;
    }
    
    int result = gm_validate_link_path(argv[1]);
    printf("%d\n", result);
    return 0;
}
EOF
    
    # Compile it (include all sources except main.c to avoid duplicate main)
    if ! gcc -o test_path test_path.c src/gitmind.c src/link.c src/sha1.c src/path.c src/check.c src/status.c src/traverse.c -I./include -Wall -Wextra -Wno-format-truncation 2>&1; then
        echo -e "${RED}✗${NC} Compilation failed for test"
        rm -f test_path test_path.c
        exit 1
    fi
    
    # Run the test
    local result=$(./test_path "$path" 2>/dev/null || echo "-99")
    
    if [ "$result" = "$expected" ]; then
        echo -e "${GREEN}✓${NC} $description"
        ((PASSED++))
    else
        echo -e "${RED}✗${NC} $description"
        echo "  Path: '$path'"
        echo "  Expected: $expected, Got: $result"
        ((FAILED++))
    fi
    
    rm -f test_path test_path.c
}

echo "Testing path traversal patterns..."

# Valid paths
test_path "file.txt" "0" "Simple filename"
test_path "src/main.c" "0" "Relative path"
test_path "a/b/c/d.txt" "0" "Deep relative path"
test_path "./file.txt" "0" "Current directory prefix"

# Invalid paths - absolute
test_path "/etc/passwd" "-6" "Absolute path"
test_path "/home/user/file" "-6" "Absolute home path"

# Invalid paths - path traversal attempts
test_path "../file.txt" "-6" "Simple parent directory"
test_path "../../etc/passwd" "-6" "Multiple parent directories"
test_path "foo/../bar" "-6" "Traversal in middle"
test_path "foo/../../bar" "-6" "Double traversal in middle"
test_path "./../../etc/passwd" "-6" "Current then traversal"
test_path "foo/bar/../../../etc" "-6" "Complex traversal"
test_path "foo/.." "-6" "Traversal at end"
test_path "../" "-6" "Just parent directory"
test_path "foo/bar/.." "-6" "Nested traversal"

# Tricky variants
test_path "..file.txt" "0" "Dots at start (valid)"
test_path "foo..bar" "0" "Dots in middle (valid)"
test_path "..." "0" "Three dots (valid)"
test_path "...." "0" "Four dots (valid)"
test_path "foo/..bar/baz" "0" "Dots after slash (valid)"

# URL encoding attempts
test_path "foo%2F..%2Fbar" "0" "URL encoded slashes (treated as literal)"
test_path "foo/../bar" "-6" "Real traversal for comparison"

# Windows-style paths (should be rejected due to backslashes)
test_path "C:\\file.txt" "-6" "Windows absolute (rejected - contains backslash)"
test_path "..\\file.txt" "-6" "Windows traversal (rejected - contains backslash)"

# Empty and special
test_path "" "-6" "Empty path"
test_path "." "0" "Current directory"

echo
echo "Summary: $PASSED passed, $FAILED failed"

if [ $FAILED -gt 0 ]; then
    exit 1
fi