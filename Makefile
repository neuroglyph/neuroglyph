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
	@echo "  make demo         - Run demo setup in Docker"
	@echo "  make fuzz         - Run AFL++ fuzzing tests"
	@echo "  make valgrind     - Run Valgrind memory checks"
	@echo "  make man          - Generate man pages from docs"

# Build the binary in Docker
build:
	cd c && docker compose build dev
	cd c && docker compose run --rm dev make clean all

# Run tests in Docker
test:
	cd c && docker compose build test
	cd c && docker compose run --rm -T test

# Clean build artifacts
clean:
	cd c && docker compose down
	rm -f c/gitmind c/src/*.o

# Development shell
dev:
	cd c && docker compose run --rm dev

# Run benchmarks in Docker
benchmark:
	cd c && docker compose build benchmark
	cd c && docker compose run --rm benchmark

# Install locally (build first in Docker, then copy)
install: build
	sudo cp c/gitmind /usr/local/bin/

# Run demo in Docker
demo:
	cd demos/mvp && ./test-demo.sh

# Run fuzz testing
fuzz:
	cd c && ./tests/integration/fuzz-test.sh

# Run memory checks
valgrind:
	cd c && ./tests/integration/valgrind-test.sh

# Generate man pages
man:
	cd docs/cli && docker build -f Dockerfile.man -t gitmind-man . && \
	docker run --rm -v "$$PWD/../../c:/output" gitmind-man