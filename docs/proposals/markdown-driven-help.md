# Markdown-Driven Help System Proposal

**Status: ACCEPTED ✅**  
**Accepted Date: June 12, 2025**  
**Implementation Priority: Phase 1a (Pre-MVP)**  

Date: June 12, 2025  
Author: Claude (with James)

## Executive Summary

Implement a help system where CLI help text is driven by markdown documentation files. This creates a single source of truth for documentation that serves multiple purposes: CLI help output, man pages, and web documentation.

## Problem Statement

Current state:
- Help text is hardcoded in Rust source files
- No man pages for Unix users who expect `man gitmind`
- Documentation drift between CLI help and external docs
- Updating help requires recompilation

## Proposed Solution

### Core Concept

```
docs/cli/*.md (source) → Multiple outputs:
  1. CLI --help (runtime display)
  2. Man pages (build-time generation)  
  3. Web docs (static site generation)
```

### Implementation Details

1. **Markdown Source Files**
   ```
   docs/cli/
   ├── gitmind.md          # Main command help
   ├── gitmind-init.md     # Subcommand documentation
   ├── gitmind-link.md
   ├── gitmind-list.md
   ├── gitmind-unlink.md
   └── gitmind-check.md
   ```

2. **CLI Integration**
   ```rust
   // Embed at compile time for single binary
   const HELP_LINK: &str = include_str!("../docs/cli/gitmind-link.md");
   
   // Render markdown to terminal
   fn show_help(command: &str) {
       let markdown = get_help_markdown(command);
       let rendered = render_for_terminal(markdown);
       println!("{}", rendered);
   }
   ```

3. **Man Page Generation**
   - Use `clap_mangen` or similar at build time
   - Generate from same markdown source
   - Include in distribution packages

## Benefits

1. **Single Source of Truth**
   - All documentation comes from one place
   - No drift between help and docs
   - Version controlled with code

2. **Rich Documentation**
   - Examples with syntax highlighting
   - Structured sections
   - Cross-references between commands

3. **Multiple Output Formats**
   - Terminal help (with optional colors)
   - Traditional man pages
   - Web documentation
   - PDF exports

4. **Developer Experience**
   - Edit markdown, not troff
   - Preview in any markdown editor
   - Easy to contribute to docs

5. **User Experience**
   - Consistent help everywhere
   - Familiar `--help` and `man` commands
   - Rich examples and explanations

## Implementation Phases

### Phase 1: Basic Markdown Help (Pre-MVP)
- Create markdown documentation files
- Implement basic markdown→terminal rendering
- Wire up to `--help` flags

### Phase 2: Man Page Generation (MVP)
- Add build-time man page generation
- Include in release artifacts
- Test on various Unix systems

### Phase 3: Enhanced Rendering (Post-MVP)
- Color support with terminal detection
- Pager integration for long help
- Interactive help browser

## Example Markdown Help File

```markdown
# gitmind link

Create a semantic link between two files in your repository.

## Usage

```
gitmind link <source> <target> [OPTIONS]
```

## Arguments

- `<source>` - Path to the source file
- `<target>` - Path to the target file  

## Options

- `-t, --type <TYPE>` - Type of relationship [default: REFERENCES]
  - `IMPLEMENTS` - Source implements the target specification
  - `REFERENCES` - Source references the target  
  - `INSPIRED_BY` - Source was inspired by target
  - `DEPENDS_ON` - Source depends on target

## Examples

Link implementation to specification:
```bash
gitmind link src/auth.rs docs/auth-spec.md --type IMPLEMENTS
```

Add contextual reference:
```bash
gitmind link README.md LICENSE --message "Mentions license terms"
```

## See Also

- `gitmind list` - View existing links
- `gitmind unlink` - Remove links
- `gitmind check` - Validate link integrity
```

## Rendering Considerations

### Terminal Output
- Strip markdown formatting for basic terminals
- Preserve structure and indentation
- Optional ANSI colors for capable terminals
- Respect NO_COLOR environment variable

### Man Page Output
- Convert to troff format at build time
- Follow man page conventions
- Include in standard sections

## Alternatives Considered

1. **Status Quo** - Keep help in source code
   - ❌ Hard to maintain
   - ❌ No man pages

2. **Direct troff** - Write man pages directly
   - ❌ Arcane syntax
   - ❌ Hard to contribute to

3. **External docs only** - Point to website
   - ❌ Requires internet
   - ❌ Not Unix-like

## Security Considerations

- Markdown files embedded at compile time (no runtime file access)
- No execution of markdown code blocks
- Safe rendering with no HTML injection

## Success Criteria

- Users can run `gitmind --help` and get helpful documentation
- Users can run `man gitmind` on Unix systems
- Documentation updates don't require code changes
- Help text matches web documentation

## References

- [Git's documentation system](https://github.com/git/git/tree/master/Documentation)
- [clap_mangen](https://docs.rs/clap_mangen) - Man page generation
- [termimad](https://docs.rs/termimad) - Terminal markdown rendering