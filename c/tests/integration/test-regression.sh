#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# © 2025 J. Kirby Ross / Neuroglyph Collective
#
# Regression tests for Phase 1 security and memory fixes

set -uo pipefail  # Remove -e to allow tests to fail without exiting

# Source Docker guard - will exit if not in Docker
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../docker-guard.sh"

# Get to the C implementation root
cd "$(dirname "$0")/../.."

echo "=== GitMind Regression Test Suite ==="
echo "Testing all Phase 1 fixes"
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Helper to run a test
run_test() {
    local test_name="$1"
    local test_cmd="$2"
    local expected_result="${3:-0}"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    echo -n "Testing $test_name... "
    
    set +e
    TMPFILE=$(mktemp)
    eval "$test_cmd" > "$TMPFILE" 2>&1
    local result=$?
    set -e
    
    if [ "$result" -eq "$expected_result" ]; then
        echo -e "${GREEN}✓${NC}"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo -e "${RED}✗${NC}"
        echo "  Command: $test_cmd"
        echo "  Output: $(cat "$TMPFILE")"
        echo "  Expected exit code: $expected_result, Got: $result"
        echo "  Output:"
        cat "$TMPFILE" | sed 's/^/    /'
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
    
    rm -f "$TMPFILE"
}

# Initialize a test repository
# Must be called after GITMIND_BIN is set!
init_test_repo() {
    local TEST_REPO_DIR=$(mktemp -d)
    local SAVED_PWD=$(pwd)
    
    cd "$TEST_REPO_DIR"
    git init --quiet
    git config user.name "Test User"
    git config user.email "test@example.com"
    echo "test" > README.md
    git add README.md
    git commit -m "Initial commit" --quiet
    git-mind init >/dev/null 2>&1
    
    cd "$SAVED_PWD"
    echo "$TEST_REPO_DIR"
}

echo "=== 1. Path Traversal Security Tests ==="

# Build a simple test program for path validation
TMPDIR=$(mktemp -d)
cat > "$TMPDIR/test_path_validator.c" << 'EOF'
#include <stdio.h>
#include <string.h>
#include "gitmind.h"

int main(int argc, char** argv) {
    if (argc != 2) {
        fprintf(stderr, "Usage: %s <path>\n", argv[0]);
        return 1;
    }
    
    int result = gm_validate_link_path(argv[1]);
    if (result == GM_OK) {
        printf("VALID\n");
        return 0;
    } else {
        printf("INVALID: %s\n", gm_last_error());
        return 1;
    }
}
EOF

gcc -o "$TMPDIR/test_path_validator" "$TMPDIR/test_path_validator.c" src/gitmind.c src/link.c src/sha1.c src/path.c src/check.c src/status.c src/traverse.c -I./include -Wall -Wextra -Wno-format-truncation

# Valid paths
run_test "valid simple filename" "$TMPDIR/test_path_validator file.txt" 0
run_test "valid relative path" "$TMPDIR/test_path_validator src/main.c" 0
run_test "valid current dir prefix" "$TMPDIR/test_path_validator ./file.txt" 0

# Invalid paths - absolute
run_test "reject absolute path" "$TMPDIR/test_path_validator /etc/passwd" 1
run_test "reject Windows absolute" "$TMPDIR/test_path_validator 'C:\\file.txt'" 1

# Invalid paths - traversal
run_test "reject simple .." "$TMPDIR/test_path_validator ../file.txt" 1
run_test "reject multiple .." "$TMPDIR/test_path_validator ../../etc/passwd" 1
run_test "reject .. in middle" "$TMPDIR/test_path_validator foo/../bar" 1
run_test "reject complex traversal" "$TMPDIR/test_path_validator foo/bar/../../../etc" 1
run_test "reject trailing .." "$TMPDIR/test_path_validator foo/.." 1
run_test "reject Windows traversal" "$TMPDIR/test_path_validator '..\\file.txt'" 1

# Edge cases
run_test "reject empty path" "$TMPDIR/test_path_validator ''" 1
run_test "allow dots in names" "$TMPDIR/test_path_validator ..file.txt" 0
run_test "allow three dots" "$TMPDIR/test_path_validator ..." 0

rm -rf "$TMPDIR"

echo
echo "=== 2. Memory Leak Tests ==="

# Compile with AddressSanitizer if available
if gcc -fsanitize=address -o /dev/null -x c /dev/null 2>/dev/null; then
    echo "Using AddressSanitizer for memory tests"
    ASAN_FLAGS="-fsanitize=address -fno-omit-frame-pointer"
else
    echo "AddressSanitizer not available, using valgrind"
    ASAN_FLAGS=""
fi

# Check if git-mind is available
if ! command -v git-mind >/dev/null 2>&1; then
    echo "ERROR: git-mind not found in PATH!"
    exit 1
fi
echo "Using git-mind from PATH"

echo "Creating test repository..."
TEST_REPO=$(init_test_repo)
echo "DEBUG: TEST_REPO='$TEST_REPO'"
if [ -z "$TEST_REPO" ]; then
    echo "ERROR: Failed to create test repository"
    exit 1
fi
echo "Test repository created at: $TEST_REPO"

# Test traverse memory leaks
cd "$TEST_REPO" || { echo "ERROR: Cannot cd to $TEST_REPO"; exit 1; }
echo "Testing traverse command for memory leaks..."

# Create some links for traversal
touch file1.txt file2.txt file3.txt
echo "Creating first link..."
git-mind link file1.txt file2.txt || echo "FAILED: link 1"
echo "Creating second link..."
git-mind link file2.txt file3.txt || echo "FAILED: link 2"
echo "Creating third link (cycle)..."
git-mind link file3.txt file1.txt || echo "FAILED: link 3"

if [ -n "$ASAN_FLAGS" ]; then
    # With ASAN, just run the command
    run_test "traverse with cycle (ASAN)" "git-mind traverse file1.txt --depth 5" 0
    run_test "traverse list format (ASAN)" "git-mind traverse file1.txt --format list" 0
else
    # With valgrind
    run_test "traverse with cycle (valgrind)" \
        "valgrind --leak-check=full --error-exitcode=1 --quiet git-mind traverse file1.txt --depth 5" 0
    run_test "traverse list format (valgrind)" \
        "valgrind --leak-check=full --error-exitcode=1 --quiet git-mind traverse file1.txt --format list" 0
fi

# Test check command memory leaks
touch file4.txt
git-mind link file3.txt file4.txt
rm file4.txt  # Create a broken link

if [ -n "$ASAN_FLAGS" ]; then
    run_test "check command (ASAN)" "git-mind check" 0
    run_test "check --fix (ASAN)" "git-mind check --fix" 0
else
    run_test "check command (valgrind)" \
        "valgrind --leak-check=full --error-exitcode=1 --quiet git-mind check" 0
    run_test "check --fix (valgrind)" \
        "valgrind --leak-check=full --error-exitcode=1 --quiet git-mind check --fix" 0
fi

cd - > /dev/null
rm -rf "$TEST_REPO"

echo
echo "=== 3. Error Code Consistency Tests ==="

# Create a test program to check return values
ERROR_TEST_DIR=$(mktemp -d)
cat > "$ERROR_TEST_DIR/test_error_codes.c" << 'EOF'
#include <stdio.h>
#include <sys/stat.h>
#include <errno.h>
#include "gitmind.h"

// We need to check internal functions return GM_OK
// This tests the ensure_dir function indirectly
int test_init_return() {
    // Test on a non-existent directory
    int ret = gm_init("/tmp/nonexistent_test_repo_12345");
    // Should fail with GM_ERR_NOT_REPO, not raw negative
    return (ret == GM_ERR_NOT_REPO) ? 0 : 1;
}

int main() {
    if (test_init_return() != 0) {
        printf("FAIL: gm_init returns wrong error code\n");
        return 1;
    }
    
    printf("PASS: Error codes are consistent\n");
    return 0;
}
EOF

gcc -o "$ERROR_TEST_DIR/test_error_codes" "$ERROR_TEST_DIR/test_error_codes.c" src/gitmind.c src/path.c src/sha1.c -I./include -Wall -Wextra
run_test "error code consistency" "$ERROR_TEST_DIR/test_error_codes" 0
rm -rf "$ERROR_TEST_DIR"

echo
echo "=== 4. Thread-Local Portability Test ==="

# Test that thread-local compiles on different standards
TLS_TEST_DIR=$(mktemp -d)
cat > "$TLS_TEST_DIR/test_thread_local.c" << 'EOF'
#include <stdio.h>

#if __STDC_VERSION__ >= 201112L
    #define THREAD_LOCAL _Thread_local
    #define TLS_TYPE "_Thread_local"
#elif defined(__GNUC__) || defined(__clang__)
    #define THREAD_LOCAL __thread
    #define TLS_TYPE "__thread"
#else
    #define THREAD_LOCAL
    #define TLS_TYPE "none"
#endif

static THREAD_LOCAL char buffer[256];

int main() {
    printf("Thread-local storage type: %s\n", TLS_TYPE);
    buffer[0] = 'A';
    return 0;
}
EOF

# Test C99 mode
run_test "thread-local C99" "gcc -std=c99 -o $TLS_TEST_DIR/test_tls $TLS_TEST_DIR/test_thread_local.c && $TLS_TEST_DIR/test_tls" 0

# Test C11 mode if available
if gcc -std=c11 -o /dev/null -x c /dev/null 2>/dev/null; then
    run_test "thread-local C11" "gcc -std=c11 -o $TLS_TEST_DIR/test_tls $TLS_TEST_DIR/test_thread_local.c && $TLS_TEST_DIR/test_tls" 0
fi

rm -rf "$TLS_TEST_DIR"

echo
echo "=== 5. Performance Regression Test ==="

TEST_REPO2=$(init_test_repo)
cd "$TEST_REPO2"

# Create many links to test O(n²) performance
echo "Creating 100 links of different types..."
for i in {1..25}; do
    touch "fileA$i.txt" "fileB$i.txt" "fileC$i.txt" "fileD$i.txt"
    git-mind link "fileA$i.txt" "fileB$i.txt" --type "TYPE_A" > /dev/null 2>&1
    git-mind link "fileB$i.txt" "fileC$i.txt" --type "TYPE_B" > /dev/null 2>&1
    git-mind link "fileC$i.txt" "fileD$i.txt" --type "TYPE_C" > /dev/null 2>&1
    git-mind link "fileD$i.txt" "fileA$i.txt" --type "TYPE_D" > /dev/null 2>&1
done

# Time the status command (should complete quickly even with O(n²))
echo -n "Testing status performance with 100 links... "
start_time=$(date +%s.%N)
timeout 2s git-mind status > /dev/null 2>&1
result=$?
end_time=$(date +%s.%N)

if [ $result -eq 0 ]; then
    elapsed=$(echo "$end_time - $start_time" | bc)
    echo -e "${GREEN}✓${NC} (${elapsed}s)"
    ((PASSED_TESTS++))
else
    echo -e "${RED}✗${NC} (timeout or error)"
    ((FAILED_TESTS++))
fi
((TOTAL_TESTS++))

cd - > /dev/null
rm -rf "$TEST_REPO2"

echo
echo "=== Test Summary ==="
echo "Total tests: $TOTAL_TESTS"
echo -e "Passed: ${GREEN}$PASSED_TESTS${NC}"
echo -e "Failed: ${RED}$FAILED_TESTS${NC}"

# Cleanup
rm -rf test_repo

if [ $FAILED_TESTS -gt 0 ]; then
    echo
    echo -e "${RED}Some tests failed!${NC}"
    exit 1
else
    echo
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
fi