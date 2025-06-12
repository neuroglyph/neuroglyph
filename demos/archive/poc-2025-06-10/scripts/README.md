# Gitmind Test Environment

This directory contains the essential scripts and configuration for creating the Gitmind test environment.

## Scripts

- **extract-links.sh**: Extracts markdown links from files (used by git hooks)
- **chaos-worker.sh**: Generates continuous test data by creating/modifying/deleting files
- **setup-test-env.sh**: Sets up the complete test environment with three git repositories

## Docker Setup

### Quick Start

1. Build and run the test environment:
```bash
docker-compose -f docker-compose.test.yml up -d
```

2. Access the test environment:
```bash
docker exec -it gitmind-test-env bash
cd git-knowledge-test
```

3. Start the chaos worker (if not already running):
```bash
./chaos-worker.sh
```

### Manual Docker Commands

Build the image:
```bash
docker build -f Dockerfile.testenv -t gitmind-test .
```

Run interactively:
```bash
docker run -it --rm gitmind-test bash
```

Run with chaos worker:
```bash
docker run -d --name gitmind-chaos gitmind-test
```

## Local Development

If you need to run the test environment locally (not recommended for main repo):
```bash
./scripts/setup-test-env.sh
```

This will create a `git-knowledge-test` directory with the full test environment.

## Architecture

The test environment consists of:
- **vault/**: Personal knowledge repository
- **projects/**: Project documentation repository  
- **relationships/**: Extracted relationships storage
- Git hooks that automatically extract links on commit
- Chaos worker that continuously generates test data