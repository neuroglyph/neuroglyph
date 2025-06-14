# Design Documentation

This directory contains all technical design documentation for Neuroglyph.

## Structure

- **features/** - Feature specifications organized by status
  - **active/** - Features currently being implemented
  - **planned/** - Features planned for future implementation
  - **completed/** - Successfully implemented features
- **decisions/** - Architecture Decision Records (ADRs)
- **proposals/** - Design proposals for new features or changes
- **research/** - Research documents and analysis
- **ideas/** - Experimental ideas and concepts
- **issues/** - Technical issue documentation

## Key Documents

- [Architecture Overview](ARCHITECTURE.md)
- [C Implementation Architecture](ARCHITECTURE-C.md)
- [GitMind Whitepaper](gitmind-whitepaper.md)
- [Feature Test Mapping](features/feature-test-mapping.md)

## Feature Organization

Features are organized by their implementation status:

1. **Completed Features** - Fully implemented with tests
2. **Active Features** - Currently being worked on
3. **Planned Features** - Approved but not yet started

Each feature document follows the format F###-feature-name.md and includes:
- User story
- Acceptance criteria
- Technical approach
- Test requirements