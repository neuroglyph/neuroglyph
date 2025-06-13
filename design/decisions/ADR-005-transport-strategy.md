# ADR-005: Transport and Distribution Strategy

**Status:** Accepted  
**Date:** June 12, 2025  
**Deciders:** James (Project Lead), Claude (Technical Advisor)  

## Context

As we design GitMind's architecture, we need to decide how semantic links will be shared between repositories and users. The temptation is to create a custom protocol, but Git already provides robust transport mechanisms.

Key insights from early design:
- Git already syncs over SSH/HTTPS with packfiles
- Semantic objects can ride inside existing Git transport
- Real-time features are optional enhancements, not core requirements

## Decision

We will use a phased approach to transport and distribution:

### Phase 1: Git-Native Transport
- Semantic links stored in `.gitmind/links/` are regular Git objects
- Distribution happens via standard `git push/pull`
- No custom protocols or external dependencies
- Users share knowledge graphs by sharing Git repositories

### Phase 2: Optional Local Daemon
- `gitmind serve` starts on port `:7432` (memorable: 7432 = "MIND" on phone keypad)
- HTTP/WebSocket for local web UI only
- No network exposure by default (localhost only)
- Daemon is viewer/editor, not required for core functionality

### Phase 3: Future Real-Time Mesh
- Experimental gRPC or QUIC for live cross-repo suggestions
- Peer-to-peer gossip layer for "chaos" discovery
- Always optional - repos remain functional without it
- No blockchain, no consensus required

## Rationale

### Why Not Invent a Protocol?

The next-steps.md document wisely noted: "Don't invent a UDP protocol yet." This is because:

1. **Git Already Solved This**: Git has 15+ years of battle-tested transport
2. **Security**: SSH/HTTPS have proven auth and encryption
3. **Simplicity**: Fewer moving parts means fewer bugs
4. **Compatibility**: Works with existing Git infrastructure

### Why Port 7432?

- Memorable: 7432 spells "MIND" on phone keypads
- Unlikely to conflict with common services
- Easy to remember for debugging
- Can be overridden with `--port` flag

### Why Optional Daemon?

Keeping the daemon optional ensures:
- CLI remains fully functional standalone
- No background processes unless user wants UI
- Clear separation between core (CLI) and enhancement (web UI)
- Reduced attack surface when not using visualization

## Consequences

### Positive
- **Simple deployment**: Just share Git repos as normal
- **Security**: Inherits Git's security model
- **Reliability**: No custom networking code to debug
- **Progressive enhancement**: Advanced features don't break basics
- **Future flexibility**: Can add mesh networking later

### Negative
- **No real-time sync**: Changes require explicit push/pull
- **No cross-repo discovery**: Can't automatically find related repos (until Phase 3)
- **Manual coordination**: Teams must share repo locations

### Neutral
- **Performance**: Git transport is efficient but not real-time
- **Learning curve**: Users must understand Git remotes
- **Tool integration**: IDEs can use local daemon when available

## Implementation Notes

### Phase 1 (Current)
```bash
# Share knowledge graph
git push origin main

# Get updated graph
git pull origin main
```

### Phase 2 (Local UI)
```bash
# Start local server
gitmind serve --port 7432

# UI connects to http://localhost:7432
```

### Phase 3 (Future Mesh)
```
# Hypothetical future command
gitmind mesh --discover --peer alice@example.com
```

## References
- [Architecture Overview](../architecture.md) - System diagrams
- [F019: Local Web Server](../features/F019-local-web-server.md)
- [MILESTONES](../MILESTONES.md) - Phased development plan
- Git transport documentation