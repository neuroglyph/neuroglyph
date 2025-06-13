#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Benchmark gitmind performance in Docker

set -e

echo "=== GitMind Performance Benchmark ==="
echo

# Create temp directory
TESTDIR=$(mktemp -d)
cd "$TESTDIR"

# Initialize git repo
git init >/dev/null 2>&1
git config user.email "test@example.com"
git config user.name "Test User"

# Startup time
echo "Startup time (version command):"
time -p gitmind version
echo

# Init benchmark
echo "Init command:"
time -p gitmind init
echo

# Create test files
for i in {1..100}; do
    echo "# File $i" > "file$i.md"
done
git add . >/dev/null 2>&1
git commit -m "Add files" >/dev/null 2>&1

# Link creation benchmark
echo "Creating 100 links:"
time -p bash -c 'for i in {1..100}; do gitmind link file$i.md file$((i%10+1)).md >/dev/null; done'
echo

# List benchmark
echo "Listing all links:"
time -p gitmind list >/dev/null
LINK_COUNT=$(gitmind list | wc -l)
echo "Total links: $LINK_COUNT"
echo

# Status benchmark
echo "Status command with $LINK_COUNT links:"
time -p gitmind status >/dev/null
echo

# Memory usage
echo "Memory usage:"
ps aux | grep gitmind | grep -v grep || echo "Process too fast to measure!"

# Binary details
echo
echo "Binary size: $(ls -lh $(which gitmind) | awk '{print $5}')"
echo "Stripped size: $(file $(which gitmind) | grep -o 'stripped')"

# Clean up
cd /
rm -rf "$TESTDIR"