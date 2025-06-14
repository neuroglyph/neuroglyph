# F025: CLI Help System

**Status:** ACCEPTED  
**Priority:** High (Pre-MVP)  
**Effort:** 3 days  
**Dependencies:** Core CLI functionality  

## Summary

Implement a comprehensive help system for the GitMind CLI that uses markdown files as the single source of truth for all documentation formats, including inline help, man pages, and web documentation.

## User Stories

### 1. The Quick Checker
As a command-line user, I want to run `gitmind --help` to see a quick overview of available commands, so I can find what I need without leaving the terminal.

### 2. The Deep Learner  
As a new user, I want to run `gitmind link --help` to see detailed documentation with examples, so I can understand how to use each command effectively.

### 3. The Unix Traditionalist
As a long-time Unix user, I want to run `man gitmind` to read documentation in my preferred format, so I can use familiar tools like search and navigation.

### 4. The Offline Worker
As someone working without internet, I want all help documentation available locally, so I can be productive anywhere.

## Acceptance Criteria

### 1. Markdown Documentation Files
- [ ] Create `docs/cli/` directory structure
- [ ] Write comprehensive markdown for each command:
  - [ ] `gitmind.md` - Overview and global options
  - [ ] `gitmind-init.md` - Initialize repository
  - [ ] `gitmind-link.md` - Create links
  - [ ] `gitmind-list.md` - List links
  - [ ] `gitmind-unlink.md` - Remove links  
  - [ ] `gitmind-check.md` - Validate links
- [ ] Follow consistent structure: Usage, Arguments, Options, Examples, See Also
- [ ] Include real-world examples for each command

### 2. CLI Integration
- [ ] `--help` flag shows rendered markdown for each command
- [ ] `gitmind help <command>` alternative syntax
- [ ] `gitmind --version` shows version info
- [ ] Help text embedded in binary at compile time
- [ ] Graceful fallback if markdown rendering fails

### 3. Terminal Rendering
- [ ] Strip markdown formatting for basic display
- [ ] Preserve code block indentation
- [ ] Maintain readable structure
- [ ] Optional: Detect terminal capabilities
- [ ] Optional: Add color for capable terminals
- [ ] Respect NO_COLOR environment variable

### 4. Man Page Generation
- [ ] Build-time generation from markdown
- [ ] Proper man page formatting:
  - [ ] NAME section
  - [ ] SYNOPSIS section  
  - [ ] DESCRIPTION section
  - [ ] OPTIONS section
  - [ ] EXAMPLES section
  - [ ] SEE ALSO section
- [ ] Generate for main command and subcommands
- [ ] Include in release artifacts

### 5. Help Content Quality
- [ ] Clear, concise descriptions
- [ ] Practical examples for each command
- [ ] Common use cases covered
- [ ] Error scenarios explained
- [ ] Cross-references between related commands

## Technical Design

### Architecture
```
docs/cli/*.md → Build Process → Binary with embedded help
                              ↘ Generated man pages
```

### Implementation Details

#### 1. Embedding Documentation
```rust
// In build.rs or source files
const HELP_INIT: &str = include_str!("../../docs/cli/gitmind-init.md");
const HELP_LINK: &str = include_str!("../../docs/cli/gitmind-link.md");
// ... etc

struct HelpDocs {
    main: &'static str,
    init: &'static str,
    link: &'static str,
    list: &'static str,
    unlink: &'static str,
    check: &'static str,
}

static HELP: HelpDocs = HelpDocs {
    main: include_str!("../../docs/cli/gitmind.md"),
    // ... etc
};
```

#### 2. Markdown Rendering
```rust
use termimad::MadSkin;

fn render_help(markdown: &str, use_color: bool) -> String {
    if use_color && terminal_supports_color() {
        let skin = MadSkin::default();
        skin.term_text(markdown)
    } else {
        strip_markdown_simple(markdown)
    }
}

fn strip_markdown_simple(md: &str) -> String {
    // Basic stripping of markdown syntax
    // Preserve structure and code blocks
}
```

#### 3. Man Page Generation (build.rs)
```rust
use clap_mangen::Man;

fn generate_man_pages() {
    let app = build_cli();
    let man = Man::new(app);
    
    let mut buffer = Vec::new();
    man.render(&mut buffer).unwrap();
    
    std::fs::write("target/man/gitmind.1", buffer).unwrap();
}
```

### Documentation Standards

Each markdown file should follow this structure:

```markdown
# gitmind [command]

Brief one-line description.

## Usage

```
gitmind [command] [arguments] [options]
```

## Arguments

- `<arg1>` - Description
- `[arg2]` - Optional argument description

## Options

- `-s, --short` - Short description
- `--long-flag <VALUE>` - Longer description

## Examples

Basic usage:
```bash
gitmind command arg1 arg2
```

Advanced usage:
```bash
gitmind command --option value
```

## See Also

- `gitmind other-command` - Related functionality
- Project documentation: https://neuroglyph.dev
```

## Testing Strategy

### Unit Tests
- Markdown rendering functions
- Help text retrieval
- Terminal capability detection
- Fallback behavior

### Integration Tests  
- All commands have help text
- Help flags work correctly
- Man page generation succeeds
- No missing documentation

### Manual Testing
- Various terminal emulators
- NO_COLOR environment
- Pager behavior (less, more)
- Man page formatting

## Implementation Plan

### Phase 1: Create Documentation (1 day)
1. Set up `docs/cli/` structure
2. Write markdown for all commands
3. Review for consistency
4. Add examples

### Phase 2: Basic Integration (1 day)
1. Embed markdown in binary
2. Add --help flag handling  
3. Basic markdown stripping
4. Test all commands

### Phase 3: Enhanced Features (1 day)
1. Man page generation
2. Terminal color detection
3. Better markdown rendering
4. Package for distribution

## Success Metrics
- All commands have comprehensive help
- Users can work entirely offline
- Help is actually helpful (user feedback)
- Man pages available on Unix systems
- Documentation stays in sync

## Future Enhancements
- Interactive help browser
- Context-sensitive help
- Localization support (i18n)
- Video tutorials linked from help
- Shell completion generation

## Related Documents
- [ADR-006: Markdown-Driven Help](../decisions/ADR-006-markdown-driven-help.md)
- [Markdown-Driven Help Proposal](../proposals/markdown-driven-help.md)
- [F013: CLI Tools](F013-cli-tools.md) - Core CLI implementation