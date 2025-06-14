#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Ultra simple test - just one file, no links

set -e

# Source Docker guard - will exit if not in Docker
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../docker-guard.sh"

echo "=== Ultra Simple Traverse Test ==="

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

# Try traverse on file with NO links - should show 0 connections
echo "Testing traverse with no links:"
gitmind traverse README.md --depth 1 --format list || echo "Exit code: $?"

echo ""
echo "Now create a link and try again:"
echo "Doc" > doc.md
git add doc.md
git commit -q -m "Add doc"

gitmind link README.md doc.md --type REFERENCES
echo "Link created. Now traverse:"
gitmind traverse README.md --depth 1 --format list || echo "Exit code: $?"

# Clean up
cd /
rm -rf "$TESTDIR"

echo "✓ Test complete"