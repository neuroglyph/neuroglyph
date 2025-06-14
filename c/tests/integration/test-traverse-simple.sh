#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Simple test for traverse command

set -e

# Source Docker guard - will exit if not in Docker
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../docker-guard.sh"

echo "=== Simple Traverse Test ==="

# Create temp directory
TESTDIR=$(mktemp -d)
cd "$TESTDIR"

# Initialize git repo
git init
git config user.email "test@example.com"
git config user.name "Test User"

# Create test files
echo "README" > README.md
echo "Doc" > doc.md
git add .
git commit -m "Initial"

# Initialize git-mind
git-mind init

# Create a simple link
git-mind link README.md doc.md --type REFERENCES

# List to verify
echo "Links created:"
git-mind list

# Try traverse with list format first
echo ""
echo "Testing traverse command (list format):"
git-mind traverse README.md --depth 1 --format list

echo ""
echo "Testing traverse command (tree format):"
git-mind traverse README.md --depth 1 --format tree

# Clean up
cd /
rm -rf "$TESTDIR"

echo "✓ Simple test complete"