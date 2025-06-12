# Neuroglyph Development Scripts

This directory contains scripts for development and CI/CD.

## Scripts

### setup-dev.sh
First-time setup for development environment:
```bash
./scripts/setup-dev.sh
```

### Git Hooks
- **pre-push-combined** - Runs Git LFS pre-push and then tests
- **post-checkout** - Git LFS hook for checking out files
- **post-commit** - Git LFS hook for post-commit processing
- **post-merge** - Git LFS hook for post-merge processing

All hooks are automatically installed by `make install-hooks`.

### Git LFS Setup
This project uses Git LFS for binary files (images, PDFs, etc.). To set up:
1. Install Git LFS: `brew install git-lfs` (macOS)
2. Run: `git lfs install`
3. Run: `make install-hooks`

## Docker-Based Testing

All tests run in Docker to ensure consistency between local development and CI. The same `make test` command runs:
- Locally via pre-push hook
- In GitHub Actions CI
- During development

This guarantees that if tests pass locally, they'll pass in CI.

## Testing Philosophy

1. **No local dependencies** - Everything runs in Docker
2. **Exact parity** - Local and CI run identical commands
3. **Fast feedback** - Pre-push hook catches issues early
4. **Easy setup** - One command to get started