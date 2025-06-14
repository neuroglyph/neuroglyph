#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Run tests in Docker container

set -e

echo "Building Docker test image..."
docker build -f Dockerfile.test -t gitmind-test .

echo "Running tests in container..."
docker run --rm gitmind-test /build/test-all.sh