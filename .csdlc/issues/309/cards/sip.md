# Structured Intent Prompt

Template: 1.0.0

Issue: 309

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Reduce the current ADL source surface by deleting every path proven dead or superseded while preserving supported behavior, active Runtime authority, and executable per-band rollback.

## Required Outcome

A complete baseline inventory gives every adl/src Rust file and active reverse reference one accountable disposition; independently reversible deletion bands remove all currently provable dead or superseded code; exact behavior, authority, platform, and rollback proof passes; achieved reduction is reported without a quota.

## Scope

- Immutable adl/src baseline and complete normalized reverse-reference inventory
- Band A dead and unreachable source deletion
- Band B characterized orphan implementation deletion with exact path-manifest and pinned Git identities
- Band C Runtime v2 contraction decision and only already-proven deletions
- Clean-install, exact PR-fast routing, parity, continuity, platform, accounting, and rollback proof
- Exact retained-authority and residual-migration reporting

## Authority

- Current Runtime v2 authorities consumed by Runtime v3, #414 continuity, or supported commands remain protected without a merged parity-proven replacement
- Movement, build exclusion, feature gating, or compatibility copying earns no reduction credit
- Issue #309 owns deletion and its proof; #310 owns later refactoring
- Every baseline file and active reference receives exactly one disposition
- The operator-local planning file is not execution authority or a dependency

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 and an issue-bound FastWork worktree
- Pin execution and rollback to base e926e3bca0ab1981d77b4658d2feb4059bdf33a6
- Use one independently reversible commit per deletion band
- No mandatory percentage, file-count, or line-count target
- Preserve unrelated worktrees and clean primary main
- No AWS or paid validation
