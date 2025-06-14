# GitMind Demos

This directory contains demonstration applications and examples for GitMind.

## Directory Structure

```
demos/
├── README.md          # This file
├── vault/             # Demo content (realistic project)
├── mvp/               # Docker-based demo environment
├── archive/           # Historical POCs and experiments
│   └── poc-2025-06-10/  # Initial proof-of-concept (Node.js/D3.js)
├── quickstart/        # (future) Quick demo for new users
├── use-cases/         # (future) Demos of the 10 canonical use cases
└── integrations/      # (future) Integration examples (VS Code, Obsidian, etc.)
```

## 🚀 Quick Demo

Want to see GitMind in action? Run the demo:

```bash
cd mvp
./test-demo.sh
```

This creates a complete demo repository with semantic links in an isolated Docker environment.

## Archive

### poc-2025-06-10
The original proof-of-concept created on June 10, 2025 that validated the core idea:
- Node.js server with Git integration
- D3.js visualization
- Chaos testing scripts
- Docker test environment

This POC proved that Git could function as a distributed knowledge graph and led to the current C implementation.

## Future Demos

Once the C implementation is fully distributed, this directory will contain:
- **quickstart/**: A 5-minute demo showing basic functionality
- **use-cases/**: Full demonstrations of each of the 10 use cases from the whitepaper
- **integrations/**: Examples of Gitmind integrated with popular tools