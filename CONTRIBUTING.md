<!-- SPDX-License-Identifier: Apache-2.0 -->
# Contributing to Neuroglyph

Thank you for your interest in contributing to Neuroglyph! We're building a protocol for transforming Git into a substrate for distributed semantic memory, and we'd love your help.

## 🚀 Getting Started

### Prerequisites

- Git 2.40+
- C compiler (gcc, clang, or compatible)
- Make
- Basic understanding of Git internals is helpful but not required
- Docker Desktop (optional, for consistent testing environment)

### Development Setup

1. Fork and clone the repository:
   ```bash
   git clone https://github.com/YOUR_USERNAME/neuroglyph.git
   cd neuroglyph
   ```

2. Install Docker:
   - [Docker Desktop](https://www.docker.com/products/docker-desktop/) (recommended)
   - Ensure Docker is running

3. Install git hooks:
   ```bash
   make install-hooks
   ```

4. Build the project:
   ```bash
   cd c
   make
   ```

   Or use Docker:
   ```bash
   make docker-test
   ```

### Running Tests

All tests MUST be run in Docker to ensure consistency with CI:

```bash
# Run full test suite
cd c && make test

# Run tests in Docker (same as CI)
make docker-test

# For C development
cd c
make          # Build binary
make test     # Run tests
make clean    # Clean build artifacts
```

**Important:** The pre-push hook will automatically run `make test` before allowing pushes.

## 📝 Code Style Guidelines

### C Code

- Follow K&R or Linux kernel style
- Use meaningful variable and function names
- Always use `snprintf` instead of `sprintf`
- Check return values from system calls
- Free allocated memory appropriately
- Write tests for new functionality
- Document functions with clear comments

### Commit Messages

Follow the conventional commits format:
```
type(scope): subject

body (optional)

footer (optional)
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Test additions or changes
- `chore`: Build process or auxiliary tool changes

Example:
```
feat(cli): add gitmind init command

Implements the initialization of .gitmind directory structure
and creates necessary refs for semantic links.

Closes #42
```

## 🔄 Pull Request Process

1. **Create an issue first** - Discuss the change you wish to make
2. **Fork and create a branch** - Use a descriptive branch name
3. **Make your changes** - Follow the code style guidelines
4. **Add tests** - Ensure your changes are tested
5. **Update documentation** - If you changed behavior, update docs
6. **Run checks locally**:
   ```bash
   make test  # This runs all checks in Docker
   ```
7. **Submit PR** - Reference the issue and provide clear description

### PR Template

Your PR description should include:
- What problem does this solve?
- How does it solve it?
- What are the key changes?
- Testing performed
- Screenshots (if UI changes)

## 🧪 Testing Requirements

### Unit Tests
- Test individual functions and methods
- Mock external dependencies
- Aim for 80%+ code coverage

### Integration Tests
- Test CLI commands end-to-end
- Use temporary Git repositories
- Test error cases and edge conditions

### Example Test
```rust
#[test]
fn test_link_creation() {
    let temp_repo = TempRepo::new();
    let result = create_link("doc1.md", "doc2.md");
    assert!(result.is_ok());
    assert!(link_exists("doc1.md", "doc2.md"));
}
```

## 🎯 What to Work On

### Good First Issues
Look for issues labeled `good first issue` - these are ideal for newcomers.

### Priority Areas
1. **CLI Commands** - Implementing core functionality
2. **Performance** - Optimizing for large repositories
3. **Documentation** - Improving guides and examples
4. **Testing** - Increasing test coverage
5. **Cross-platform** - Ensuring Windows/macOS/Linux compatibility

## 💬 Communication

- **GitHub Issues** - For bug reports and feature requests
- **GitHub Discussions** - For questions and ideas
- **Pull Request Comments** - For code review discussions

## 🤝 Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md). We're committed to maintaining a welcoming and inclusive community.

## 📜 License

By contributing to Neuroglyph, you agree that your contributions will be licensed under the Apache License 2.0.

## 🙏 Recognition

Contributors will be recognized in:
- The project README
- Release notes
- Special thanks in documentation

Thank you for helping make Gitmind better! 🐵✨