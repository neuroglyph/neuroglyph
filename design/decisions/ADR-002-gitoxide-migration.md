# ADR-002: Migrate from Git CLI to Gitoxide Library

## Status
Accepted (June 12, 2025)

## Context
The initial implementation of gitmind used `std::process::Command` to shell out to the git CLI for all Git operations. This approach had several limitations:

1. **Performance overhead** - Spawning a new process for each Git operation
2. **Error handling** - Parsing stderr strings instead of structured errors
3. **Platform dependency** - Requires git to be installed on the system
4. **Testing limitations** - Cannot use in-memory repositories for fast tests
5. **Limited control** - Can only do what the CLI exposes

Given that our philosophy is "Git IS the database", we should treat Git as a first-class library, not an external tool.

## Decision
We will migrate from shelling out to git commands to using the [gitoxide](https://github.com/GitoxideLabs/gitoxide) library (gix) for all Git operations.

## Consequences

### Positive
- **Better performance** - Native Rust implementation, no process spawning
- **Type safety** - Structured error types and proper Rust APIs
- **Cross-platform** - Works anywhere Rust compiles, no git dependency
- **Better testing** - Can use in-memory repositories for unit tests
- **Direct Git access** - Can work with Git internals when needed
- **Consistent behavior** - Not dependent on git version or configuration

### Negative
- **Additional dependency** - Adds ~2MB to binary size
- **API differences** - Gitoxide API is lower-level than git CLI
- **Learning curve** - Team needs to understand gitoxide APIs
- **Potential bugs** - Gitoxide is newer than libgit2

### Neutral
- **No user-visible changes** - Thanks to our `GitOperations` trait abstraction
- **Same Git compatibility** - Gitoxide maintains full Git compatibility

## Implementation Details

The migration was straightforward thanks to our existing `GitOperations` trait:

1. Added `gix` dependency with minimal features:
   ```toml
   gix = { version = "0.64", default-features = false, features = ["basic", "extras"] }
   ```

2. Reimplemented `GitClient` to use gitoxide APIs:
   - `is_repository()` uses `gix::discover()`
   - `add()` works directly with the Git index
   - `commit()` creates proper commit objects
   - `remove()` handles both index and working tree

3. No other code changes required - the trait abstraction worked perfectly

## Alternatives Considered

1. **libgit2** - More mature but requires C bindings
2. **git2-rs** - Rust bindings for libgit2, but still C underneath
3. **Keep shelling out** - Simple but has all the limitations mentioned above

## References
- [Gitoxide GitHub](https://github.com/GitoxideLabs/gitoxide)
- Git as database philosophy in project README
- Dependency injection pattern used throughout codebase