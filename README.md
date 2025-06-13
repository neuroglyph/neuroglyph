<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- © 2025 J. Kirby Ross / Neuroglyph Collective -->

# ↯ Neuroglyph

> _"A glyph is a thought committed. A repo is a memory that persists."_

Neuroglyph is an open protocol and cognitive infrastructure layer that turns Git into a substrate for distributed semantic memory — no database, no gods, no masters.

[![CI](https://github.com/neuroglyph/neuroglyph/actions/workflows/ci.yml/badge.svg)](https://github.com/neuroglyph/neuroglyph/actions/workflows/ci.yml)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

It enables:
- ✅ Time-aware semantic linking of thoughts, files, and ideas
- 🧠 Git-powered knowledge graphs with zero external dependencies
- 🔗 Cross-repo, cross-temporal relationship mapping
- 🧪 Optional entropy injection via Chaos Mode for speculative discovery

Built for devs, researchers, writers, and AI agents—anyone who treats thought as infrastructure.

## 🧠 What is Neuroglyph?

**Git as cognition layer.**  
Neuroglyph transforms Git from a version control system into a **thinking system**.

You can:
- Link paper.md ➝ implementation.rs semantically (IMPLEMENTS)
- Store links as immutable Git objects (“glyphs”)
- Annotate with evolving metadata via Git Notes
- Visualize your repo as a **living semantic graph**

Neuroglyph doesn’t store _documents_.  
It stores **relationships, confidence**, and **epistemic state**.

> This is version control for understanding itself.

## ✨ Core Features
- 🔗 gitmind link A B — establish semlinks between concepts
- 📜 gitmind scan — extract links from markdown, wikis, and code
- 🧱 Git-native object storage — decentralized, content-addressable
- 🧠 Visualize the graph of meaning
- 🧬 Plugin-ready (VSCode, Obsidian, anything Git-aware)
- 🧨 Chaos Mode — Gonzai adds speculative links and uncanny insight

🧠 Monorepo Layout

```
neuroglyph/              # This monorepo
├── cli/                 # gitmind CLI (Rust)
├── demos/               # Example applications
├── design/              # Technical specifications
├── docs/                # User documentation
├── lore/                # Philosophy & Gonzai + Claude's Dev Logs
└── testdata/            # Test fixtures
```

Planned:
- glyphd/ — Optional daemon for Web UI
- plugins/ — Editor integrations
- sdk/ — Language bindings

## 🚀 Quick Start

### Prereqs

- Docker
- Git 2.40+
- Git LFS (brew install git-lfs)
- Rust 1.70+ (or use the Dockerized CLI)

### Setup

```bash
git clone https://github.com/neuroglyph/neuroglyph.git
cd neuroglyph
./scripts/setup-dev.sh
make test
make dev
```

### Basic Usage (Development)

```bash
cd cli
cargo build --release

# Initialize repo
./target/release/gitmind init

# Link files semantically
./target/release/gitmind link README.md docs/architecture.md

# List the current glyph graph
./target/release/gitmind list
```

## 🧪 Dev Workflow

All dev runs in Docker for consistency:

```bash
make dev          # Dev container shell
make test         # Full suite
make test-quick   # Unit tests only
make fmt          # Format
make clippy       # Lint
```

Pre-push hooks enforce tests and style.

## 📚 Learn More

- [Technical Roadmap](docs/README.md)
- [Architecture Diagrams](design/gitmind_architecture.md)
- [Semlink Protocol Spec](design/features/F001-git-object-storage.md)
- [The Gonzai Engine](lore/GONZAI_PERSONALITY.md)
- [Use Cases](docs/archive/use-cases.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Early Testers Program](docs/early-testers.md)

Explore `docs/ideas` and `design/` for lots of goodies.

## 🗺️ Roadmap

- [x] Phase 0: Repository setup and documentation
- [x] Phase 1: MVP CLI (init, link, list)
- [ ] Phase 2: Full CLI implementation  
- [ ] Phase 3: Web visualization
- [ ] Phase 4: Chaos engine
- [ ] Phase 5: Launch preparation

See [TASKLIST.md](TASKLIST.md) for detailed implementation plan.  
Also [design/README](design/README.md).

## 🌐 Project Ecosystem

- [@neuroglyph on GitHub](https://github.com/neuroglyph)
- [Gonzai, the Semantic Ape](lore/GONZAI_PERSONALITY.md) 🐵💥

## 🧑‍💻 Contributing

This project is open to dreamers, hackers, historians, poets, and systems architects.
See [CONTRIBUTING.md](CONTRIBUTING.md) for how to get involved.

**For AI assistants**: See [CLAUDE.md](CLAUDE.md) for important project conventions.

**Key points:**
- All tests run in Docker
- Pre-push hooks ensure code quality
- Follow conventional commits
- GPG keys strictly required

## ⚖️ License

Apache License 2.0 — see [LICENSE](LICENSE) for details

## 🙏 Acknowledgments

- Inspired by the realization that _everythin's a graph, bro_ and that *Git is already a graph database*
- Made for anyone who *believes knowledge deserves version control*
- Special thanks to Gonzai 🐵 for keeping things chaotic
- Git notes

> Neuroglyph is not just software. It’s an operating system for memory.  
> It doesn’t tell you what’s true. It tells you what connects.
