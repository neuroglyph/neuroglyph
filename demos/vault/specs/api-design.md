# API Design Specification

## CLI Interface

### Core Commands

```bash
gitmind init                          # Initialize in current repo
gitmind link <src> <dst> [--type T]   # Create link
gitmind list [--source S] [--target T] # Query links
gitmind unlink <src> <dst>            # Remove link
gitmind check [--fix]                 # Validate links
gitmind status                        # Show statistics
```

### Advanced Commands

```bash
gitmind serve                         # Start web server
gitmind export --format json          # Export graph
gitmind import links.csv              # Bulk import
gitmind chaos --rate 5                # Speculative mode
```

## Web API (Future)

### Endpoints

```
GET  /api/graph              # Full graph data
GET  /api/links?source=X     # Query links
POST /api/links              # Create link
DELETE /api/links/:id        # Remove link
WS   /api/stream             # Real-time updates
```

### Data Format

```json
{
  "links": [{
    "id": "sha1hash",
    "type": "IMPLEMENTS",
    "source": "design.md",
    "target": "code.rs",
    "timestamp": 1234567890
  }]
}
```