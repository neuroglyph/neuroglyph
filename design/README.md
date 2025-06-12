# Gitmind Technical Design

This directory contains technical specifications, architecture decisions, and design documents.

## Contents

- **decisions/** - Architecture Decision Records (ADRs)
- **features/** - Detailed feature specifications (F001-F015)
- **gitmind_architecture.md** - Core architecture diagrams and concepts
- **next-steps.md** - Technical implementation roadmap

## Key Documents

1. **Architecture Overview** - See `gitmind_architecture.md`
2. **Storage Decision** - See `decisions/LICENSE_DECISION.md` and upcoming ADR-001
3. **Feature Specs** - See `features/` for detailed specifications

## Design Principles

1. **Git-native** - Everything is a Git operation
2. **No external database** - Git IS the database
3. **Content-addressable** - Use Git's SHA-1 for deduplication
4. **Distributed-first** - No central server required
5. **Human-readable** - Storage format should be debuggable