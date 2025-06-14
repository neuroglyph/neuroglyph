# SPDX-License-Identifier: Apache-2.0
# © 2025 J. Kirby Ross / Neuroglyph Collective

# Neuroglyph C Implementation Test Suite

This directory contains the test infrastructure for the Neuroglyph C implementation.

## Test Structure

### Test Categories

1. **Unit Tests** (`unit/`)
   - Individual component tests
   - Fast, focused tests for single functions/modules

2. **Integration Tests** (`integration/`)
   - Full command tests with Git operations
   - End-to-end scenarios
   - Security and regression tests

### Integration Test Scripts

#### Core Test Runners
- `run-all-tests.sh` - Master test runner that executes ALL test suites
- `test-all.sh` - Convenience wrapper that calls `run-all-tests.sh`
- `test.sh` - Core functionality tests (init, add, list, etc.)

#### Specialized Test Suites
- `test-traverse.sh` - Graph traversal functionality
- `test-traverse-simple.sh` - Basic traverse tests
- `test-traverse-ultra-simple.sh` - Minimal traverse test case
- `test-traverse-debug.sh` - Debug build traverse tests
- `test-path-validation.sh` - Security: path traversal prevention
- `test-regression.sh` - Regression tests for fixed bugs
- `test-depth-error.sh` - Error handling for depth limits
- `test-minimal.sh` - Minimal smoke test

#### Docker Runners (run from host)
- `docker-test.sh` - Runs tests in Docker container
- `fuzz-test.sh` - Runs AFL++ fuzzing in Docker
- `valgrind-test.sh` - Runs memory checks in Docker

### Docker Guards

All test scripts (except Docker runners) include Docker guards to ensure they only run inside Docker containers. This prevents accidental execution on your working repository.

## Running Tests

### From Project Root
```bash
make test           # Runs all tests in Docker
make test-quick     # Runs core tests only
make fuzz          # Runs fuzzing tests
make valgrind      # Runs memory checks
```

### From C Directory
```bash
docker compose run --rm test      # Runs all tests
./tests/integration/docker-test.sh  # Alternative way
```

### Test Execution Flow

1. `make test` or `docker compose run test`
2. Starts Docker container with test environment
3. Runs `run-all-tests.sh` inside container
4. Each test script:
   - Checks Docker guard (exits if not in Docker)
   - Creates isolated temp directory
   - Initializes Git repo for testing
   - Runs test scenarios
   - Cleans up

## Writing New Tests

1. Create new test script in `integration/`
2. Add SPDX header
3. Add Docker guard:
   ```bash
   # Source Docker guard - will exit if not in Docker
   SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
   source "$SCRIPT_DIR/../docker-guard.sh"
   ```
4. Add test to `run-all-tests.sh`
5. Ensure script is executable: `chmod +x your-test.sh`

## Test Principles

- **Always use real Git repos** - No mocking Git operations
- **Run in Docker only** - Protects your working repository
- **Test behavior, not implementation** - Focus on user-visible outcomes
- **One test file per feature** - Keep tests focused and organized
- **Clean up after tests** - Use temp directories, remove when done

## Continuous Integration

The test suite is designed to run in CI environments:
- All tests run in Docker for consistency
- Exit codes indicate success/failure
- Detailed output for debugging failures
- Memory leak detection with Valgrind
- Fuzz testing for security