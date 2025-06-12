<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- © 2025 J. Kirby Ross / Neuroglyph Collective -->

# Neuroglyph

> _"A glyph is a thought committed. A repo is a memory that persists."_

**Neuroglyph** is a protocol and open-source system for transforming Git into a substrate for distributed semantic memory.

[![CI](https://github.com/neuroglyph/neuroglyph/actions/workflows/ci.yml/badge.svg)](https://github.com/neuroglyph/neuroglyph/actions/workflows/ci.yml)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

It enables:
- ✅ Time-aware **semantic linking** of files and ideas
- 🧠 Git-based **knowledge graphs** with zero external databases
- 🔗 Cross-repository, cross-temporal relationship mapping
- 🧪 Optional chaos-mode entropy for exploratory discovery

Built for devs, researchers, writers, AI agents, and anyone who treats thought as versioned infrastructure.

---

## 🧠 What Is Neuroglyph?
Neuroglyph is **Git as cognition layer.**

It lets you:
- Create links like `note.md ➝ spec.md` with semantic meaning
- Store these links as versioned Git objects ("glyphs")
- Query, visualize, and sync knowledge across time and devices
- Extend Git from version control to **mind control** 🧙

---

## ✨ Key Features

- 🔗 `gitmind link A B` — link two nodes with semantic intent
- 📜 `gitmind scan` — parse links from Markdown, wiki-style, more
- 🧱 Git-native storage — content-addressable, decentralized
- 🧠 Visualize your repo as a living semantic graph
- 🧬 Plugin-ready architecture (VSCode, Obsidian, etc.)
- 🧨 Chaos mode — inject entropy to discover novel connections

---

## 📦 Monorepo Structure

```
neuroglyph/              # This monorepo
├── cli/                 # gitmind CLI (Rust)
├── demos/               # Example applications
├── design/              # Technical specifications
├── docs/                # User documentation
├── lore/                # Philosophy & Gonzai
└── testdata/            # Test fixtures
```

Future additions:
- `glyphd/` - Optional daemon for Web UI
- `plugins/` - Editor integrations
- `sdk/` - Language bindings

---

## 🚀 Quick Start

### Prerequisites

- Docker Desktop installed and running
- Git 2.40+
- Git LFS installed (`brew install git-lfs` on macOS)
- Rust 1.70+ (or use Docker)

### Development Setup

```bash
# Clone the repository
git clone https://github.com/neuroglyph/neuroglyph.git
cd neuroglyph

# Set up development environment
./scripts/setup-dev.sh

# Run tests (in Docker, same as CI)
make test

# Start development container
make dev
```

### Basic Usage (Development)

```bash
# Build the CLI
cd cli
cargo build --release

# Use gitmind
./target/release/gitmind init
./target/release/gitmind link README.md docs/architecture.md
./target/release/gitmind list
```

---

## 🧪 Development

All development happens in Docker to ensure consistency:

```bash
make dev          # Start development container
make test         # Run full test suite (format, lint, tests)
make test-quick   # Run only unit tests
make fmt          # Format code
make clippy       # Run linter
```

A pre-push hook automatically runs tests before pushing.

---

## 📚 Learn More

- [Technical Roadmap](docs/README.md)
- [Architecture Diagrams](design/gitmind_architecture.md)
- [Semlink Protocol Spec](design/features/F001-git-object-storage.md)
- [The Gonzai Engine](lore/GONZAI_PERSONALITY.md)
- [Use Cases](docs/archive/use-cases.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Early Testers Program](docs/early-testers.md)

---

## 🗺️ Roadmap

- [x] Phase 0: Repository setup and documentation
- [ ] Phase 1: MVP CLI (init, link, list)
- [ ] Phase 2: Full CLI implementation  
- [ ] Phase 3: Web visualization
- [ ] Phase 4: Chaos engine
- [ ] Phase 5: Launch preparation

See [TASKLIST.md](TASKLIST.md) for detailed implementation plan.

---

## 🌐 Project Ecosystem

- [neuroglyph.dev](https://neuroglyph.dev) *(coming soon)*
- [@neuroglyph on GitHub](https://github.com/neuroglyph)
- [Gonzai, the Semantic Ape](lore/GONZAI_PERSONALITY.md) 🐵💥

---

## 🧑‍💻 Contributing

This project is open to dreamers, hackers, historians, poets, and systems architects.
See [CONTRIBUTING.md](CONTRIBUTING.md) for how to get involved.

**For AI assistants**: See [CLAUDE.md](CLAUDE.md) for important project conventions.

**Key points:**
- All tests run in Docker
- Pre-push hooks ensure code quality
- Follow conventional commits

---

## ⚖️ License

Apache License 2.0 — see [LICENSE](LICENSE) for details

---

## 🙏 Acknowledgments

- Inspired by the realization that Git is already a graph database
- Built with love for knowledge workers everywhere
- Special thanks to Gonzai 🐵 for keeping things chaotic

---

> _"Neuroglyph is not just software. It's the infrastructure for memory itself."_