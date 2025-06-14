#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Debug test for traverse command

set -e

# Source Docker guard - will exit if not in Docker
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../docker-guard.sh"

echo "=== Debug Traverse Test ==="

# Build with debug symbols
make clean
make debug

# Create temp directory
TESTDIR=$(mktemp -d)
cd "$TESTDIR"

# Initialize git repo
git init
git config user.email "test@example.com"
git config user.name "Test User"

# Create test files
echo "README" > README.md
git add .
git commit -m "Initial"

# Initialize gitmind
/build/gitmind init

# Create a simple link
echo "Doc" > doc.md
git add doc.md
git commit -m "Add doc"

/build/gitmind link README.md doc.md --type REFERENCES

# List to verify
echo "Links created:"
/build/gitmind list

# Try traverse with gdb if available
echo ""
echo "Testing traverse command:"
if command -v gdb &> /dev/null; then
    echo "run traverse README.md --depth 1" | gdb /build/gitmind -batch -ex "bt"
else
    /build/gitmind traverse README.md --depth 1 || echo "Exit code: $?"
fi

# Clean up
cd /
rm -rf "$TESTDIR"