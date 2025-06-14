#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Run all test suites for git-mind

set -e

# Source Docker guard - will exit if not in Docker
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/../docker-guard.sh"

echo "=== Running GitMind Test Suite ==="
echo ""

# Delegate to run-all-tests.sh which runs the complete test suite
./run-all-tests.sh