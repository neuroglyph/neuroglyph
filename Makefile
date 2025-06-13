# SPDX-License-Identifier: Apache-2.0
# © 2025 J. Kirby Ross / Neuroglyph Collective
# Neuroglyph Go Implementation Makefile
.PHONY: help build test clean install dev fmt lint deps update-deps

# Default target
help:
	@echo "GitMind Go Development Commands:"
	@echo "  make build        - Build the gitmind binary"
	@echo "  make test         - Run all tests"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make install      - Install to GOPATH/bin"
	@echo "  make dev          - Build and run"
	@echo "  make fmt          - Format code"
	@echo "  make lint         - Run linter (requires golangci-lint)"
	@echo "  make deps         - Download dependencies"
	@echo "  make update-deps  - Update dependencies"

# Build the binary (for Linux in Docker)
build:
	docker compose run --rm dev sh -c "cd go && GOOS=linux GOARCH=amd64 go build -o ../gitmind ./cmd/gitmind"

# Build for host platform (macOS)
build-host:
	docker compose run --rm dev sh -c "cd go && GOOS=darwin GOARCH=arm64 go build -o ../gitmind ./cmd/gitmind"

# Run tests
test:
	docker compose run --rm -T test

# Clean build artifacts
clean:
	rm -f gitmind
	docker compose down
	docker volume rm neuroglyph_go-cache neuroglyph_go-build-cache || true

# Development shell
dev:
	docker compose run --rm dev

# Format code
fmt:
	docker compose run --rm dev sh -c "cd go && go fmt ./..."

# Run linter (will install golangci-lint in container)
lint:
	docker compose run --rm dev sh -c "cd go && golangci-lint run || echo 'Install golangci-lint first'"

# Download dependencies
deps:
	docker compose run --rm dev sh -c "cd go && go mod download"

# Update dependencies
update-deps:
	docker compose run --rm dev sh -c "cd go && go get -u ./... && go mod tidy"