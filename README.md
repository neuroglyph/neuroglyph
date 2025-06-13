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

# 🧠 Okay, But What Am I Looking At Here?

| Level | Audience | Explanation | The "Aha!" |
|--------|----------------|-------------|-------------|
| 📱 | **My Mom** | *"It's like a smart assistant that remembers how your files connect. When you're reading something, it shows you what else it's related to."* | "Oh! I don’t have to remember everything myself!" |
| 👥 | **Teams** | *"When we share files, we also share the connections between them. Everyone sees the same web of context — so nothing gets lost, and onboarding is instant."* | "We're sharing context, not just content!" |
| ⚙️ | **Developers** | *"It’s Git — but for semantic relationships. Each link is a versioned Git object. No database. Your file tree IS the graph."* | "Wait... Git can do THAT?" |
| 🏗️ | **Architects** | *"Replace your knowledge stack with the file system. Relationships are files. Insights are commits. Understanding becomes infrastructure."* | "This removes entire categories of software!" |
| 🧠 | **Researchers** | *"Version-controlled epistemology. Fork ideas. Diff beliefs. Merge insights. Understand how understanding itself evolves."* | "We can literally version knowledge itself." |
| 🌍 | **Visionaries** | *"Distributed cognition substrate. Repos become neurons. Memory becomes shareable. Thought becomes forkable. Humanity begins to think as one."* | "This is how collective intelligence actually works." |

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
├── c/                   # gitmind CLI (Pure C implementation)
├── demos/               # Example applications
├── docs/                # All documentation (specs, features, decisions)
├── lore/                # Philosophy & Gonzai
└── testdata/            # Test fixtures
```

Future additions:
- `glyphd/` - Optional daemon for Web UI
- `plugins/` - Editor integrations
- `sdk/` - Language bindings

---

## ⚡ Performance

GitMind is **the fastest knowledge graph tool on Earth**. Not a claim—a fact.

### Benchmarks

```bash
Binary size:        67KB      (0.067MB)
Startup time:       <1ms      ("Process too fast to measure!")
Memory usage:       ~500KB    (Less than this README)
Dependencies:       Zero      (Just libc—already on your system)

# Operations (tested on 100 links)
Create link:        1.8ms
List all links:     <1ms  
Check integrity:    <1ms
```

### Size Comparison

| Tool | Size | Startup | Runtime |
|------|------|---------|---------|
| **GitMind** | **67KB** | **<1ms** | **None** |
| Obsidian | 150MB+ | ~2s | Electron |
| Roam Research | Web app | Network latency | Browser |
| Neo4j Desktop | 200MB+ | ~3s | JVM |
| Logseq | 200MB+ | ~3s | Electron |

That's not a typo. We're **2,000x smaller** than Electron apps. By the time their splash screen loads, you've already:
- Created 100 links
- Queried your entire graph  
- Made coffee
- Questioned why anyone uses Electron

### The Secret

Pure C. No runtime. No GC. No framework. Just focused, efficient code that respects your CPU cycles.

---

## 🌀 Speed as Philosophy

GitMind doesn't just start fast — it thinks fast.  
Because **every delay is a tax on cognition.**

- No splash screens. No spinners. No loading bars.
- Your graph is ready before your thoughts finish forming.
- It's not just fast. It's *telepathic*.

While Electron apps are figuring out if you're online,  
**GitMind already refactored your brain.**

---

## 🏁 Sonic Challenge

We ran the numbers. Sonic lost.

| Task                         | GitMind | Sonic (at top speed) |
|------------------------------|---------|-----------------------|
| Startup                      | <1ms    | ~200ms reaction time  |
| 100 semantic links created   | ~180ms  | ~2 steps              |
| Full knowledge graph scan    | <1ms    | Didn't see it coming  |
| Memory usage                 | ~500KB  | 5 chili dogs          |

> *"Too slow." — GitMind, probably*

---

## 🚀 Quick Start

### Prerequisites

- Git 2.40+
- C compiler (gcc, clang, or compatible)
- Make
- Git LFS installed (`brew install git-lfs` on macOS) *[for logo/images in monorepo]*

Optional:
- Docker Desktop (for consistent testing environment)

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

### Basic Usage

```bash
# Build the CLI (C implementation)
cd c
make

# Install (optional)
sudo make install  # Installs to /usr/local/bin

# Use gitmind
gitmind init
gitmind link README.md docs/architecture.md --type IMPLEMENTS
gitmind list
gitmind status
gitmind check --fix  # Remove broken links
```

Or use the pre-built binary (when available):
```bash
# macOS/Linux
curl -L https://github.com/neuroglyph/neuroglyph/releases/latest/download/gitmind-$(uname -s)-$(uname -m) -o gitmind
chmod +x gitmind
./gitmind init
```

---

## 🧪 Development

Development can be done locally or in Docker:

```bash
# Local development
cd c
make          # Build binary
make test     # Run test suite
make clean    # Clean build artifacts

# Docker development (for consistency)
make docker-test    # Run tests in Docker
make docker-shell   # Development shell in Docker
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
- [x] Phase 1: MVP CLI (init, link, list, unlink, check, status) ✨
- [ ] Phase 2: Cross-platform distribution
- [ ] Phase 3: Web visualization
- [ ] Phase 4: Chaos engine
- [ ] Phase 5: World domination

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

## 📜 The GitMind Manifesto

We believe:

- Thought is infrastructure.
- Speed is cognition.
- Files are nodes. Git is truth.
- Bloat is a betrayal of clarity.
- Every idea deserves context.

Neuroglyph isn't software.  
It's **a substrate for shared memory** — human, machine, or both.

---

<details>
<summary>🐵 Secret Gonzai Message</summary>

> "If you are reading this, you're already part of the graph."  
> — G🧠NZAI

</details>

---

> _"Neuroglyph is not just software. It's the infrastructure for memory itself."_