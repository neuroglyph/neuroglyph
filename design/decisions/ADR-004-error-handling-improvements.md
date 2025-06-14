# ADR-004: Error Handling Improvements

**Status:** Accepted  
**Date:** June 12, 2025  
**Deciders:** James (Project Lead), Claude (Technical Advisor)  

## Context

During the Rust code audit, we identified critical issues with error handling:
1. Raw Git errors were being passed through to users (e.g., "fatal: Unable to create '.git/index.lock': File exists")
2. No consistent error classification or recovery guidance
3. Users encountering errors had no clear path to resolution

The audit revealed this was violating our user-centric philosophy - errors should be teaching moments, not dead ends.

## Decision

We will implement comprehensive error handling improvements as specified in [F017: Error Handling Improvements](../features/F017-error-handling-improvements.md), based on the [Error Handling Improvement Proposal](../proposals/error-handling-improvement.md).

Key components:
1. Error classification system (Transient, Configuration, Corruption, etc.)
2. Context-aware error messages with plain English explanations
3. Actionable solutions for each error type
4. Error codes for reference and help system integration

## Rationale

### Why Improve Error Handling?

Errors are critical touchpoints in the user journey. When users encounter cryptic Git errors, they often:
- Abandon the tool
- Search online for solutions (breaking flow)
- Develop negative associations with the software

By transforming errors into helpful guidance, we:
- Reduce user frustration
- Increase successful task completion
- Build user confidence with Git concepts
- Create a competitive advantage (most tools have poor error messages)

### Why Now (Pre-MVP)?

Initially, error handling seemed like a "nice to have" post-MVP feature. However, the audit revealed it's essential for a viable product:
- Common scenarios (index.lock, no HEAD) happen frequently
- Without good error handling, early users will have bad experiences
- First impressions matter for adoption

## Consequences

### Positive
- **Better UX**: Users get helpful guidance instead of cryptic errors
- **Faster Resolution**: Solutions provided directly, no searching needed
- **Learning Opportunity**: Each error teaches Git concepts
- **Reduced Support**: Fewer users need help with common issues
- **Professional Feel**: Polished error handling signals quality software

### Negative
- **Additional Complexity**: Error classification and message generation
- **More Testing**: Need comprehensive edge case coverage
- **Maintenance**: Error messages need updates as Git evolves
- **Translation**: Future i18n will require translating many messages

### Neutral
- **Dependencies**: Adding `colored` for terminal colors, `backoff` for retries
- **Code Structure**: New error handling layer throughout codebase
- **Documentation**: Need to document all error codes

## Implementation Priority

This is a **Pre-MVP requirement**. The [Git Edge Cases Audit](../audits/git-edge-cases-audit.md) identified 25+ scenarios that users will encounter. At minimum, we must handle:

1. Index lock contention
2. No HEAD reference (empty repo)
3. Disk full errors
4. Permission denied
5. Missing Git configuration

## Success Metrics
- 90% of errors provide actionable solutions
- Average time to resolve errors reduced by 50%
- User feedback indicates errors are helpful
- Support requests for errors reduced by 75%

## References
- [Error Handling Improvement Proposal](../proposals/error-handling-improvement.md)
- [F017: Error Handling Improvements](../features/F017-error-handling-improvements.md)
- [Git Edge Cases Audit](../audits/git-edge-cases-audit.md)
- [Test Double Audit](../audits/test-double-audit.md)

---

*"Every error should leave the user more capable than before."*