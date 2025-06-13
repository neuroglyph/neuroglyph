#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Run performance benchmarks in Docker

set -e

echo "Building Docker image..."
docker build -f Dockerfile.test -t gitmind-test . >/dev/null 2>&1

echo "Running benchmarks..."
docker run --rm gitmind-test /build/benchmark.sh