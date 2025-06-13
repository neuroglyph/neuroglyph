# F013: CLI Tools & Developer Experience

**Status:** In Progress  
**Priority:** High  
**Complexity:** Medium  
**Estimation:** 6-7 days  
**Dependencies:** F001, F008

## Implementation Progress

- [x] Project setup with Rust and clap
- [x] Basic CLI structure
- [x] `gitmind init` command
- [x] `gitmind link` command
- [x] `gitmind list` command
- [ ] Advanced query operations

---

## Overview

Create a powerful command-line interface that makes Gitmind accessible to developers, enabling graph operations from terminal, shell integration, and pipeline automation.

## Core CLI Commands

### Basic Operations
```bash
# Initialize gitmind in repository
gitmind init

# Query the graph
gitmind query "find all related to AI"
gitmind query --gql 'MATCH (n)-[:REFERENCES]->(m) RETURN n,m'

# Create relationships
gitmind link README.md docs/api.md --type IMPLEMENTS
gitmind link --auto  # Auto-discover relationships

# Visualize
gitmind viz --browser  # Open web interface
gitmind viz --ascii    # Terminal visualization

# Gonzai interaction
gitmind gonzai suggest  # Get AI suggestions
gitmind gonzai chaos   # Activate chaos mode
```

### Advanced Features
```bash
# Time travel
gitmind at HEAD~10 query "what was connected"

# Bulk operations  
gitmind import links.csv
gitmind export --format cypher > graph.cypher

# Performance
gitmind stats
gitmind optimize
gitmind gc --aggressive

# Configuration
gitmind config gonzai.personality playful
gitmind config cache.size 1GB
```

## Shell Integration

### Zsh/Bash Completions
```bash
# Autocomplete for files
gitmind link <TAB>  # Shows files
gitmind query "find <TAB>  # Shows recent queries

# Fuzzy search integration
gitmind fzf  # Interactive node selection
```

### Git Aliases
```gitconfig
[alias]
    mind = !gitmind
    mindlog = !gitmind query "nodes modified in last commit"
    mindshow = !gitmind viz --focus
```

## Developer Experience

### REPL Mode
```typescript
class GitmindREPL {
  async start() {
    console.log(chalk.green('🐵 Gonzai: Welcome to Gitmind REPL!'));
    
    while (true) {
      const input = await readline('gitmind> ');
      
      if (input.startsWith('query')) {
        await this.handleQuery(input);
      } else if (input === 'chaos') {
        await this.activateChaosMode();
      }
      
      // Gonzai responds to commands
      this.gonzai.respond(input);
    }
  }
}
```

### Pipeline Integration
```yaml
# GitHub Actions
- name: Update Knowledge Graph
  run: |
    gitmind extract --changed-files
    gitmind test --verify-links
    gitmind push
    
# Pre-commit hook
repos:
  - repo: https://github.com/gitmind/gitmind
    hooks:
      - id: gitmind-extract
      - id: gitmind-validate
```

## ASCII Visualization
```
$ gitmind viz --ascii --depth 2

     README.md
    /    |    \
   /     |     \
api.md  doc.md  test.md
  |             /  \
  |           /     \
spec.md   unit.md  e2e.md

Nodes: 7 | Edges: 6 | Gonzai: 😊
```

## Key Features

1. **Scriptable API**: Full automation support
2. **JSON Output**: Machine-readable formats
3. **Watch Mode**: Auto-update on file changes
4. **Batch Mode**: Process multiple operations
5. **Plugin System**: Extend CLI functionality