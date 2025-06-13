# SPDX-License-Identifier: Apache-2.0
# © 2025 J. Kirby Ross / Neuroglyph Collective
# Neuroglyph C Implementation Makefile
.PHONY: help build test clean install dev benchmark

# Default target
help:
	@echo "GitMind C Development Commands:"
	@echo "  make build        - Build the gitmind binary in Docker"
	@echo "  make test         - Run all tests in Docker"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make install      - Install to /usr/local/bin (from Docker build)"
	@echo "  make dev          - Open development shell in Docker"
	@echo "  make benchmark    - Run benchmarks in Docker"

# Build the binary in Docker
build:
	docker compose build dev
	docker compose run --rm dev sh -c "cd c && make clean && make"

# Run tests in Docker
test:
	docker compose build test
	docker compose run --rm -T test

# Clean build artifacts
clean:
	docker compose down
	rm -f c/gitmind c/src/*.o

# Development shell
dev:
	docker compose run --rm dev

# Run benchmarks in Docker
benchmark:
	docker compose build benchmark
	docker compose run --rm benchmark

# Install locally (build first in Docker, then copy)
install: build
	sudo cp c/gitmind /usr/local/bin/