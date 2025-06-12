#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
# Development environment setup script

set -e

echo "🚀 Setting up Neuroglyph development environment..."
echo ""

# Check for Docker
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed!"
    echo "Please install Docker Desktop from: https://www.docker.com/products/docker-desktop/"
    exit 1
fi

# Check if Docker is running
if ! docker info >/dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker Desktop and try again."
    exit 1
fi

echo "✅ Docker is installed and running"

# Check for Git LFS
if ! command -v git-lfs &> /dev/null; then
    echo "⚠️  Git LFS is not installed!"
    echo "Install it with: brew install git-lfs (macOS) or see https://git-lfs.github.com"
    echo "Then run: git lfs install"
else
    echo "✅ Git LFS is installed"
fi

# Install git hooks
echo ""
echo "📎 Installing git hooks..."
make install-hooks

# Build Docker images
echo ""
echo "🏗️  Building Docker images..."
make docker-build

# Success message
echo ""
echo "✅ Development environment setup complete!"
echo ""
echo "Available commands:"
echo "  make dev        - Start development container"
echo "  make test       - Run all tests (same as CI)"
echo "  make test-quick - Run unit tests only"
echo "  make fmt        - Format code"
echo "  make clippy     - Run linter"
echo ""
echo "The pre-push hook is installed and will run tests before pushing."
echo ""
echo "Happy coding! 🐵✨"