# Link Storage Format Specification

## File Format

Each link is stored as a single line in a `.link` file:

```
LINK_TYPE: source_path -> target_path  # ts:unix_timestamp
```

## File Naming

Files are named using SHA-1 of their content:
```
.gitmind/links/a3f5c8d9e2b1d7f6a8c4e9b2d5f8a3c6d9e2b1d7.link
```

## Link Types

- **IMPLEMENTS** - Source implements concepts from target
- **REFERENCES** - Source references target  
- **INSPIRED_BY** - Source was inspired by target
- **DEPENDS_ON** - Source depends on target
- **CONTRADICTS** - Source contradicts target
- **DISCUSSES** - Source discusses target
- **REVIEWS** - Source reviews target

## Examples

```
IMPLEMENTS: specs/auth.md -> rfc/oauth2.md  # ts:1641234567
REFERENCES: docs/api.md -> specs/auth.md  # ts:1641234590
INSPIRED_BY: ideas/ml-integration.md -> papers/neural-nets.pdf  # ts:1641234600
```

## Design Rationale

- Human-readable for debugging
- Git-diff friendly
- Deterministic filenames prevent duplicates
- Timestamps enable temporal queries
- Simple parsing