# Semantic Layer Concept

## The Problem

Current tools treat files as isolated islands. Relationships exist in our minds but not in our tools.

## The Vision

What if every file knew its relationships?
- "This implements that"
- "This was inspired by that"  
- "This depends on that"
- "This contradicts that"

## Implementation Ideas

### Link Types
- IMPLEMENTS
- REFERENCES
- INSPIRED_BY
- DEPENDS_ON
- CONTRADICTS
- DISCUSSES
- REVIEWS

### Storage Format
Simple, human-readable:
```
IMPLEMENTS: design.md -> code.rs  # ts:1234567890
```

### Discovery Mechanisms
- Manual linking via CLI
- Automatic extraction from Markdown
- Pattern detection
- Chaos mode speculation

## Benefits
- Knowledge becomes traversable
- Context travels with content
- Onboarding is instant
- Understanding is shareable