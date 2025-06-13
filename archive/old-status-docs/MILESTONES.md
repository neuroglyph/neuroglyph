# GitMind Development Milestones

This document outlines the key milestones in GitMind's development, providing a clear progression from MVP to full distributed system.

## Milestone Overview

| Milestone | Deliverable | Key Components | Success Criteria |
|-----------|-------------|----------------|------------------|
| **MVP-CLI** | `gitmind` binary with core commands | Rust CLI, gitoxide integration | 5 commands working, <10MB binary |
| **Local UI** | `gitmind serve` + D3 visualization | HTTP server, WebSocket, D3.js | Graph renders <1000 nodes smoothly |
| **Chaos Mode** | `gitmind chaos --rate 5/s` | Chaos engine, pattern detection | Discovers non-obvious connections |
| **Plug-ins** | VS Code extension | Language server protocol | IntelliSense for semantic links |
| **Mesh Alpha** | P2P repository discovery | gRPC/QUIC transport | Cross-repo link suggestions |

## Detailed Milestone Descriptions

### 🎯 MVP-CLI (Phase 1a) - COMPLETE ✅
**Goal:** Prove the concept with minimal features

**Deliverables:**
- Single static binary (Rust + gitoxide)
- Five essential commands:
  - `gitmind init` - Initialize repository
  - `gitmind link` - Create semantic links
  - `gitmind list` - Query links
  - `gitmind unlink` - Remove links
  - `gitmind check` - Validate links
- JSON output for tooling integration
- Comprehensive error messages

**Stack:** Rust, gitoxide, clap, serde

**Status:** Phase 1a complete, ready for user testing

### 🖥️ Local UI (Phase 1b)
**Goal:** Make knowledge graphs visible and interactive

**Deliverables:**
- `gitmind serve` command starts local server on :7432
- Web-based D3.js visualization
- Real-time updates via WebSocket
- Drag-and-drop link creation
- Search and filter capabilities
- Export to PNG/SVG

**Stack:** Same binary + axum HTTP server + embedded web assets

**Timeline:** 2-3 weeks after MVP feedback

### 🎲 Chaos Mode (Phase 2)
**Goal:** Discover emergent patterns through controlled randomness

**Deliverables:**
- `gitmind chaos --rate 5/s` spawns discovery workers
- Configurable chaos algorithms
- Pattern detection and clustering
- Gonzai personality integration
- Safety limits and rollback

**Stack:** Tokio async workers, still file-based

**Use Case:** "Show me connections I didn't know existed"

### 🔌 Plug-ins (Phase 2-3)
**Goal:** Integrate with existing developer workflows

**Deliverables:**
- VS Code extension
  - Hover to preview linked files
  - IntelliSense for link creation
  - Inline graph visualization
- Vim/Neovim plugin
- JetBrains IDE support

**Stack:** Node.js wrapper calling CLI binary

**Success:** Developers never leave their editor

### 🌐 Mesh Alpha (Phase 3)
**Goal:** Enable cross-repository knowledge sharing

**Deliverables:**
- Experimental P2P daemon
- Repository discovery protocol
- Cross-repo link suggestions
- Distributed chaos algorithms
- Privacy controls

**Stack:** Optional gRPC/QUIC overlay, not required for basic use

**Note:** Git push/pull remains primary transport

## Technology Evolution

### Phase 1: Local First
- Everything works offline
- Single-user focused
- File-based storage
- Git as transport

### Phase 2: Enhanced Experience  
- Optional web UI
- Real-time updates
- Plugin ecosystem
- Performance optimizations

### Phase 3: Distributed Knowledge
- Peer-to-peer discovery
- Cross-repository insights
- Collaborative chaos
- Mesh networking

## Core Principles Maintained

Throughout all milestones:
1. **No external database** - Git remains the source of truth
2. **Progressive enhancement** - Each phase adds optional features
3. **Backward compatibility** - Old repos work with new versions
4. **Local-first** - Network features are always optional

## Definition of "Done" per Milestone

Each milestone is complete when:
1. All deliverables are implemented and tested
2. Documentation is updated
3. Binary builds for Linux/macOS/Windows
4. Performance meets specified criteria
5. User feedback incorporated
6. No critical bugs remain

## Risk Mitigation

- **Scope creep**: Each milestone has clear boundaries
- **Complexity**: Optional features don't affect core
- **Adoption**: MVP useful standalone, rest is bonus
- **Performance**: Benchmarks at each milestone
- **Security**: No network features until Phase 3

## References
- [Implementation TASKLIST](../TASKLIST.md)
- [Architecture Overview](architecture.md)
- [Transport Strategy](decisions/ADR-005-transport-strategy.md)