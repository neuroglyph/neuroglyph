<!-- SPDX-License-Identifier: Apache-2.0 -->
# Neuroglyph Implementation Task List

**Status:** Ready to begin Rust CLI implementation  
**Last Updated:** June 2025  
**Project:** Neuroglyph (monorepo)  
**Component:** gitmind CLI

---

## 🧹 Phase 0: Repository Hygiene & Baseline

### Clean Repository State
- [x] Archive `combined_markdown.md` to `/archive/` if it exists
- [x] Clean up any remaining test data or temporary files
- [x] Remove `/chaos-backup/` directory (already in .gitignore)
- [x] Verify no sensitive data in repository history

### Legal & Community Foundation
- [x] **Choose and add LICENSE file**
  - [x] Decision: MIT, Apache 2.0, or GPL?
  - [ ] Add license headers to source files
  - [x] Document license rationale in `/docs/decisions/`
- [x] Create `CONTRIBUTING.md` with:
  - [x] Development setup instructions
  - [x] Code style guidelines
  - [x] Pull request process
  - [x] Testing requirements
- [x] Add `CODE_OF_CONDUCT.md` (Contributor Covenant)
- [x] Create issue templates:
  - [x] Bug report
  - [x] Feature request
  - [x] Documentation improvement
- [x] Create pull request template

### Documentation Structure
- [x] Create clear documentation hierarchy:
  ```
  /docs/        # User-facing: installation, usage, tutorials
  /design/      # Technical: architecture, specs, decisions, ADRs
  /lore/        # Creative: Gonzai, philosophy, manifestos
  /archive/     # Historical: old versions, early drafts
  /testdata/    # Test fixtures and sample repositories
  ```
- [x] Move creative/philosophical content from `/docs/archive/` to `/lore/`:
  - [x] Gonzai personality and stories
  - [x] Poems and manifestos
  - [x] Vision documents
- [x] Move technical specs to `/design/`:
  - [x] Architecture diagrams
  - [x] Feature specifications
  - [x] Link storage format decisions
- [x] Create `/demo/` directory for example repositories

### User Feedback Infrastructure
- [x] Create "Call for Testers" landing page (docs/early-testers.md)
- [ ] Set up email list for early adopters (requires external service)
- [ ] Plan telemetry opt-in for Phase 3
- [ ] Create feedback collection mechanism

## 📦 Phase 1: Project Setup & Core Implementation

### Critical Architecture Decision: F001 Link Storage ✅ DECIDED
- [x] **DECISION: Option B - Tracked files in `.gitmind/links/`**
  - Clean, debuggable, grepable
  - Visible in all Git tools
  - Easy migration to Option C later
- [ ] Document decision in `/design/ADR-001-link-storage.md`
- [ ] Implementation spec:
  ```bash
  # Directory structure
  .gitmind/
    links/
      <sha-of-content>.link    # Deterministic filenames
  
  # File format (one line, human-readable)
  LINK_TYPE: source_path -> target_path  # ts:1736637876
  
  # Example
  CROSS_REF: notes/idea.md -> specs/refactor.md  # ts:1736637876
  
  # Commit message convention
  link(F001): notes/idea.md -> specs/refactor.md
  
  # Optional reference (for future migration)
  refs/semlinks/<sha>
  ```

### Repository Structure (Monorepo)
- [x] Move `/server/` and `/webapp/` to `/demos/archive/poc-2025-06-10/`
- [ ] Create directory structure:
  ```
  neuroglyph/              # This monorepo (root)
  ├── cli/                 # gitmind CLI (Rust)
  ├── demos/               # Example applications
  │   ├── archive/         # Historical POCs
  │   └── example-repos/   # Demo repositories
  ├── design/              # Technical specifications
  ├── docs/                # User documentation
  ├── lore/                # Philosophy & Gonzai
  └── testdata/            # Test fixtures
  ```
- [ ] Update root `.gitignore` for Rust artifacts (`target/`, `Cargo.lock`)
- [ ] Create root `Makefile` for common tasks

### Demo Repository Setup
- [ ] Create `/demo/vault/` with 5-10 example documents
- [ ] Add F001 links between documents
- [ ] Include varied link types (references, dependencies, inspirations)
- [ ] Create `/demo/README.md` explaining the demo structure
- [ ] Add quick-start script that sets up demo environment

### Rust CLI Project Initialization
- [ ] Create `cli/` directory
- [ ] Create `cli/Cargo.toml` with:
  ```toml
  [package]
  name = "gitmind"
  version = "0.1.0"
  edition = "2021"
  
  [dependencies]
  gitoxide = "0.35"  # or git2
  clap = { version = "4", features = ["derive"] }
  serde = { version = "1", features = ["derive"] }
  serde_json = "1"
  thiserror = "1"
  ```
- [ ] Set up CLI structure:
  - [ ] `cli/src/main.rs` - CLI entry point
  - [ ] `cli/src/lib.rs` - Core library
  - [ ] `cli/src/git_store.rs` - F001 implementation
  - [ ] `cli/src/link_types.rs` - Link type definitions
  - [ ] `cli/src/error.rs` - Error types

### Development Environment
- [x] Create `Dockerfile.dev` for Rust development
- [x] Add Rust targets to `docker-compose.yml`
- [x] Set up GitHub Actions for CI:
  - [x] Rust formatting check (`cargo fmt`)
  - [x] Linting (`cargo clippy`)
  - [x] Tests (`cargo test`)
  - [ ] Build artifacts for Linux/macOS/Windows

---

## 🚀 Phase 1a: Minimal Viable Gitmind (Week 1)

### Three Commands Only - Ship Fast, Learn Fast
- [ ] Implement ONLY these commands:
  ```bash
  gitmind init      # Creates .gitmind/links/ directory
  gitmind link A B  # Creates link file, stages, commits
  gitmind list      # Shows all links in current repo
  ```
- [ ] Implementation details:
  - [ ] `init`: Create `.gitmind/links/` (tracked, NOT in .gitignore)
  - [ ] `link`: 
    1. Build canonical content: `CROSS_REF: A -> B  # ts:1736637876`
    2. Hash content to get filename
    3. Write to `.gitmind/links/<sha>.link`
    4. `git add .gitmind/links/<sha>.link`
    5. `git commit -m "link(F001): A -> B"`
  - [ ] `list`: Read all `.gitmind/links/*.link`, parse and display
- [ ] Create demo video showing these 3 commands
- [ ] Ship binaries for Linux/macOS
- [ ] Post "Show HN" with minimal demo
- [ ] **SUCCESS METRIC: 10 people try it and 1 gives feedback**

---

## 🔨 Phase 2: Full CLI Implementation

### F001: Complete Git Storage (Week 2-3)

#### Basic Git Operations
- [ ] Implement `GitStore` struct with gitoxide
- [ ] Create link structure:
  ```rust
  struct Link {
      link_type: LinkType,
      source: PathBuf,
      target: PathBuf,
      timestamp: u64,
  }
  
  impl Link {
      fn to_canonical_string(&self) -> String {
          format!("{}: {} -> {}  # ts:{}", 
              self.link_type, 
              self.source.display(), 
              self.target.display(),
              self.timestamp)
      }
      
      fn sha(&self) -> String {
          // SHA-1 of canonical string
      }
  }
  ```
- [ ] Store link files in `.gitmind/links/<sha>.link`
- [ ] Parse link files back into Link structs
- [ ] (Optional) Create lightweight refs in `refs/semlinks/<sha>` for future migration

#### CLI Commands
- [ ] `gitmind init` - Initialize semlink refs in current repo
- [ ] `gitmind link <source> <target>` - Create relationship
- [ ] `gitmind show <sha>` - Display relationship details
- [ ] `gitmind list` - List all relationships
- [ ] `gitmind graph --json` - Export graph as JSON

#### Testing Infrastructure
- [ ] Create `/testdata/` directory with:
  - [ ] 100+ markdown files with various link patterns
  - [ ] Pre-built Git repositories for testing
  - [ ] Edge case examples (circular links, missing targets, etc.)
- [ ] Unit tests for Git operations
- [ ] Integration tests with temp Git repos
- [ ] Snapshot tests for graph output
- [ ] Test with Docker test environment
- [ ] Benchmark performance with 1K, 10K, 100K links

### F002: Relationship Extraction (Week 2)

#### Markdown Parser
- [ ] Implement Markdown link extraction
- [ ] Support multiple link formats:
  - [ ] `[text](path.md)`
  - [ ] `[[wiki-links]]`
  - [ ] `../cross-repo/links.md`
- [ ] Handle relative vs absolute paths

#### Batch Operations
- [ ] `gitmind scan` - Extract all links from current repo
- [ ] `gitmind scan --watch` - Monitor for changes
- [ ] Progress reporting for large repos
- [ ] Incremental scanning (only changed files)

#### Git Hooks
- [ ] Generate post-commit hook script
- [ ] `gitmind install-hooks` command
- [ ] Hook configuration options

---

## 🌐 Phase 2: Optional Services

### Daemon Implementation (Week 4)
- [ ] Create `gitmind serve` subcommand
- [ ] Implement minimal HTTP server (using `axum` or `warp`)
- [ ] Add WebSocket support for real-time updates
- [ ] API endpoints:
  - [ ] `GET /api/graph` - Full graph data
  - [ ] `POST /api/link` - Create relationship
  - [ ] `DELETE /api/link/:id` - Remove relationship
  - [ ] `WS /api/stream` - Real-time updates

### Web UI Migration (Week 5)
- [ ] Move existing D3.js code to `/examples/web-demo/webapp/`
- [ ] Update to connect to Rust daemon instead of Node.js
- [ ] Add configuration for daemon URL
- [ ] Test with new backend
- [ ] Package as standalone module

---

## 🔌 Phase 3: Developer Experience

### VS Code Extension (Week 6)
- [ ] Initialize TypeScript project in `/plugins/vscode/`
- [ ] Implement Git repository detection
- [ ] Add commands:
  - [ ] "Gitmind: Initialize"
  - [ ] "Gitmind: Create Link"
  - [ ] "Gitmind: Show Graph"
- [ ] Add hover provider for link previews
- [ ] Integrate graph visualization webview

### Documentation (Week 7)
- [ ] Write `/docs/getting-started.md`
- [ ] Create `/docs/cli-reference.md`
- [ ] Document link type specifications
- [ ] Add architecture decision records (ADRs)
- [ ] Create `/design/architecture.md` with:
  - [ ] Local-Machine Stack diagram
  - [ ] Semantic Link Creation Flow (F001)
  - [ ] Distributed Mesh diagram (future)
- [ ] Create example repositories

### Additional Interfaces
- [ ] TUI/ncurses interface for terminal usage
- [ ] Integration with git aliases
- [ ] Shell completions (bash, zsh, fish)

### Distribution
- [ ] Set up release process with `cargo-release`
- [ ] Create Homebrew formula
- [ ] Create AUR package
- [ ] Add to cargo crates.io
- [ ] Create install script

---

## 🚀 Phase 4: Advanced Features

### Chaos Engine (Week 8)
- [ ] Port existing `chaos-worker.sh` to Rust
- [ ] Design chaos algorithm with safety limits
- [ ] Implement `gitmind chaos --rate 5/s` subcommand
- [ ] Add entropy injection modes:
  - [ ] Random link creation
  - [ ] Temporal shuffling
  - [ ] Cross-repo discovery
- [ ] Pattern detection algorithms
- [ ] Gonzai personality integration
- [ ] Add chaos metrics and visualization


### Performance Optimization
- [ ] Implement caching layer:
  - [ ] BoltDB or SQLite as cache ONLY (not source of truth)
  - [ ] In-memory cache per invocation
  - [ ] Cache invalidation on Git changes
- [ ] Add graph indexing for large repos
- [ ] Parallel processing for large repos
- [ ] Memory-mapped file support
- [ ] Stream processing for huge graphs

---

## 🚢 Phase 5: Launch Preparation

### Polish & Documentation
- [ ] Create comprehensive `/docs/getting-started.md`
- [ ] Write `/docs/architecture.md` with diagrams
- [ ] Document all 10 use cases with examples
- [ ] Create troubleshooting guide
- [ ] Add FAQ section

### Marketing Materials
- [ ] Create demo video/screencast showing key features
- [ ] Design README badges (build status, license, chaos level)
- [ ] Write "Accidentally Built a Distributed Mind" blog post
- [ ] Prepare social media announcement templates
- [ ] Create landing page or GitHub Pages site

### Community Building
- [ ] Set up Discord server or GitHub Discussions
- [ ] Create roadmap for post-launch features
- [ ] Establish release schedule
- [ ] Plan first community call/demo

### Launch Checklist
- [ ] Test installation on fresh systems (Linux/macOS/Windows)
- [ ] Verify all documentation links work
- [ ] Create GitHub release with binaries
- [ ] Submit to package managers:
  - [ ] crates.io
  - [ ] Homebrew
  - [ ] AUR
- [ ] Post to:
  - [ ] Hacker News (`Show HN: Gitmind - Git as a Semantic Knowledge Graph`)
  - [ ] r/programming
  - [ ] r/rust
  - [ ] Twitter/X with demo GIF
  - [ ] LinkedIn article
- [ ] Monitor and respond to feedback

---

## 🔮 Future Phases (Post-Launch)

### Phase 6: Cross-Repository & Federation
- [ ] Implement `gitmind remote scan` - detect links in sibling repos
- [ ] Auto-generate `CROSS_REF` edges when semantic overlap detected
- [ ] Repository discovery mechanisms:
  - [ ] Local filesystem scanning
  - [ ] Git remote analysis
  - [ ] Manual repository registration
- [ ] Cross-repo conflict resolution
- [ ] Demo: Two repos performing "mind-meld"
- [ ] Create `/design/MESH_ARCHITECTURE.md` for P2P plans

### Phase 7: Ecosystem Growth
- [ ] Plugin architecture improvements
- [ ] More editor integrations (Vim, Emacs, Sublime)
- [ ] API stability guarantees
- [ ] Enterprise features (LDAP, SSO)

### Phase 8: Advanced Distribution
- [ ] P2P mesh networking (gRPC/QUIC)
- [ ] Blockchain integration for trust
- [ ] IPFS backend option
- [ ] Federation protocol standardization

### Phase 9: AI Integration
- [ ] ML-powered link suggestions
- [ ] Semantic similarity detection
- [ ] Natural language queries
- [ ] Automated knowledge extraction

---

## ✅ Definition of Done

Each task is complete when:
1. Code is written and passes all tests
2. Documentation is updated
3. Integration tests pass
4. Performance benchmarks meet targets
5. Code is reviewed (if working with team)

## 🎯 Success Metrics

### Phase 1 Complete When:
- [ ] Can create and query Git-stored relationships
- [ ] All basic CLI commands work
- [ ] Tests pass on Linux/macOS/Windows
- [ ] Docker test environment validates functionality

### Phase 2 Complete When:
- [ ] Web UI works with new Rust backend
- [ ] Real-time updates function properly
- [ ] API is documented with OpenAPI spec

### Phase 3 Complete When:
- [ ] VS Code extension published
- [ ] Users can install via package managers
- [ ] Documentation is comprehensive

### Phase 4 Complete When:
- [ ] Chaos mode reveals interesting patterns
- [ ] Performance handles 100K+ nodes
- [ ] Community is contributing

---

**Next Action:** Begin Phase 0 by creating the Rust project structure in `/cli/`