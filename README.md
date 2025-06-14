<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- © 2025 J. Kirby Ross / Neuroglyph Collective -->

[![CI](https://github.com/neuroglyph/neuroglyph/actions/workflows/ci.yml/badge.svg)](https://github.com/neuroglyph/neuroglyph/actions/workflows/ci.yml)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
# ↯ Neuroglyph

> _"A glyph is a thought committed. A repo is a memory that persists."_

## ⚡️ _What is Neuroglyph?_

Neuroglyph is an open protocol and cognitive infrastructure layer that turns Git into a substrate for distributed semantic memory — no database, no gods, no masters.

Semantic scaffolding for planetary-scale cooperative cognition. A map of thought across minds.
### 🧠 **Okay, So... What Does That Mean?**

| Level | Audience | Explanation | The "Aha!" |
|--------|----------------|-------------|-------------|
| 📱 | **My Mom** | *"It's like a smart assistant that remembers how your files connect. When you're reading something, it shows you what else it's related to."* | "Oh! I don’t have to remember everything myself!" |
| 👥 | **Teams** | *"When we share files, we also share the connections between them. Everyone sees the same web of context — so nothing gets lost, and onboarding is instant."* | "We're sharing context, not just content!" |
| ⚙️ | **Developers** | *"It’s Git — but for semantic relationships. Each link is a versioned Git object. No database. Your file tree IS the graph."* | "Wait... Git can do THAT?" |
| 🏗️ | **Architects** | *"Replace your knowledge stack with the file system. Relationships are files. Insights are commits. Understanding becomes infrastructure."* | "This removes entire categories of software!" |
| 🧠 | **Researchers** | *"Version-controlled epistemology. Fork ideas. Diff beliefs. Merge insights. Understand how understanding itself evolves."* | "We can literally version knowledge itself." |
| 🌍 | **Visionaries** | *"Distributed cognition substrate. Repos become neurons. Memory becomes shareable. Thought becomes forkable. Humanity begins to think as one."* | "This is how collective intelligence actually works." |

It enables:
- ✅ Time-aware semantic linking of thoughts, files, and ideas
- 🧠 Git-powered knowledge graphs with zero external dependencies
- 🔗 Cross-repo, cross-temporal relationship mapping
- 🧪 Optional entropy injection via Chaos Mode for speculative discovery

Built for devs, researchers, writers, and AI agents—anyone who treats thought as infrastructure.

### **Git as cognition layer** 

Neuroglyph transforms Git from a version control system into a **thinking system**.

You can:
- Link paper.md ➝ implementation.rs semantically (IMPLEMENTS)
- Store links as immutable Git objects (“glyphs”)
- Annotate with evolving metadata via Git Notes
- Visualize your repo as a **living semantic graph**

Neuroglyph doesn’t store _documents_.  
It stores **relationships, confidence**, and **epistemic state**.

> This is version control for understanding itself.

## ✨ What's Working Today

**Current Status:** Core CLI complete in pure C (67KB binary!)

### Available Commands
- `gitmind init` — Initialize semantic links in your repo
- `gitmind link A B --type TYPE` — Create semantic relationships
- `gitmind list [--source FILE]` — Query your knowledge graph  
- `gitmind unlink A B` — Remove specific links
- `gitmind check [--fix]` — Find and fix broken links
- `gitmind status` — See graph statistics
- `gitmind version` — Version info

### Coming Soon (MVP Sprint - June 2025)
- 🚧 `gitmind traverse FILE --depth N` — Explore connections (in development)
- 🚧 `gitmind serve` — Local web visualization (in development)
- 🚧 Interactive D3.js graph view (in development)

### Future Vision
- 📜 Query languages (SQL, Cypher, Natural)
- 🧬 Plugin ecosystem (VSCode, Obsidian)
- 🧨 Chaos Mode with Gonzai 🐵

---

## 🧠 Monorepo Layout

```
neuroglyph/              # This monorepo
├── c/                   # git-mind CLI (Pure C, 130KB binary)
├── demos/               # Example applications
├── design/              # Technical design documentation
│   ├── features/        # Feature specs (active/planned/completed)
│   ├── decisions/       # Architecture Decision Records (ADRs)
│   ├── proposals/       # Design proposals
│   └── research/        # Research documents
├── docs/                # User documentation
├── lore/                # Philosophy & Gonzai + Claude's Dev Logs
├── project/             # Project management
│   ├── community/       # Community guidelines
│   ├── legal/           # Legal & security docs
│   └── meta/            # Project metadata
└── scripts/             # Development scripts
```

Coming Soon:
- `gitmind traverse` — Graph exploration command
- `gitmind serve` — Web visualization server
- Query languages — SQL/Cypher/Natural language support

---

## ⚡ Performance

GitMind is **the fastest knowledge graph tool on Earth**. Not a claim—a fact.

### Benchmarks

```bash
Binary size:        130KB     (0.13MB - still smaller than this README!)
Startup time:       <1ms      ("Process too fast to measure!")
Memory usage:       ~500KB    (About the size of a medium blog post)
Dependencies:       Zero      (Just libc—already on your system)

# Operations (tested on 100 links)
Create link:        1.6ms     (11% faster with LTO!)
List all links:     <1ms  
Check integrity:    <1ms
```

### Size Comparison

| Tool | Size | Startup | Runtime |
|------|------|---------|---------|
| **GitMind** | **130KB** | **<1ms** | **None** |
| Obsidian | 150MB+ | ~2s | Electron |
| Roam Research | Web app | Network latency | Browser |
| Neo4j Desktop | 200MB+ | ~3s | JVM |
| Logseq | 200MB+ | ~3s | Electron |

That's not a typo. We're **1,000x smaller** than Electron apps. By the time their splash screen loads, you've already:
- Created 100 links
- Queried your entire graph  
- Made coffee
- Questioned why anyone uses Electron
  
### The Secret

Pure C. No runtime. No GC. No framework. Just focused, efficient code that respects your CPU cycles.

### 🌀 Speed as Philosophy

GitMind doesn't just start fast — it thinks fast.  
Because **every delay is a tax on cognition.**

- No splash screens. No spinners. No loading bars.
- Your graph is ready before your thoughts finish forming.
- It's not just fast. It's *telepathic*.

While Electron apps are figuring out if you're online, **GitMind already refactored your brain.**

### 🏁 Sonic Challenge

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

**→ See the [Quick Start Guide](docs/QUICK_START.md) for detailed instructions!**

### Install Binary (Recommended)

```bash
# Quick install script
curl -fsSL https://raw.githubusercontent.com/neuroglyph/neuroglyph/main/install.sh | bash

# Or download directly from releases:
# https://github.com/neuroglyph/neuroglyph/releases/latest
```

### Build from Source

Prerequisites:
- Docker (for development)
- Git 2.40+

```bash
git clone https://github.com/neuroglyph/neuroglyph.git
cd neuroglyph/c  # Enter the C implementation directory
make build       # Builds in Docker
# Binary is now at ./gitmind
```

### Basic Usage

```bash
# Initialize in any git repo
gitmind init

# Create semantic links
gitmind link README.md docs/api.md --type IMPLEMENTS
gitmind link design.md implementation.c --type REFERENCES

# View your knowledge graph
gitmind list                     # Show all links
gitmind list --source README.md  # What does README link to?
# Note: --target filter coming soon

# Keep links healthy
gitmind check        # Find broken links
gitmind check --fix  # Remove broken links
gitmind status       # Repository overview
```

## 🐳 Why Docker for Development?

**All development and testing *MUST* run through Docker.** The compiled `gitmind` binary runs natively without Docker, but development is Docker-only. Here's why:

### 1. Zero Setup Issues
No more *"works on my machine"* problems. Whether you're on macOS, Linux, or that one person still using Windows, Docker ensures everyone has the exact same C compiler, same libraries, same everything. You clone, you `make test`, it works. Period.

### 2. Tests Use REAL Git Operations
Our test suite creates actual Git repositories, makes real commits, and performs genuine Git operations. Running these tests on your working repository would be catastrophic:
- Tests could corrupt your actual work
- Git operations might conflict with your current branch
- You'd lose uncommitted changes
- Your `.git` directory could get mangled

Docker provides isolated, ephemeral Git environments where tests can safely:
- Create and destroy repositories
- Make commits without affecting your work
- Test edge cases that would be dangerous locally
- Run Git operations in parallel without conflicts

**This is why the pre-push hook runs tests in Docker** — it's not just convenience, it's safety.

## 🧪 Dev Workflow

Development happens in component directories:

```bash
cd c/             # Enter C implementation
make dev          # Dev container shell
make test         # Full test suite
make benchmark    # Performance tests
make build        # Build binary in Docker
```

Pre-push hooks enforce tests and style.
See [MONOREPO.md](project/meta/MONOREPO.md) for repository structure.

## 📚 Learn More

- [Technical Roadmap](docs/README.md)
- [Architecture Overview](design/ARCHITECTURE.md)
- [Semlink Protocol Spec](design/features/completed/F001-git-object-storage.md)
- [The Gonzai Engine](lore/GONZAI_PERSONALITY.md)
- [Use Cases](project/meta/archive/use-cases.md)
- [Contributing Guide](project/community/CONTRIBUTING.md)
- [Early Testers Program](project/community/early-testers.md)

Explore `docs/ideas` and `design/` and, of course, `lore/` for lots of goodies. We try to track everything, including rationale behind important decisions, ideas, and we honor our digital  collaborators by granting them space to keep a dev log (see `lore/claude-mind`) and encouraging them to write freely to it.

## 🗺️ Roadmap

- [x] Phase 0: Repository setup and documentation
- [x] Phase 1a: Core CLI (init, link, list, unlink, check, status) ✅
- [x] Phase 1b: Pure C implementation (67KB binary!) ✅
- [ ] Phase 1c: Graph traversal & web visualization (June 2025)
- [ ] Phase 2: Query languages & advanced features
- [ ] Phase 3: Plugin ecosystem
- [ ] Phase 4: Chaos engine with Gonzai 🐵

See [TASKLIST.md](project/meta/TASKLIST.md) for detailed implementation plan.  
Also [design/README.md](design/README.md) for technical documentation.

## 🌐 Project Ecosystem

- [@neuroglyph on GitHub](https://github.com/neuroglyph)
- [Gonzai, the Semantic Ape](lore/GONZAI_PERSONALITY.md) 🐵💥

## 🧑‍💻 Contributing

This project is open to dreamers, hackers, historians, poets, and systems architects.
See [CONTRIBUTING.md](project/community/CONTRIBUTING.md) for how to get involved.

**For AI assistants**: See [CLAUDE.md](project/meta/CLAUDE.md) for important project conventions.

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

Neuroglyph is **a substrate for shared memory** — human, machine, or both.

---

<details>
<summary>🐵 Secret Gonzai Message</summary>

> "If you are reading this, you're already part of the graph."  
> — G🧠NZAI

</details>

> Neuroglyph is not just software. It’s an operating system for memory.  
> It doesn’t tell you what’s true. It tells you what connects.
