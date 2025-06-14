# Web UI Specification

## Overview

Interactive graph visualization for exploring semantic links.

## Core Features

### Graph Visualization
- Force-directed layout
- Node sizing by importance
- Edge coloring by type
- Smooth pan/zoom
- Click to focus

### Interaction
- Drag to create links
- Right-click for options
- Double-click to open file
- Hover for preview
- Search with highlighting

### Performance Targets
- 60 FPS with 500 nodes
- Usable with 10,000 nodes
- Initial load <1 second
- Incremental updates

## Technology Stack

- **Frontend**: Vanilla JS + D3.js
- **Backend**: gitmind serve
- **Protocol**: WebSocket for updates
- **Size**: <100KB total

## Design Principles

1. **Fast** - Instant response
2. **Simple** - No framework bloat
3. **Intuitive** - Discoverable UI
4. **Powerful** - Advanced features hidden

See [visualization ideas](../ideas/visualization.md) for mockups.