# F019: Local Web Server

**Status:** PLANNED  
**Priority:** High (Phase 1b)  
**Effort:** 1 week  
**Dependencies:** Core CLI functionality  

## Summary
Implement `gitmind serve` command that starts a local web server, providing HTTP/WebSocket APIs for the web UI to visualize and interact with the current repository's knowledge graph.

## User Stories

### 1. The Visual Explorer
As a GitMind user, I want to run `gitmind serve` and see my knowledge graph in a web browser, so I can understand the big picture of my project's connections.

### 2. The Presentation Giver
As a developer in a code review, I want to show my team the architectural connections visually, so they understand the impact of changes.

### 3. The Link Creator
As someone who prefers GUIs, I want to create links by dragging and dropping in a visual interface, so I don't have to remember CLI syntax.

### 4. The Pattern Seeker
As a project maintainer, I want to see clusters and patterns in my knowledge graph, so I can identify architectural issues or opportunities.

## Acceptance Criteria

### 1. Server Lifecycle
- [ ] `gitmind serve` starts server on default port (7432)
- [ ] `gitmind serve --port 8080` allows custom port
- [ ] `gitmind serve --open` opens browser automatically
- [ ] Graceful shutdown on Ctrl+C
- [ ] Clear startup message with URL
- [ ] Detects port conflicts and suggests alternatives

### 2. API Endpoints
- [ ] `GET /api/graph` - Returns complete graph data
- [ ] `GET /api/nodes` - List all nodes with metadata
- [ ] `GET /api/links` - List all links with types
- [ ] `GET /api/node/:path` - Get specific node details
- [ ] `POST /api/link` - Create new link
- [ ] `DELETE /api/link/:id` - Remove link
- [ ] `GET /api/stats` - Graph statistics
- [ ] `GET /health` - Server health check

### 3. WebSocket Support
- [ ] `/ws` endpoint for real-time updates
- [ ] Broadcasts file change events
- [ ] Broadcasts new/removed links
- [ ] Handles client reconnection
- [ ] Graceful degradation without WebSocket

### 4. Static Asset Serving
- [ ] Serves bundled web UI from `/`
- [ ] Proper MIME types for all assets
- [ ] Cache headers for performance
- [ ] Fallback to index.html for SPA routing
- [ ] Embedded assets (no external dependencies)

### 5. Security & Isolation
- [ ] Only binds to localhost by default
- [ ] No authentication (local only)
- [ ] Read-only mode with `--readonly` flag
- [ ] CORS headers for local development
- [ ] Validates all path inputs

### 6. File System Watching
- [ ] Monitors `.gitmind/links/` for changes
- [ ] Watches for file moves/renames
- [ ] Debounces rapid changes
- [ ] Handles watch errors gracefully
- [ ] Configurable watch patterns

## Technical Design

### Architecture
```rust
struct WebServer {
    app: App<GitOperations, FileSystem, Clock>,
    port: u16,
    watcher: FileWatcher,
    clients: Arc<Mutex<Vec<WebSocketClient>>>,
}

impl WebServer {
    async fn start(&self) -> Result<()> {
        let router = Router::new()
            .route("/api/graph", get(Self::get_graph))
            .route("/api/link", post(Self::create_link))
            .route("/ws", get(Self::websocket_handler))
            .fallback(Self::static_handler);
            
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await?;
        Ok(())
    }
}
```

### API Response Formats

#### Graph Response
```json
{
  "nodes": [
    {
      "id": "src/main.rs",
      "path": "src/main.rs",
      "exists": true,
      "size": 2048,
      "modified": "2025-06-12T10:00:00Z",
      "extension": "rs",
      "directory": "src"
    }
  ],
  "links": [
    {
      "id": "abc123",
      "source": "src/main.rs",
      "target": "README.md",
      "type": "IMPLEMENTS",
      "timestamp": 1736637876
    }
  ],
  "stats": {
    "nodeCount": 42,
    "linkCount": 67,
    "linkTypes": ["IMPLEMENTS", "REFERENCES", "INSPIRED_BY"]
  }
}
```

#### WebSocket Messages
```json
{
  "type": "link_added",
  "data": {
    "link": { /* link object */ },
    "timestamp": "2025-06-12T10:00:00Z"
  }
}
```

### Dependencies
```toml
# Cargo.toml additions
axum = "0.7"           # Web framework
tokio = { version = "1", features = ["full"] }
tower = "0.4"          # Middleware
tower-http = { version = "0.5", features = ["fs", "cors"] }
notify = "6.0"         # File watching
tokio-tungstenite = "0.21"  # WebSocket
include_dir = "0.7"    # Embed static assets
```

### Embedded Assets Strategy
The web UI will be built and embedded into the binary:
```rust
use include_dir::{include_dir, Dir};

static WEB_UI: Dir = include_dir!("$CARGO_MANIFEST_DIR/web-ui/dist");

async fn static_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    match WEB_UI.get_file(path) {
        Some(file) => {
            let mime = mime_guess::from_path(path);
            Response::builder()
                .header("content-type", mime.as_ref())
                .body(file.contents())
                .unwrap()
        }
        None => {
            // SPA fallback
            Response::builder()
                .header("content-type", "text/html")
                .body(WEB_UI.get_file("index.html").unwrap().contents())
                .unwrap()
        }
    }
}
```

## Error Handling

### Server Startup Errors
- Port already in use → Suggest alternative ports
- Cannot bind to address → Check permissions
- Missing .gitmind directory → Suggest `gitmind init`

### Runtime Errors  
- File not found → Return 404 with helpful message
- Invalid link creation → Return 400 with validation errors
- Git operation failed → Return 500 with safe error message
- WebSocket disconnection → Client auto-reconnect

## Performance Requirements
- Startup time < 1 second
- API response time < 100ms for 10K nodes
- WebSocket broadcast < 50ms
- Memory usage < 50MB base
- Graceful handling of 100+ concurrent connections

## Testing Strategy

### Unit Tests
- API endpoint logic
- WebSocket message handling
- File watching debounce
- Static file serving

### Integration Tests
- Full server lifecycle
- API request/response
- WebSocket connections
- File change notifications
- Concurrent request handling

### Manual Testing
- Various repository sizes
- Rapid file changes
- Browser compatibility
- Network interruptions
- Performance profiling

## Implementation Plan

### Phase 1: Basic Server (2 days)
1. Create serve subcommand
2. Implement basic HTTP server
3. Add /api/graph endpoint
4. Serve static files
5. Add health check

### Phase 2: Full API (2 days)
1. Implement all CRUD endpoints
2. Add WebSocket support
3. Integrate file watching
4. Add error handling
5. Implement --readonly mode

### Phase 3: Production Ready (3 days)
1. Embed web UI assets
2. Add graceful shutdown
3. Implement port conflict handling
4. Add performance optimizations
5. Comprehensive error messages

## Security Considerations
- Local-only by default (no 0.0.0.0 binding)
- Path traversal prevention in API
- No execution of user content
- Safe Git operations only
- No sensitive data in responses

## Future Enhancements
- HTTPS support with self-signed certs
- Basic auth for network access
- GraphQL API option
- Server-sent events as WebSocket alternative
- Metrics and monitoring endpoints

## Related Documents
- [ADR-003: Web Visualization Strategy](../decisions/ADR-003-web-visualization-strategy.md) - Architecture decision
- [Local Web Companion Proposal](../proposals/local-web-companion.md) - Original proposal
- [F018: Web Demo Mode](F018-web-demo-mode.md) - Marketing demo
- [F020: Graph Visualization Engine](F020-graph-visualization-engine.md) - The frontend
- [F021: Interactive Graph Editing](F021-interactive-graph-editing.md) - UI interactions
- [F001: Git Object Storage](F001-git-object-storage.md) - Data source