# Gitmind Server

Backend server for the Gitmind knowledge graph system.

## Overview

This Express.js server provides:
- RESTful API for graph data
- WebSocket support for real-time updates
- Git repository scanning
- Manual node/edge creation

## Setup

```bash
npm install
npm start
```

## API Endpoints

- `GET /api/graph-data` - Get current graph data
- `POST /api/node` - Create manual node
- `POST /api/edge` - Create manual edge
- WebSocket events on port 3000

## Configuration

The server expects test repositories in `../git-knowledge-test/` relative to the server directory.

For Docker deployment, this path can be configured via environment variables (TODO).