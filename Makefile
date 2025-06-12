# SPDX-License-Identifier: Apache-2.0
# © 2025 J. Kirby Ross / Neuroglyph Collective
# Neuroglyph CLI Development Makefile
.PHONY: help dev test test-quick fmt clippy clean docker-build install-hooks

# Default target
help:
	@echo "Neuroglyph CLI (gitmind) Development Commands:"
	@echo "  make dev          - Start development container"
	@echo "  make test         - Run all tests in Docker (same as CI)"
	@echo "  make test-quick   - Run tests without format/clippy checks"
	@echo "  make fmt          - Run cargo fmt"
	@echo "  make clippy       - Run cargo clippy"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make docker-build - Build Docker images"
	@echo "  make install-hooks - Install git hooks"

# Start development environment
dev: docker-build
	docker compose run --rm dev

# Run all tests exactly as CI would
test: docker-build
	docker compose run --rm -T test

# Quick test run (no format/clippy)
test-quick: docker-build
	docker compose run --rm dev cargo test

# Format code
fmt: docker-build
	docker compose run --rm dev cargo fmt

# Run clippy
clippy: docker-build
	docker compose run --rm dev cargo clippy -- -D warnings

# Clean build artifacts
clean:
	docker compose down
	docker volume rm neuroglyph_target-cache || true
	rm -rf cli/target/

# Build Docker images
docker-build:
	docker compose build

# Install git hooks (including Git LFS)
install-hooks:
	@echo "Installing git hooks..."
	@mkdir -p .git/hooks
	@cp scripts/pre-push-combined .git/hooks/pre-push
	@cp scripts/post-checkout .git/hooks/post-checkout
	@cp scripts/post-commit .git/hooks/post-commit
	@cp scripts/post-merge .git/hooks/post-merge
	@chmod +x .git/hooks/pre-push
	@chmod +x .git/hooks/post-checkout
	@chmod +x .git/hooks/post-commit
	@chmod +x .git/hooks/post-merge
	@echo "Git hooks installed (including Git LFS support)!"