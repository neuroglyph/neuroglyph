# ADR-003: Web Visualization Strategy

**Status:** Accepted  
**Date:** June 12, 2025  
**Deciders:** James (Project Lead), Claude (Technical Advisor)  

## Context

After achieving near-MVP status with the CLI (90% complete), we identified a critical gap in user experience and adoption potential. While the CLI provides powerful Git-native knowledge graph functionality, several factors pointed to the need for visual representation:

1. **Adoption Barrier**: "What does a semantic knowledge graph even mean?" - potential users struggle to understand the value without seeing it
2. **User Feedback**: "it would be helpful in general to visualize things, as a tool"
3. **Knowledge Nature**: Knowledge graphs are inherently visual - connections and patterns emerge better through visualization
4. **Use Case Gaps**: CLI alone doesn't serve visual thinkers, presenters, or those needing quick exploration

## Decision

We will implement a comprehensive web visualization system with two distinct modes:

1. **Demo Mode** (Pre-installation): A static, zero-friction demo at neuroglyph.dev/demo showing GitMind's value in 30 seconds
2. **Local Companion Mode** (Post-installation): A local web server (`gitmind serve`) providing rich visualization and interaction for installed users

Both proposals are **ACCEPTED**:
- [Local Web Companion Proposal](../proposals/local-web-companion.md) - Accepted for Phase 1b
- [Web Visualization Revival Proposal](../proposals/web-visualization-revival.md) - Accepted with clarification on local-first approach

## Rationale

### Why Both Modes?

The discussion revealed an important clarification: the web visualization serves different purposes at different stages of the user journey:

1. **Demo Mode** solves the discovery problem - users can't value what they can't understand
2. **Local Mode** solves the productivity problem - visual exploration complements CLI precision

### Why Local-First?

Initial proposals included remote/cloud considerations, but user clarification emphasized local operation:
- "OK I meant, the server/website would act LOCALLY"
- This aligns with GitMind's philosophy: your data, your machine, your control
- No accounts, no tracking, no external dependencies

### Why High Priority?

Despite being post-MVP, web visualization is critical for adoption:
- Without visualization, we're asking users to work with an abstract concept
- The CLI proves it works; the web UI proves it's worth it
- Visual interface dramatically lowers barrier to entry

## Consequences

### Positive
- **Improved Adoption**: Demo mode provides instant understanding
- **Better UX**: Visual interface serves different thinking styles
- **Complete Experience**: CLI + Web = comprehensive toolkit
- **Showcase Value**: Easy to demonstrate in presentations/videos
- **Pattern Discovery**: Visual representation reveals emergent structures

### Negative
- **Increased Scope**: 2-3 weeks additional development
- **Maintenance Burden**: Two interfaces to maintain
- **Complexity**: More dependencies (D3.js, web server)
- **Performance Challenges**: Must handle large graphs smoothly

### Neutral
- **Different Skills**: Requires frontend development expertise
- **Testing Complexity**: Browser compatibility, visual regression tests
- **Documentation**: Need to document both CLI and web workflows

## Implementation Plan

### Phase 1: Pre-MVP Demo (3 days)
- Implement F018 (Web Demo Mode)
- Deploy static site
- Include in launch materials

### Phase 2: Core Infrastructure (Week 1)
- Implement F019 (Local Web Server)
- Implement F020 (Graph Visualization Engine)
- Basic read-only visualization

### Phase 3: Interactivity (Week 2)
- Implement F021 (Interactive Graph Editing)
- Implement F023 (Search and Filter UI)
- Full read/write capabilities

### Phase 4: Advanced Features (Based on feedback)
- Implement F022 (Time Travel Interface)
- Implement F024 (Export and Sharing)
- Polish and optimization

## Success Metrics
- Demo: 50% interaction rate, 10% install conversion
- Local: Users report "finally understanding" their knowledge
- Performance: 60 FPS with 500 nodes
- Adoption: Significant increase in user retention

## Discussion Summary

The conversation evolved from initial web revival proposals through clarification of local-first approach to comprehensive feature planning:

1. **Initial Misunderstanding**: First proposal assumed remote demo hosting
2. **Clarification**: "the server/website would act LOCALLY"
3. **Refinement**: Local companion for productivity, static demo for marketing
4. **Expansion**: Seven detailed feature specifications (F018-F024)
5. **Integration**: Updated TASKLIST with prioritized implementation plan

Key insight from discussion: "The CLI provides power, the web UI provides understanding, and together they create a complete experience."

## References
- [F018: Web Demo Mode](../features/F018-web-demo-mode.md)
- [F019: Local Web Server](../features/F019-local-web-server.md)
- [F020: Graph Visualization Engine](../features/F020-graph-visualization-engine.md)
- [F021: Interactive Graph Editing](../features/F021-interactive-graph-editing.md)
- [F022: Time Travel Interface](../features/F022-time-travel-interface.md)
- [F023: Search and Filter UI](../features/F023-search-and-filter-ui.md)
- [F024: Export and Sharing](../features/F024-export-and-sharing.md)

---

*"Sometimes you need to see the forest, not just git log --graph the trees." - Gonzai* 🐵✨