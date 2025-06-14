# Design Documentation

This directory contains all technical design documentation for the Neuroglyph project.

## 🏗️ START HERE
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - System architecture, roadmap, everything technical
- **[ADR-009-c.md](decisions/ADR-009-c.md)** - Pure C implementation decision (June 13, 2025) ✨
- **[TASKLIST.md](../TASKLIST.md)** - Current work items

## Map of Contents

### 📋 Features
Core functionality specifications with user stories and acceptance criteria.

#### ✅ Completed Features
*Fully implemented and tested features moved to [features/completed/](features/completed/)*
- [F001: Git Object Storage](features/completed/F001-git-object-storage.md) - Core link storage ✅
- [F013: CLI Tools](features/completed/F013-cli-tools.md) - Core 7 commands (init, link, list, unlink, check, status, version) ✅
- [F016: Link Hygiene](features/completed/F016-link-hygiene.md) - Unlink and check commands ✅
- [F026: Basic Graph Traversal](features/completed/F026-basic-graph-traversal.md) - Traverse command with BFS ✅ NEW!

#### 🎯 Current Sprint - MVP for HN
*Features needed for Hacker News launch*
- [F019: Local Web Server](features/F019-local-web-server.md) - `gitmind serve` command
- [F020: Graph Visualization Engine](features/F020-graph-visualization-engine.md) - D3.js visualization

#### 🔜 Next Up
- [F027: Path Finding](features/F027-path-finding.md) - How are A and B connected?
- [F028: Pattern Matching](features/F028-pattern-matching.md) - Find structural patterns
- [F029: Graph Analytics](features/F029-graph-analytics.md) - PageRank, centrality
- [F030: Query Languages](features/F030-pluggable-query-languages.md) - SQL/Cypher/Natural

#### 📋 Future Features
- [F002: Relationship Extraction](features/F002-relationship-extraction.md) - Auto-detect links
- [F003: Git Hook Integration](features/F003-git-hook-integration.md) - Automatic updates
- [F006: Web Visualization](features/F006-web-visualization.md) - Original web UI spec
- [F007: Realtime Updates](features/F007-realtime-updates.md) - WebSocket support
- [F012: Performance Optimization](features/F012-performance-optimization.md) - Caching layer

### 📐 Architecture
Technical specifications and design documents.

- [ARCHITECTURE.md](ARCHITECTURE.md) - System architecture and roadmap ✅
- [ARCHITECTURE-C.md](ARCHITECTURE-C.md) - C implementation architecture ✅
- [feature-test-mapping.md](features/feature-test-mapping.md) - Test coverage mapping ✅ NEW

### 🎯 Decisions
Architecture Decision Records (ADRs) documenting key choices.

- [ADR-001: Link Storage](decisions/ADR-001-link-storage.md) - Why we chose tracked files
- [ADR-002: Gitoxide Migration](decisions/ADR-002-gitoxide-migration.md) - Moving from shell commands
- [ADR-003: Web Visualization Strategy](decisions/ADR-003-web-visualization-strategy.md) - Local-first web UI
- [ADR-004: Error Handling Improvements](decisions/ADR-004-error-handling-improvements.md) - User-centric errors
- [ADR-005: Transport Strategy](decisions/ADR-005-transport-strategy.md) - Git-native transport
- [ADR-006: Markdown-Driven Help](decisions/ADR-006-markdown-driven-help.md) - Help system design
- [ADR-007: Language Pivot to Go](decisions/ADR-007-language-pivot-to-go.md) - Go decision (superseded)
- [ADR-008: Polyglot Architecture](decisions/ADR-008-polyglot-architecture.md) - Multi-language approach
- [ADR-009: Pure C Implementation](decisions/ADR-009-c.md) - Final C decision ✨
- [LICENSE_DECISION.md](decisions/LICENSE_DECISION.md) - Apache 2.0 rationale

### 💡 Proposals
Detailed proposals for major features.

- [error-handling-improvement.md](proposals/error-handling-improvement.md) - ✅ IMPLEMENTED
- [local-web-companion.md](proposals/local-web-companion.md) - ✅ ACCEPTED
- [web-visualization-revival.md](proposals/web-visualization-revival.md) - ✅ ACCEPTED

### 🔍 Audits
Systematic reviews of codebase and design.

- [test-double-audit.md](audits/test-double-audit.md) - Test double friendliness review
- [git-edge-cases-audit.md](audits/git-edge-cases-audit.md) - 25+ edge cases to handle
- [gitoxide-test-double-analysis.md](audits/gitoxide-test-double-analysis.md) - Gitoxide testing approach

## Feature Dependencies

```mermaid
graph TD
    %% Core Features
    F001[F001: Git Storage] --> F013[F013: CLI Tools]
    F001 --> F016[F016: Link Hygiene]
    F013 --> F016
    
    %% Error Handling
    F013 --> F017[F017: Error Handling]
    F016 --> F017
    
    %% Web Infrastructure
    F001 --> F019[F019: Web Server]
    F019 --> F020[F020: Graph Viz]
    F020 --> F021[F021: Interactive Edit]
    F020 --> F022[F022: Time Travel]
    F020 --> F023[F023: Search/Filter]
    F020 --> F024[F024: Export]
    
    %% Demo Mode
    F020 --> F018[F018: Demo Mode]
    
    %% Phase 2
    F001 --> F002[F002: Extraction]
    F002 --> F003[F003: Git Hooks]
    F019 --> F006[F006: Web UI]
    F019 --> F007[F007: Realtime]
    F001 --> F012[F012: Performance]
    
    %% Styling
    classDef complete fill:#90EE90
    classDef accepted fill:#87CEEB
    classDef planned fill:#FFE4B5
    
    class F001,F013,F016 complete
    class F017,F018,F019,F020,F021,F022,F023,F024 accepted
    class F002,F003,F006,F007,F012 planned
```

## Implementation Timeline

```mermaid
gantt
    title GitMind Implementation Timeline
    dateFormat  YYYY-MM-DD
    section Phase 1a (Complete)
    F001 Git Storage           :done,    f001, 2025-06-10, 2d
    F013 CLI Tools            :done,    f013, after f001, 2d
    F016 Link Hygiene         :done,    f016, after f013, 1d
    
    section Phase 1b (Current Sprint)
    F026 Graph Traversal      :active,  f026, 2025-06-16, 5d
    F019 Web Server           :active,  f019, 2025-06-16, 5d
    F020 Graph Viz            :         f020, after f019, 3d
    
    section Phase 2 (Future)
    F027 Path Finding         :         f027, 2025-07-01, 5d
    F028 Pattern Matching     :         f028, after f027, 5d
    F029 Graph Analytics      :         f029, after f028, 5d
    F030 Query Languages      :         f030, after f029, 7d
```

## Design Principles

1. **Git-Native**: Git is the database, not an afterthought
2. **Local-First**: Your data, your machine, your control
3. **Progressive Enhancement**: CLI first, web as enhancement
4. **User-Centric**: Errors are teaching moments, not failures
5. **Test-Driven**: Behavior matters, not implementation

## Quick Links

- [Project Root README](../README.md)
- [Implementation TASKLIST](../TASKLIST.md)
- [Development Instructions](../CLAUDE.md)
- [Philosophy & Lore](../lore/)