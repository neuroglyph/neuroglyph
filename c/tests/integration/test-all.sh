#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Run all test suites for gitmind

set -e

echo "=== Running GitMind Test Suite ==="
echo ""

# Run basic tests
echo ">>> Running basic tests..."
./test.sh
echo ""

# Run traverse tests
echo ">>> Running traverse tests..."
./test-traverse.sh
echo ""

echo "=== All test suites passed! ===🚀"