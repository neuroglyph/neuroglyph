# GitMind Quick Start Guide

Get up and running with GitMind in under 5 minutes!

## Installation

### Option 1: Download Pre-built Binary (Fastest)

```bash
# Download the latest release
curl -LO https://github.com/neuroglyph/neuroglyph/releases/latest/download/gitmind-$(uname -s)-$(uname -m)

# Make it executable
chmod +x gitmind-*

# Move to your PATH
sudo mv gitmind-* /usr/local/bin/gitmind

# Verify installation
gitmind version
```

### Option 2: Build from Source

Prerequisites: Docker, Make

```bash
# Clone the repository
git clone https://github.com/neuroglyph/neuroglyph.git
cd neuroglyph

# Build with Docker (ensures consistency)
make build

# Binary will be at c/gitmind
sudo cp c/gitmind /usr/local/bin/

# Verify installation
gitmind version
```

## Basic Usage

### 1. Initialize GitMind in Your Repository

```bash
cd your-git-repo
gitmind init
```

This creates a `.gitmind/links/` directory to store your semantic links.

### 2. Create Your First Links

```bash
# Link your README to documentation
gitmind link README.md docs/API.md --type DOCUMENTS

# Link implementation to its spec
gitmind link src/parser.c docs/parser-spec.md --type IMPLEMENTS

# Link related files
gitmind link tests/auth.test.js src/auth.js --type TESTS
```

### 3. Explore Your Knowledge Graph

```bash
# List all links
gitmind list

# See what README.md links to
gitmind list --source README.md

# Check repository status
gitmind status
```

### 4. Maintain Link Health

```bash
# Check for broken links (after deleting files)
gitmind check

# Automatically remove broken links
gitmind check --fix
```

### 5. Remove Links

```bash
# Remove a specific link
gitmind unlink README.md docs/old-api.md
```

## Real-World Example

Here's a practical example for a typical project:

```bash
# Initialize
cd my-project
gitmind init

# Document your architecture
gitmind link README.md docs/architecture.md --type REFERENCES
gitmind link docs/architecture.md src/core/engine.js --type DOCUMENTS
gitmind link src/core/engine.js tests/engine.test.js --type TESTED_BY

# Track dependencies
gitmind link package.json README.md --type REFERENCED_BY
gitmind link src/api/server.js package.json --type DEPENDS_ON

# View the connections
gitmind list
# Output:
# REFERENCES: README.md -> docs/architecture.md
# DOCUMENTS: docs/architecture.md -> src/core/engine.js
# TESTED_BY: src/core/engine.js -> tests/engine.test.js
# REFERENCED_BY: package.json -> README.md
# DEPENDS_ON: src/api/server.js -> package.json

# See all connections from architecture doc
gitmind list --source docs/architecture.md
# Output:
# DOCUMENTS: docs/architecture.md -> src/core/engine.js
```

## Common Link Types

While you can use any type, here are some common patterns:

- `IMPLEMENTS` - Code implements a specification
- `DOCUMENTS` - Documentation describes code
- `REFERENCES` - General reference between files
- `DEPENDS_ON` - File depends on another
- `TESTS` - Test file tests implementation
- `INCLUDES` - File includes/imports another
- `RELATED_TO` - General relationship

## Git Integration

Links are just files, so they work with Git:

```bash
# Add links to your repository
git add .gitmind
git commit -m "Add semantic links for documentation"
git push

# Your teammates can now see the relationships!
```

## Performance

GitMind is blazing fast:
- **Startup**: <1ms
- **Create link**: ~2ms
- **List 100 links**: <1ms
- **Binary size**: 67KB

## Next Steps

1. **Coming Soon**: Graph traversal
   ```bash
   # This will let you explore connections
   gitmind traverse README.md --depth 3
   ```

2. **Coming Soon**: Web visualization
   ```bash
   # This will show an interactive graph
   gitmind serve
   # Open http://localhost:7432
   ```

## Tips

1. **Start Small**: Begin with a few key relationships
2. **Be Consistent**: Use consistent link types across your team
3. **Commit Links**: Add `.gitmind/` to version control
4. **Regular Cleanup**: Run `gitmind check` periodically
5. **Document Important Connections**: Focus on non-obvious relationships

## Troubleshooting

**"gitmind: command not found"**
- Make sure gitmind is in your PATH
- Try using the full path: `./gitmind`

**"Not a git repository"**
- GitMind requires a Git repository
- Run `git init` first

**"Permission denied"**
- Make sure the binary is executable: `chmod +x gitmind`
- Use `sudo` when copying to `/usr/local/bin`

## Get Help

- GitHub Issues: https://github.com/neuroglyph/neuroglyph/issues
- Documentation: https://github.com/neuroglyph/neuroglyph/tree/main/docs

---

That's it! You're now tracking semantic relationships in your codebase with GitMind. 🚀