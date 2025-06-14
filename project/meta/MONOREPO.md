# Neuroglyph Monorepo Structure

This is a monorepo containing multiple implementations and components of the Neuroglyph project.

## 📁 Repository Structure

```
neuroglyph/
├── c/           # Pure C implementation of gitmind CLI
├── demos/       # Example applications and demos
├── docs/        # Project-wide documentation
├── design/      # Technical specifications and ADRs
├── lore/        # Philosophy, mascot, and development journals
└── scripts/     # Repository-wide scripts
```

## 🚀 Getting Started

### For the C Implementation

```bash
cd c/
make help      # See available commands
make build     # Build in Docker
make test      # Run tests in Docker
make dev       # Open development shell
```

See [c/README.md](c/README.md) for detailed C implementation documentation.

### For Demos

```bash
cd demos/
# See demos/README.md for available demos
```

## 🏗️ Monorepo Principles

1. **Each component is self-contained** - Has its own Makefile, tests, and docs
2. **No root-level build tools** - The root doesn't favor any implementation
3. **Shared resources in common directories** - docs/, design/, lore/
4. **Component independence** - Each can be developed separately

## 📚 Documentation

- [Project Overview](README.md) - What is Neuroglyph?
- [Technical Roadmap](docs/README.md) - Architecture and features
- [C Implementation](c/README.md) - Pure C gitmind CLI
- [Contributing](../community/CONTRIBUTING.md) - How to contribute

## ⚡ Quick Links

- **Current Implementation**: [C](c/) - 130KB binary, zero dependencies
- **Future Implementations**: Rust, Go (planned)
- **Specifications**: [design/](design/)
- **Philosophy**: [lore/](lore/)