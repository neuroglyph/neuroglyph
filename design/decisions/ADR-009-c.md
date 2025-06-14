# ADR-009: Pivot to Pure C Implementation

**Date**: June 13, 2025  
**Status**: Accepted  
**Supersedes**: [ADR-007](ADR-007-language-pivot-to-go.md), [ADR-008](ADR-008-polyglot-architecture.md)

## Context

After initial prototyping in Rust and planning a migration to Go (see ADR-007 and ADR-008), we made a critical decision to implement GitMind in pure C. This decision was driven by a game developer's 20+ years of experience and the realization that for a tool focused on speed and minimalism, C offers unparalleled control and efficiency.

## Decision

We will implement GitMind entirely in C with the following principles:
- Zero dependencies beyond libc
- Memory-safe practices (bounds checking, no sprintf, etc.)
- Embedded implementations where needed (e.g., SHA1)
- Focus on sub-100KB binary size
- Startup time faster than measurable

## Rationale

1. **Ultimate Performance**: C provides direct control over memory and system calls
2. **Minimal Size**: No runtime, no garbage collector, no overhead
3. **Universal Compatibility**: C runs everywhere Git runs
4. **Simplicity**: The entire codebase fits in ~1000 lines
5. **Philosophy Alignment**: "Thought is infrastructure. Speed is cognition."

## Consequences

### Positive
- Achieved 67KB binary (goal was <100KB)
- Startup time <1ms ("Process too fast to measure!")
- Zero external dependencies
- Memory usage ~500KB total
- Simple, auditable codebase

### Negative
- Manual memory management required
- More careful coding needed
- No high-level abstractions

### Neutral
- Need to implement some utilities ourselves (SHA1)
- Cross-platform code requires some #ifdef work

## Implementation Details

The C implementation includes:
- Thread-safe error handling using `__thread`
- Centralized constants and error messages
- Comprehensive input validation
- Memory-safe string operations (snprintf everywhere)
- Clean modular architecture

## Validation

Performance benchmarks confirmed our decision:
```
Binary size:     67KB
Startup time:    <1ms
Link creation:   1.8ms
Memory usage:    ~500KB
Dependencies:    Zero
```

## Notes

This decision represents not just a technical choice but a philosophical stance: software should be invisible, instant, and inevitable. GitMind in C embodies these principles completely.

See the [C implementation journal](../../lore/claude-mind/2025-06-13/21:23-c-implementation-victory-a1b2c3.md) for the full story of this achievement.