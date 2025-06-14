#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Run Valgrind memory checks in Docker

set -e

echo "🔍 Building Valgrind test image..."
docker build -f Dockerfile.valgrind -t gitmind-valgrind .

echo "🧪 Running memory checks..."
docker run --rm gitmind-valgrind