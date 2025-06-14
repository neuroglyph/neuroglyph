#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Simple test for traverse command

set -e

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

# Initialize gitmind
gitmind init

# Create a simple link
gitmind link README.md doc.md --type REFERENCES

# List to verify
echo "Links created:"
gitmind list

# Try traverse with list format first
echo ""
echo "Testing traverse command (list format):"
gitmind traverse README.md --depth 1 --format list

echo ""
echo "Testing traverse command (tree format):"
gitmind traverse README.md --depth 1 --format tree

# Clean up
cd /
rm -rf "$TESTDIR"

echo "✓ Simple test complete"