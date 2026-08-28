# C-SDLC v3 construction boundary

This directory is the implementation boundary for the independently planned
C-SDLC v3 work packages in milestone v0.92.1. It is active construction work,
but it is not the repository's operational lifecycle authority yet.

Root `AGENTS.md` and `csdlc-v2/AGENTS.md` remain the current C-SDLC authority
until an explicit operator-reviewed V3-F cutover changes that. Code in this
crate must therefore model, test, and document v3 behavior without binding
worktrees, mutating GitHub, publishing pull requests, finishing issues, cleaning
registered worktrees, retiring v2, or claiming default lifecycle authority.

## Current package slices

- V3-A / #500 established the package contract, predecessor denominator, and
  proportional-lifecycle baseline for the v3 crate.
- V3-B / #501 added the foundation surfaces for issue/repository/application
  state modeling while keeping v3 read-only and non-authoritative.
- V3-C / #502 adds the lifecycle kernel construction slice: typed transition
  decisions, transaction storage/recovery, adapter boundaries, and focused
  transaction tests for retained requirements #168, #169, and #170.

## Clean replacement target

The target is a clean v3 replacement, not permanent v2/v3 coexistence. V3-F or
another explicit cutover issue must prove parity/import, rollback, publication,
finish, cleanup, documentation, operator-start ergonomics, and migration truth
before any default authority moves from v2 to v3.

After that cutover is approved, active v2 command/source surfaces may be
removed in one approval-gated retirement wave. Historical Gate 10A-D records,
review packets, milestone evidence, and rollback evidence remain immutable
project history; do not delete or rewrite them as part of normal v3
construction.

## Operator ergonomics target

A prepared v3 issue should be inspectable, bindable, and startable in three
minutes or less once dependencies are satisfied. Add automation only when it
removes real operator friction while preserving deterministic state,
reviewability, and the current v2 authority boundary.

## Focused local proof

For the current v3 crate surface, use focused Rust proof from the repository
root or bound issue worktree:

```bash
cargo fmt --manifest-path csdlc-v3/Cargo.toml --check
cargo test --manifest-path csdlc-v3/Cargo.toml
cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings
```

These commands validate the construction crate. They do not replace typed v2
issue validation, independent review, publication, finish, or cleanup.

## Non-goals before V3-F

Before the explicit V3-F authority-transition decision, this package must not:

- bind worktrees or mutate `.csdlc/issues/**`;
- call v2 owner binaries, GitHub APIs, provider APIs, process-control commands,
  or shell lifecycle wrappers;
- publish pull requests, mark merge readiness, finish issues, or clean
  worktrees;
- retire v2, weaken v2 invariants, or claim operational cutover.

Unsupported or not-yet-proven behavior fails closed. Treat v3 output as
reviewable cutover evidence, not live lifecycle authority.
