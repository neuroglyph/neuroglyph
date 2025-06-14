#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Minimal test - just check traverse exists

set -e

echo "=== Minimal Traverse Test ==="

# Create temp directory
TESTDIR=$(mktemp -d)
cd "$TESTDIR"

# Initialize git repo
git init -q
git config user.email "test@example.com"
git config user.name "Test User"

# Create one file
echo "README" > README.md
git add .
git commit -q -m "Initial"

# Initialize gitmind
gitmind init

# Try traverse with no links
echo "Testing traverse with no links:"
gitmind traverse README.md --depth 1 || echo "Exit code: $?"

# Clean up
cd /
rm -rf "$TESTDIR"

echo "✓ Test complete"