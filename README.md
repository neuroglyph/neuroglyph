<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- © 2025 J. Kirby Ross / Neuroglyph Collective -->

# ↯ Neuroglyph

> _"A glyph is a thought committed. A repo is a memory that persists."_

Neuroglyph is an open protocol and cognitive infrastructure layer that turns Git into a substrate for distributed semantic memory — no database, no gods, no masters.

Semantic scaffolding for planetary-scale cooperative cognition. A map of thought across minds.

# 🧠 Okay, But What Am I Looking At Here?

| Level | Audience | Explanation | The "Aha!" |
|--------|----------------|-------------|-------------|
| 📱 | **My Mom** | *"It's like a smart assistant that remembers how your files connect. When you're reading something, it shows you what else it's related to."* | "Oh! I don’t have to remember everything myself!" |
| 👥 | **Teams** | *"When we share files, we also share the connections between them. Everyone sees the same web of context — so nothing gets lost, and onboarding is instant."* | "We're sharing context, not just content!" |
| ⚙️ | **Developers** | *"It’s Git — but for semantic relationships. Each link is a versioned Git object. No database. Your file tree IS the graph."* | "Wait... Git can do THAT?" |
| 🏗️ | **Architects** | *"Replace your knowledge stack with the file system. Relationships are files. Insights are commits. Understanding becomes infrastructure."* | "This removes entire categories of software!" |
| 🧠 | **Researchers** | *"Version-controlled epistemology. Fork ideas. Diff beliefs. Merge insights. Understand how understanding itself evolves."* | "We can literally version knowledge itself." |
| 🌍 | **Visionaries** | *"Distributed cognition substrate. Repos become neurons. Memory becomes shareable. Thought becomes forkable. Humanity begins to think as one."* | "This is how collective intelligence actually works." |

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
├── c/                   # gitmind CLI (Pure C implementation)
├── demos/               # Example applications
├── docs/                # All documentation (specs, features, decisions)
├── lore/                # Philosophy & Gonzai + Claude's Dev Logs
└── testdata/            # Test fixtures
```

Planned:
- glyphd/ — Optional daemon for Web UI
- plugins/ — Editor integrations
- sdk/ — Language bindings

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

## 🚀 Quick Start

### Prereqs

- Docker
- Git 2.40+
- C compiler (gcc, clang, or compatible)
- Make
- Git LFS (brew install git-lfs) *[for logo/images in monorepo]*

Optional:
- the Dockerized CLI Desktop (for consistent testing environment)

### Setup

```bash
git clone https://github.com/neuroglyph/neuroglyph.git
cd neuroglyph
./scripts/setup-dev.sh
make test
make dev
```

### Basic Usage

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
- [ ] Phase 5: World domination

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
- Git notes

> Neuroglyph is not just software. It’s an operating system for memory.  
> It doesn’t tell you what’s true. It tells you what connects.
