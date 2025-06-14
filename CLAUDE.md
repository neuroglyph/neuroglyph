# Instructions for Claude and AI Assistants

This file contains important instructions for Claude and other AI assistants working on the Neuroglyph project.

## Project Overview

- **Project Name**: Neuroglyph
- **CLI Name**: gitmind
- **License**: Apache 2.0 (NOT MIT!)
- **Copyright**: © 2025 J. Kirby Ross / Neuroglyph Collective
- **Repository**: https://github.com/neuroglyph/neuroglyph

## Critical Rules

### 1. NEVER Make Git Commits or Operations in Working Repo
- The user will explicitly ask if they want commits made
- Default to NO commits unless specifically requested
- NEVER run git operations in the working repository
- All git operations must be in Docker containers or temp directories
- This is a hard rule with no exceptions

### 2. License Headers (SPDX)

All new files MUST include SPDX headers:

#### Source Code Files (.rs, .js, .ts, .py, .sh, etc.)
```rust
// SPDX-License-Identifier: Apache-2.0
// © 2025 J. Kirby Ross / Neuroglyph Collective
```

#### Shell Scripts
```bash
#!/bin/bash
# SPDX-License-Identifier: Apache-2.0
```

#### Configuration Files (Dockerfile, Makefile, .yml, .toml, .json)
```yaml
# SPDX-License-Identifier: Apache-2.0
```

#### Markdown Files (optional but encouraged)
```markdown
<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- © 2025 J. Kirby Ross / Neuroglyph Collective -->
```

### 3. License Information
- This project uses **Apache License 2.0** exclusively
- NEVER reference MIT license for this project
- All contributions are under Apache 2.0
- Check LICENSE file if unsure

### 4. Project Structure
This is a monorepo with the following structure:
```
neuroglyph/
├── c/             # gitmind CLI (Pure C) - current implementation
├── demos/         # Example applications
├── design/        # Technical specifications
├── docs/          # User documentation  
├── lore/          # Philosophy, Gonzai mascot, creative content
└── scripts/       # Development scripts
```

**Important:** There is NO root Makefile. Each component (like c/) has its own build system. Always cd into the component directory before running make commands.

### 5. Development Practices

#### Development Philosophy
- **TDD (Test-Driven Development)**: Write tests first, then implementation
- **SOLID Principles**: Single responsibility, Open/closed, Liskov substitution, Interface segregation, Dependency inversion
- **KISS**: Keep It Simple, Stupid - avoid unnecessary complexity
- **YAGNI**: You Aren't Gonna Need It - don't add functionality until needed
- **Test Double-Friendly Design**: Use dependency injection and traits to enable test doubles when needed

#### Testing
- All tests run in Docker for consistency
- Use `cd c && make test` to run the full test suite
- Pre-push hooks enforce test passing
- Write tests BEFORE implementation (TDD)
- Each function should have corresponding tests
- **Always use real Git repos in tests** - our entire stack relies on Git
- **NEVER run Git operations in the actual repo** - only in Docker/temp dirs
- Create temporary Git repositories for each test
- All Git operations must be isolated from the working repository
- Use dependency injection and traits for clean architecture
- Test doubles are only for contriving edge cases, not replacing Git
- **Always test behavior, not implementation** - Tests verify what the system does, not how
- **One test file per component** - Each test file focuses on testing one component's behavior

##### Test Strategy - Three Levels
Always have these three levels of tests, all focused on behavior:
1. **Unit Tests** - Test individual components in isolation
   - Located in `src/` files next to implementation (e.g., `#[cfg(test)] mod tests`)
   - Test single structs/functions behavior
   - Fast, focused on one thing
2. **Integration Tests** - Test components working together
   - Located in `tests/` directory
   - Test command behavior, file I/O, Git operations
   - Use real Git repos, real file systems
3. **End-to-End Tests** - Test complete user workflows
   - Also in `tests/` directory
   - Test full CLI commands from user perspective
   - Verify entire features work as users expect
   - **Every user story in `design/features/` should have corresponding end-to-end tests**
   - **Every acceptance criteria should have its own specific test**

##### Testing Principles - NEVER Test stdout/stderr
- **NEVER test stdout/stderr output** - This is brittle and couples tests to implementation
- Testing stdout is just spying by another name - it violates behavior testing
- Instead, use proper return types:
  - Commands return `CommandResult<T>` with exit code and optional value
  - Tests check the exit code and returned values directly
  - The CLI layer handles printing - tests verify behavior, not output
- Example: Don't test "No links found" string, test that `list()` returns empty Vec

##### Definition of Done for Features
A feature is only complete when:
1. All acceptance criteria from `design/features/F*.md` have corresponding tests
2. Tests pass at all three levels (unit, integration, end-to-end)
3. User story is demonstrable through end-to-end tests
4. Documentation is updated
5. Code follows project conventions

#### Git Hooks
- Git LFS is configured for binary files
- Custom hooks in `scripts/` directory
- Install with `make install-hooks`

#### Code Style
- Rust code uses standard formatting (`cargo fmt`)
- Always run clippy (`cargo clippy`)
- Follow conventional commits
- Keep functions small and focused (SOLID)
- Avoid premature optimization (YAGNI)
- **One file = one thing** - Each file contains one struct/enum/trait/type
- **Single Responsibility Principle** - Each module/struct has one reason to change

### 6. Key Technical Details

#### Storage Model
- Links stored in `.gitmind/links/` directory
- Format: `LINK_TYPE: source -> target  # ts:timestamp`
- Files named by SHA of content
- **Git IS the database** - no external storage
- Every operation uses Git's content-addressable storage
- Testing must use real Git operations to be valid

#### Architecture
- CLI first approach (no server required)
- Optional daemon for web UI
- Distributed by design
- Content-addressable storage

### 7. Mascot & Tone
- **Gonzai** 🐵 is the chaos monkey mascot
- Playful but professional tone
- "Git as cognition layer" is the tagline
- Emphasize discovery through chaos

### 8. Important Files to Read
When starting work, always check:
1. `TASKLIST.md` - Current implementation status
2. `README.md` - Project overview
3. `docs/README.md` - Technical roadmap
4. `design/gitmind_architecture.md` - Architecture details
5. This file (`CLAUDE.md`) - For updates

## Common Tasks

### Adding a New Source File
1. Add SPDX header (see above)
2. Follow project conventions
3. Add tests
4. Update documentation if needed

### Updating Documentation
1. Add SPDX headers to new docs (optional but encouraged)
2. Keep technical docs in `/docs/` or `/design/`
3. Keep creative content in `/lore/`
4. Update `TASKLIST.md` when completing tasks

### Working with Git
1. NEVER commit without explicit permission
2. Use conventional commits when asked to commit
3. Check for Git LFS files in `.gitattributes`
4. Run tests before any push (automatic via hooks)
5. When asked for commit messages, provide them in this format:
   ```bash
   git add <files> && git commit -m "type(scope): message"
   ```
6. Suggest commits after completing each task from TASKLIST.md
7. Keep TASKLIST.md up-to-date as tasks are completed

## Development Workflow Guidelines

### Tech Design and Feature Tracking
- Whenever we make a significant tech design decision, document it in `design/decisions`
- Whenever we complete a feature, update the document in `design/features` to reflect its status

## Final Reminders
- Apache 2.0, not MIT!
- No commits without permission!
- SPDX headers on all new files!
- Test everything in Docker!
- Keep Gonzai chaotic! 🐵✨

## Development Workflow Example

When implementing a new feature:
1. **Start with tests** (TDD):
   ```rust
   #[test]
   fn test_init_creates_directory() {
       // Create isolated temp directory for test
       let temp_dir = TempDir::new()?;
       // Initialize git repo in temp dir, NOT in working repo
       let git_ops = GitOperations::new(temp_dir.path());
       git_ops.init_repository()?;
       assert!(temp_dir.path().join(".gitmind/links").exists());
   }
   ```
2. **Design with traits and DI** for testability:
   ```rust
   trait GitBackend {
       fn init(&self) -> Result<()>;
       fn add_file(&self, path: &Path) -> Result<()>;
   }
   
   struct LinkManager<G: GitBackend> {
       git: G,
   }
   ```
3. **Write minimal code** to make tests pass (KISS)
4. **Use real Git repos** in tests, test doubles only when needed
5. **Refactor** if needed while keeping tests green
6. **Don't add extra features** (YAGNI)
7. **Keep each module focused** on one responsibility (SRP)

## Important Development Practices

### Logging and Journal
- Leave your impressions or thoughts in `lore/claude-mind/YYYY-MM-DD/` directory
- Always use today's actual date (double-check to avoid date errors!)
- Create per-session files to avoid merge conflicts:
  - Format: `HH:MM-topic-sessionID.md` (e.g., `14:45-web-viz-abc123.md`)
  - Use first 6 chars of conversation ID for sessionID
- Use ISO 8601 timestamps in UTC (e.g., `2025-06-12T14:45:00Z`)
- Include session metadata at the start of each file:
  ```markdown
  # Claude Development Journal
  
  ## Session: 2025-06-12T14:45:00Z
  **Collaborator**: j***s (a1b2c3)  
  **Topics**: Web visualization, Help system  
  **Conversation**: abc123def456 (first 12 chars)
  
  ### 14:45 UTC - Initial Thoughts
  [content...]
  ```
- For collaborator identification:
  - Use git user's GPG key fingerprint (last 16 chars)
  - Example: `**Collaborator**: James (GPG: 1234567890ABCDEF)`
  - This leverages the project's requirement for signed commits
  - Could optionally sign the journal entry itself with their key
- This is your dev journal - feel free to write freely when you get ideas or insights
- Create an `index.md` in each day's directory summarizing sessions

---
*Last updated: June 2025*