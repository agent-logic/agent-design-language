# Sprint 9 Provider Comparison And Convergence Design

## Purpose

Coordinate the v0.92.1 provider and release-tail issues `#515`, `#516`,
`#517`, `#518`, and `#519` without taking over child implementation, proof,
review, publication, or closeout authority.

## Execution Contract

- Use a sequential execution model because each stage consumes the exact
  reviewed terminal output of its predecessor.
- Begin `#515` only after provider-profile issue `#514` is reviewed, merged,
  terminal, and ancestral to the execution base.
- Begin integration admission `#516` only after every root named by its issue
  body is reviewed, merged, terminal, and ancestral, including `#515`.
- Require `#516 -> #517 -> #518 -> #519` with no stage skipping.
- Require issue-bound FastWork worktrees, child-session goals, focused PVF
  proof, exact-head review, green PRs, typed finish, and cleanup for every child.

## Safety Boundaries

- The umbrella writes only its typed coordination record, Sprint Execution
  Packet, readiness evidence, activity log, and integrated sprint review.
- Shadow provider output in `#515` cannot mutate or replace authoritative
  results.
- `#516` cannot repair child lanes or fabricate missing admission authority.
- `#517` rejects missing, failed, skipped, zero-test, or non-proving lanes.
- `#518` reviews exact candidate documentation and records residual risks
  without implementing product fixes.
- `#519` prepares a publication candidate only; merge, tag, release, and public
  publication remain outside Sprint 9 preparation authority.

## Review And Closeout

- Run one sprint-wide readiness review before child execution.
- Run one integrated code/test/docs/security review after all five members are
  terminal.
- Never close `#537` merely because a child is waiting, published, or green;
  closure requires reviewed terminal truth and ancestral merges for every child.

## Non-Goals

- Implementing child work in the umbrella.
- Provider benchmark marketing claims or production cutover.
- Repairing missing milestone roots inside admission or quality-gate children.
- Merging, tagging, releasing, or publicly publishing v0.92.1.
