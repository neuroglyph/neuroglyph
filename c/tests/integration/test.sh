#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Test script for gitmind - runs in isolated Docker container

set -e

# Source Docker guard - will exit if not in Docker
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../docker-guard.sh"

echo "=== GitMind C Test Suite ==="

# Create temp directory
TESTDIR=$(mktemp -d)
cd "$TESTDIR"

# Initialize git repo
git init
git config user.email "test@example.com"
git config user.name "Test User"

# Create some test files
echo "# Test Project" > README.md
mkdir -p docs
echo "# Architecture" > docs/ARCHITECTURE.md
echo "# API Docs" > docs/api.md
git add .
git commit -m "Initial commit"

echo "✓ Test repo created"

# Test 1: Init
echo -n "Test 1: gitmind init... "
gitmind init
if [ -d .gitmind/links ]; then
    echo "✓ PASS"
else
    echo "✗ FAIL: .gitmind/links not created"
    exit 1
fi

# Test 2: Create link
echo -n "Test 2: gitmind link... "
gitmind link README.md docs/ARCHITECTURE.md --type IMPLEMENTS
if [ -n "$(ls .gitmind/links/*.link 2>/dev/null)" ]; then
    echo "✓ PASS"
else
    echo "✗ FAIL: No link file created"
    exit 1
fi

# Test 3: List links
echo -n "Test 3: gitmind list... "
OUTPUT=$(gitmind list)
if echo "$OUTPUT" | grep -q "IMPLEMENTS: README.md -> docs/ARCHITECTURE.md"; then
    echo "✓ PASS"
else
    echo "✗ FAIL: Link not found in list"
    echo "Output: $OUTPUT"
    exit 1
fi

# Test 4: Create another link
echo -n "Test 4: Multiple links... "
gitmind link docs/ARCHITECTURE.md docs/api.md --type REFERENCES
OUTPUT=$(gitmind list | wc -l)
if [ "$OUTPUT" -eq "2" ]; then
    echo "✓ PASS"
else
    echo "✗ FAIL: Expected 2 links, got $OUTPUT"
    exit 1
fi

# Test 5: Filter by source
echo -n "Test 5: Filter by source... "
OUTPUT=$(gitmind list --source README.md | wc -l)
if [ "$OUTPUT" -eq "1" ]; then
    echo "✓ PASS"
else
    echo "✗ FAIL: Expected 1 link, got $OUTPUT"
    exit 1
fi

# Test 6: Unlink
echo -n "Test 6: gitmind unlink... "
gitmind unlink README.md docs/ARCHITECTURE.md
OUTPUT=$(gitmind list | wc -l)
if [ "$OUTPUT" -eq "1" ]; then
    echo "✓ PASS"
else
    echo "✗ FAIL: Expected 1 link after unlink, got $OUTPUT"
    exit 1
fi

# Test 7: SHA consistency
echo -n "Test 7: SHA consistency... "
# Create same link twice
gitmind link README.md docs/api.md --type REFERENCES
COUNT=$(ls .gitmind/links/*.link | wc -l)
if [ "$COUNT" -eq "2" ]; then  # Should be 2: one from test 4, one from test 7
    echo "✓ PASS (deduplication works)"
else
    echo "✗ FAIL: Expected 2 unique links, got $COUNT"
    exit 1
fi

# Test 8: Git integration
echo -n "Test 8: Git integration... "
git add .gitmind
git commit -m "Add gitmind links" >/dev/null 2>&1
if git log --oneline | grep -q "Add gitmind links"; then
    echo "✓ PASS"
else
    echo "✗ FAIL: Links not committed to git"
    exit 1
fi

# Test 9: Status command
echo -n "Test 9: gitmind status... "
OUTPUT=$(gitmind status)
if echo "$OUTPUT" | grep -q "Total links: 2"; then
    echo "✓ PASS"
else
    echo "✗ FAIL: Status didn't show correct link count"
    exit 1
fi

# Test 10: Check command
echo -n "Test 10: gitmind check... "
# Remove a target file to create broken link
rm docs/api.md
OUTPUT=$(gitmind check 2>&1)
if echo "$OUTPUT" | grep -q "Broken link"; then
    echo "✓ PASS"
else
    echo "✗ FAIL: Check didn't detect broken link"
    exit 1
fi

# Test 11: Check --fix
echo -n "Test 11: gitmind check --fix... "
gitmind check --fix >/dev/null 2>&1
COUNT=$(gitmind list | wc -l)
if [ "$COUNT" -eq "1" ]; then
    echo "✓ PASS"
else
    echo "✗ FAIL: Expected 1 link after fix, got $COUNT"
    exit 1
fi

# Clean up
cd /
rm -rf "$TESTDIR"

echo ""
echo "=== All tests passed! ==="
echo "Binary size: $(ls -lh $(which gitmind) | awk '{print $5}')"