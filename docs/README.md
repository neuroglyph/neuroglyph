# Design Documentation

This directory contains all technical design documentation for the Neuroglyph project.

## 🏗️ START HERE
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - System architecture, roadmap, everything technical
- **[ADR-009-c.md](decisions/ADR-009-c.md)** - Pure C implementation decision (June 13, 2025) ✨
- **[TASKLIST.md](../TASKLIST.md)** - Current work items

## Map of Contents

### 📋 Features
Core functionality specifications with user stories and acceptance criteria.

#### Phase 1a - MVP (Complete)
- [F001: Git Object Storage](features/F001-git-object-storage.md) - Core link storage in `.gitmind/links/`
- [F013: CLI Tools](features/F013-cli-tools.md) - Command-line interface
- [F016: Link Hygiene](features/F016-link-hygiene.md) - Unlink and check commands
- [F017: Error Handling Improvements](features/F017-error-handling-improvements.md) - User-friendly error messages
- [F025: CLI Help System](features/F025-cli-help-system.md) - Markdown-driven help ✨ NEW

#### Phase 1b - Web Visualization
- [F018: Web Demo Mode](features/F018-web-demo-mode.md) - Static marketing demo
- [F019: Local Web Server](features/F019-local-web-server.md) - `gitmind serve` command
- [F020: Graph Visualization Engine](features/F020-graph-visualization-engine.md) - D3.js visualization
- [F021: Interactive Graph Editing](features/F021-interactive-graph-editing.md) - Drag-and-drop links
- [F022: Time Travel Interface](features/F022-time-travel-interface.md) - History navigation
- [F023: Search and Filter UI](features/F023-search-and-filter-ui.md) - Finding nodes
- [F024: Export and Sharing](features/F024-export-and-sharing.md) - PNG/SVG/data export

#### Phase 2 - Full Implementation
- [F002: Relationship Extraction](features/F002-relationship-extraction.md) - Auto-detect links
- [F003: Git Hook Integration](features/F003-git-hook-integration.md) - Automatic updates
- [F006: Web Visualization](features/F006-web-visualization.md) - Original web UI spec
- [F007: Realtime Updates](features/F007-realtime-updates.md) - WebSocket support
- [F012: Performance Optimization](features/F012-performance-optimization.md) - Caching layer

### 📐 Architecture
Technical specifications and design documents.

- [architecture.md](architecture.md) - System architecture with diagrams ✨ NEW
- [gitmind_architecture.md](gitmind_architecture.md) - Original architecture notes
- [storage-format-spec.md](storage-format-spec.md) - Link file format specification
- [MILESTONES.md](MILESTONES.md) - Development progression roadmap ✨ NEW

### 🎯 Decisions
Architecture Decision Records (ADRs) documenting key choices.

- [ADR-001: Link Storage](decisions/ADR-001-link-storage.md) - Why we chose tracked files
- [ADR-002: Gitoxide Migration](decisions/ADR-002-gitoxide-migration.md) - Moving from shell commands
- [ADR-003: Web Visualization Strategy](decisions/ADR-003-web-visualization-strategy.md) - Local-first web UI
- [ADR-004: Error Handling Improvements](decisions/ADR-004-error-handling-improvements.md) - User-centric errors
- [ADR-005: Transport Strategy](decisions/ADR-005-transport-strategy.md) - Git-native transport
- [ADR-006: Markdown-Driven Help](decisions/ADR-006-markdown-driven-help.md) - Help system design ✨ NEW
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
    section Phase 1a (MVP)
    F001 Git Storage           :done,    f001, 2025-06-10, 2d
    F013 CLI Tools            :done,    f013, after f001, 2d
    F016 Link Hygiene         :done,    f016, after f013, 1d
    F017 Error Handling       :active,  f017, after f016, 2d
    
    section Phase 1b (Web)
    F018 Demo Mode            :         f018, after f017, 3d
    F019 Web Server           :         f019, after f017, 2d
    F020 Graph Viz            :         f020, after f019, 3d
    F021 Interactive          :         f021, after f020, 2d
    F023 Search/Filter        :         f023, after f020, 1d
    
    section Phase 2
    F022 Time Travel          :         f022, after f023, 2d
    F024 Export               :         f024, after f023, 1d
    F002 Extraction           :         f002, after f024, 2d
    F003 Git Hooks            :         f003, after f002, 1d
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