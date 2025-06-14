# Repository Reorganization - June 14, 2025

## Summary

Reorganized the Neuroglyph repository structure for better clarity and organization.

## Changes Made

### New Directory Structure

```
neuroglyph/
├── c/                   # gitmind CLI implementation
├── demos/               # Example applications  
├── design/              # Technical design documentation
│   ├── features/        # Feature specifications
│   │   ├── active/      # Currently being implemented
│   │   ├── planned/     # Future features
│   │   └── completed/   # Implemented features
│   ├── decisions/       # Architecture Decision Records (ADRs)
│   ├── proposals/       # Design proposals
│   ├── research/        # Research documents
│   ├── ideas/           # Experimental ideas
│   └── issues/          # Technical issues
├── docs/                # User documentation
│   └── cli/             # CLI documentation and man pages
├── lore/                # Philosophy & creative content
├── project/             # Project management
│   ├── community/       # Community guidelines
│   ├── legal/           # Legal & security docs
│   └── meta/            # Project metadata
│       ├── audits/      # Code audits
│       └── archive/     # Historical docs
└── scripts/             # Development scripts
    ├── development/     # Dev tools
    ├── deployment/      # Deploy scripts
    └── testing/         # Test utilities
```

### Files Moved

#### From docs/ to design/:
- `features/` → `design/features/` (organized by status)
- `decisions/` → `design/decisions/`
- `proposals/` → `design/proposals/`
- `research/` → `design/research/`
- `ideas/` → `design/ideas/`
- `issues/` → `design/issues/`
- `ARCHITECTURE*.md` → `design/`
- `gitmind-whitepaper.md` → `design/`

#### From root to project/:
- `CODE_OF_CONDUCT.md` → `project/community/`
- `CONTRIBUTING.md` → `project/community/`
- `ETHICAL_USE.md` → `project/community/`
- `COPYRIGHT.md` → `project/legal/`
- `SECURITY.md` → `project/legal/`
- `CLAUDE.md` → `project/meta/`
- `MONOREPO.md` → `project/meta/`
- `TASKLIST.md` → `project/meta/`

#### Other moves:
- `docs/early-testers.md` → `project/community/`
- `docs/audits/` → `project/meta/audits/`
- `docs/archive/` → `project/meta/archive/`

### Documentation Updates

1. Created new README files:
   - `design/README.md` - Technical documentation overview
   - `project/README.md` - Project management overview
   - `scripts/README.md` - Scripts documentation

2. Updated `docs/README.md` to focus on user documentation

3. Updated all file references in:
   - Root `README.md`
   - `project/meta/CLAUDE.md`

### Features Organization

Features are now organized by implementation status:
- **completed/** - Fully implemented features (F001, F013, F016, F026)
- **active/** - Currently being worked on (F002-F031)
- **planned/** - Future features (none currently)

## Benefits

1. **Clearer separation** between technical design and user documentation
2. **Better organization** of project management files
3. **Status-based** feature tracking
4. **Centralized** project metadata
5. **Cleaner** root directory

## Next Steps

1. Update any remaining broken references
2. Consider moving feature test mapping to a more appropriate location
3. Update CI/CD scripts if they reference moved files