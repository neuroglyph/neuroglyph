# Proof of Concept - June 10, 2025

**Status:** ARCHIVED - This POC was created on June 10, 2025 and successfully validated the concept, leading to the production Rust implementation.

This directory contains all the experimental and demo code from the initial Gitmind exploration phase.

## Contents

### Demo Web Application
- **server/** - Node.js Express server that demonstrated Git-based graph visualization
- **webapp/** - D3.js frontend for visualizing the knowledge graph
- **git_graph_server.js** - Original server implementation
- **git_graph_client.html** - Standalone client demo
- **git_graph_package.json** - Package config for the demo

### Test Environment Scripts
- **scripts/** - Contains test environment creation and chaos testing scripts
  - `chaos-worker.sh` - Generates random test data
  - `extract-links.sh` - Extracts markdown links
  - `setup-test-env.sh` - Docker-compatible setup
- **setup.sh** - Original test environment setup
- **git_knowledge_setup.sh** - Knowledge graph test setup
- **git_graph_setup.sh** - Graph visualization setup
- **fixed_chaos_script.sh** - Updated chaos testing script

### Assets
- **revolution_artwork.svg** - Gonzai mascot artwork

## Status

These files represent the successful proof-of-concept that demonstrated:
- Git can be used as a graph database
- Real-time visualization of Git-stored relationships works
- The chaos testing approach generates useful test data

The production implementation (Rust CLI) will be built in the main project directory, using lessons learned from this POC.