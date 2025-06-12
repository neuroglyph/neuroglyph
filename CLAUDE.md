# Instructions for Claude and AI Assistants

This file contains important instructions for Claude and other AI assistants working on the Neuroglyph project.

## Project Overview

- **Project Name**: Neuroglyph
- **CLI Name**: gitmind
- **License**: Apache 2.0 (NOT MIT!)
- **Copyright**: © 2025 J. Kirby Ross / Neuroglyph Collective
- **Repository**: https://github.com/neuroglyph/neuroglyph

## Critical Rules

### 1. NEVER Make Git Commits
- The user will explicitly ask if they want commits made
- Default to NO commits unless specifically requested
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
├── cli/           # gitmind CLI (Rust) - main component
├── demos/         # Example applications
├── design/        # Technical specifications
├── docs/          # User documentation  
├── lore/          # Philosophy, Gonzai mascot, creative content
├── scripts/       # Development scripts
└── testdata/      # Test fixtures
```

### 5. Development Practices

#### Testing
- All tests run in Docker for consistency
- Use `make test` to run the full test suite
- Pre-push hooks enforce test passing

#### Git Hooks
- Git LFS is configured for binary files
- Custom hooks in `scripts/` directory
- Install with `make install-hooks`

#### Code Style
- Rust code uses standard formatting (`cargo fmt`)
- Always run clippy (`cargo clippy`)
- Follow conventional commits

### 6. Key Technical Details

#### Storage Model
- Links stored in `.gitmind/links/` directory
- Format: `LINK_TYPE: source -> target  # ts:timestamp`
- Files named by SHA of content
- No external database - Git IS the database

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

## Final Reminders
- Apache 2.0, not MIT!
- No commits without permission!
- SPDX headers on all new files!
- Test everything in Docker!
- Keep Gonzai chaotic! 🐵✨

---
*Last updated: June 2025*