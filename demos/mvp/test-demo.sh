#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Test the demo setup entirely in Docker

set -e

echo "🐳 Building demo Docker image..."
docker build -t gitmind-demo .

echo "🚀 Running demo setup in Docker..."
docker run --rm -it gitmind-demo /home/demouser/setup-demo.sh

echo "✅ Demo test complete!"