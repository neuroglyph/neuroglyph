#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Test depth error message

set -e

# Source Docker guard - will exit if not in Docker
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../docker-guard.sh"

TESTDIR=$(mktemp -d)
cd "$TESTDIR"
git init -q
git config user.email "test@example.com"
git config user.name "Test User"
echo "README" > README.md
git add . && git commit -q -m "Initial"
gitmind init

echo "Testing depth 11:"
gitmind traverse README.md --depth 11 2>&1

cd /
rm -rf "$TESTDIR"