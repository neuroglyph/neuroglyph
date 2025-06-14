#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Run Valgrind memory checks in Docker

set -e

# If we're already in Docker, skip this test (valgrind is run by regression tests)
if [ -f /.dockerenv ] || [ "${DOCKER_CONTAINER:-}" = "1" ]; then
    echo "✅ Skipping standalone valgrind test (already in Docker, covered by regression tests)"
    exit 0
fi

echo "🔍 Building Valgrind test image..."
docker build -f ../../Dockerfile.valgrind -t git-mind-valgrind ../..

echo "🧪 Running memory checks..."
docker run --rm git-mind-valgrind