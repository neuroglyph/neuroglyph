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
**Feature:** [F001 - Git Object Storage](/design/features/F001-git-object-storage.md)
- [x] **DECISION: Option B - Tracked files in `.gitmind/links/`**
  - Clean, debuggable, grepable
  - Visible in all Git tools
  - Easy migration to Option C later
- [ ] Document decision in `/design/ADR-001-link-storage.md`
- [x] Create comprehensive whitepaper at `/docs/gitmind-whitepaper.md`
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
- [x] Create directory structure:
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
- [x] Update root `.gitignore` for Rust artifacts (`target/`, `Cargo.lock`)
- [x] Create root `Makefile` for common tasks

### Demo Repository Setup
- [ ] Create `/demo/vault/` with 5-10 example documents
- [ ] Add F001 links between documents
- [ ] Include varied link types (references, dependencies, inspirations)
- [ ] Create `/demo/README.md` explaining the demo structure
- [ ] Add quick-start script that sets up demo environment

### Rust CLI Project Initialization ✅ DONE
- [x] Create `cli/` directory
- [x] Create `cli/Cargo.toml` with:
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
- [x] Set up CLI structure:
  - [x] `cli/src/main.rs` - CLI entry point
  - [x] `cli/src/lib.rs` - Core library
  - [ ] `cli/src/git_store.rs` - F001 implementation
  - [ ] `cli/src/link_types.rs` - Link type definitions
  - [x] `cli/src/error.rs` - Error types

### Development Environment
- [x] Create `Dockerfile.dev` for Rust development
- [x] Add Rust targets to `docker-compose.yml`
- [x] Create Docker test infrastructure that runs in non-TTY mode
- [x] Fix GitHub Actions deprecated artifact actions (v3 → v4)
- [x] Add working-directory for cross-platform CI tests
- [x] Set up GitHub Actions for CI:
  - [x] Rust formatting check (`cargo fmt`)
  - [x] Linting (`cargo clippy`)
  - [x] Tests (`cargo test`)
  - [ ] Build artifacts for Linux/macOS/Windows

---

## 🚀 Phase 1a: Minimal Viable Gitmind (Week 1)
**Features:** [F001 - Git Object Storage](/design/features/F001-git-object-storage.md), [F013 - CLI Tools](/design/features/F013-cli-tools.md), [F016 - Link Hygiene](/design/features/F016-link-hygiene.md)

### ~~Three~~ Five Commands - Ship Fast, Learn Fast (+ Essential Hygiene)
- [x] Implement core commands:
  ```bash
  gitmind init      # Creates .gitmind/links/ directory ✅
  gitmind link A B  # Creates link file, stages, commits ✅
  gitmind list      # Shows all links in current repo ✅
  ```
- [x] Implement hygiene commands (F016):
  ```bash
  gitmind unlink A B  # Remove specific link ✅
  gitmind check       # Find and fix broken links ✅
  ```
- [x] Implementation details:
  - [x] `init`: Create `.gitmind/links/` (tracked, NOT in .gitignore)
  - [x] `link`: 
    1. Build canonical content: `CROSS_REF: A -> B  # ts:1736637876`
    2. Hash content to get filename (SHA-1)
    3. Write to `.gitmind/links/<sha>.link`
    4. `git add .gitmind/links/<sha>.link`
    5. `git commit -m "link(F001): A -> B"`
    6. Support link types: CROSS_REF, DEPENDS_ON, IMPLEMENTS, INSPIRED_BY
    7. Validate source and target paths exist
  - [x] `list`: Read all `.gitmind/links/*.link`, parse and display
    - [x] Support filtering by source file
    - [x] Support filtering by target file
    - [x] Show link types and timestamps
  - [x] `unlink`: Remove link between files ✅
    - [x] Find SHA-based link file
    - [x] `git rm .gitmind/links/<sha>.link`
    - [x] `git commit -m "unlink(F016): A -/-> B"`
    - [x] Support --all and --to flags
  - [x] `check`: Validate link integrity ✅
    - [x] Scan all links for missing targets
    - [x] Report broken links
    - [x] --fix flag to remove broken links
    - [x] --dry-run flag to preview changes
- [ ] Create demo video showing these 3 commands
- [ ] Ship binaries for Linux/macOS
- [ ] Post "Show HN" with minimal demo
- [ ] **SUCCESS METRIC: 10 people try it and 1 gives feedback**

---

## 🔨 Phase 2: Full CLI Implementation

### F001: Complete Git Storage (Week 2-3)
**Feature:** [F001 - Git Object Storage](/design/features/F001-git-object-storage.md)

#### Basic Git Operations
- [ ] Implement `GitStore` struct with gitoxide
- [ ] Create link structure:
  ```rust
  struct Link {
      link_type: LinkType,
      source: PathBuf,
      target: PathBuf,
      timestamp: u64,
      metadata: Option<HashMap<String, String>>,
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
- [ ] Support metadata in links (author, context, confidence)
- [ ] Implement bidirectional lookups (find all files linking to target)
- [ ] Automatic deduplication via content hashing
- [ ] Performance: Sub-second queries for <10,000 edges
- [ ] Backward compatibility with plain text format
- [ ] (Optional) Create lightweight refs in `refs/semlinks/<sha>` for future migration

#### CLI Commands
- [ ] `gitmind init` - Initialize semlink refs in current repo
- [ ] `gitmind link <source> <target> [--type TYPE]` - Create relationship
- [ ] `gitmind show <sha>` - Display relationship details
- [ ] `gitmind list` - List all relationships
- [ ] `gitmind graph --json` - Export graph as JSON
- [ ] `gitmind import` - Import relationships:
  - [ ] CSV format: `gitmind import links.csv`
  - [ ] JSON format support
  - [ ] Obsidian vault import
- [ ] `gitmind export` - Export relationships:
  - [ ] Cypher format: `--format cypher`
  - [ ] GraphML format: `--format graphml`
  - [ ] DOT format for Graphviz
- [ ] `gitmind query` - Query relationships:
  - [ ] Natural language queries: `"find all related to AI"`
  - [ ] GraphQL-style: `--gql 'MATCH (n)-[:REFERENCES]->(m) RETURN n,m'`
  - [ ] Filter by type, source, target
- [ ] `gitmind at <commit>` - Time travel queries
- [ ] `gitmind stats` - Show graph statistics
- [ ] `gitmind optimize` - Optimize graph storage
- [ ] `gitmind gc --aggressive` - Garbage collection
- [ ] `gitmind config` - Configuration management:
  - [ ] `gitmind config cache.size 1GB`
  - [ ] `gitmind config gonzai.personality playful`
- [ ] `gitmind viz`:
  - [ ] `--ascii` - Terminal visualization
  - [ ] `--browser` - Open web interface

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
**Feature:** [F002 - Relationship Extraction](/design/features/F002-relationship-extraction.md)

#### Markdown Parser
- [ ] Implement Markdown link extraction
- [ ] Support multiple link formats:
  - [ ] Standard Markdown: `[text](path.md)`
  - [ ] Reference-style: `[text][ref]` with `[ref]: path.md`
  - [ ] Wiki-style: `[[wiki-links]]`
  - [ ] Image embeds: `![alt](image.png)`
  - [ ] Anchor links: `[text](file.md#section)`
  - [ ] Cross-repo: `../cross-repo/links.md`
- [ ] Handle relative vs absolute paths
- [ ] Resolve symlinks and junction points
- [ ] Validate target existence
- [ ] Capture link text for context
- [ ] Extract surrounding paragraph for semantic analysis
- [ ] Record line numbers for precise tracking
- [ ] Identify link type (reference, embed, citation)

#### Batch Operations
- [ ] `gitmind scan` - Extract all links from current repo
- [ ] `gitmind scan --watch` - Monitor for changes
- [ ] Progress reporting for large repos
- [ ] Incremental scanning (only changed files)
- [ ] Performance requirements:
  - [ ] Process 100 markdown files in <1 second
  - [ ] Parallel processing for large repositories
  - [ ] Memory usage <100MB for 10,000 files
- [ ] Support configurable repository mappings

#### Git Hooks
**Feature:** [F003 - Git Hook Integration](/design/features/F003-git-hook-integration.md)
- [ ] Generate post-commit hook script
- [ ] `gitmind install-hooks` command
- [ ] Hook configuration options

---

## 🌐 Phase 2: Optional Services

### Daemon Implementation (Week 4)
**Features:** [F006 - Web Visualization](/design/features/F006-web-visualization.md), [F007 - Realtime Updates](/design/features/F007-realtime-updates.md)
- [ ] Create `gitmind serve` subcommand
- [ ] Implement minimal HTTP server (using `axum` or `warp`)
- [ ] Add WebSocket support for real-time updates
- [ ] API endpoints:
  - [ ] `GET /api/graph` - Full graph data
  - [ ] `POST /api/link` - Create relationship
  - [ ] `DELETE /api/link/:id` - Remove relationship
  - [ ] `WS /api/stream` - Real-time updates

### Web UI Migration (Week 5)
**Feature:** [F006 - Web Visualization](/design/features/F006-web-visualization.md)
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
- [ ] Integration with git aliases:
  ```gitconfig
  [alias]
      mind = !gitmind
      mindlog = !gitmind query "nodes modified in last commit"
      mindshow = !gitmind viz --focus
  ```
- [ ] Shell completions (bash, zsh, fish):
  - [ ] Autocomplete for files
  - [ ] Autocomplete for subcommands
  - [ ] Recent query history
- [ ] REPL mode with interactive commands
- [ ] Batch mode for processing multiple operations
- [ ] Watch mode: auto-update on file changes
- [ ] Pipeline integration (GitHub Actions, pre-commit hooks)
- [ ] JSON output for all commands (--json flag)

### Distribution
- [ ] Set up release process with `cargo-release`
- [ ] Create Homebrew formula
- [ ] Create AUR package
- [ ] Add to cargo crates.io
- [ ] Create install script

---

## 🚀 Phase 4: Advanced Features

### Chaos Engine (Week 8)
**Feature:** Chaos-driven discovery (Gonzai engine)
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
**Feature:** [F012 - Performance Optimization](/design/features/F012-performance-optimization.md)
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