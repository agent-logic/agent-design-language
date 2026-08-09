# Issue 79 design: bind-safe deferred Rust targets

## Problem

Pre-bind readiness currently requires every issue-owned Rust source and focused
test target to exist and requires every new Rust module to have an existing
owned production route. That correctly rejects false readiness after
implementation, but creates a circular admission deadlock for a prepared issue
whose exact deliverables are the missing source and test files and whose
implementation policy forbids creating them before bind.

## Decision

Readiness may defer an absent Rust source or test target only while the issue is
initialized and only when all of the following are true:

- the exact repository-relative path is present in both the owned affected-area
  set and the exact deliverables set;
- a fail-closed validation lane selects the exact path and supplies a
  non-placeholder defer reason;
- a deferred production Rust module is paired with an issue-owned deferred test
  target that can route the module through a bounded temporary `#[path]`
  harness;
- the lane remains proving: its Cargo target boundary is explicit and it cannot
  select zero issue-owned targets.

The exception is an admission-only statement of explicit future work. It is not
validation proof. Once the issue is bound or implemented, the declared source,
test target, selected tests, and proof must exist and pass normally.

## Boundaries

- Do not admit arbitrary unroutable production modules.
- Do not admit paths absent from ownership or deliverables.
- Do not admit placeholder or missing defer reasons.
- Do not admit permissive validation policy, missing lanes, or zero-test lanes.
- Do not interpret prose deliverables as repository validator paths.
- Preserve every existing negative false-readiness fixture.

## Proof

Focused Gate 2 fixtures model the three Distributed Guardian children with
distinct issue-owned module and test paths. Positive fixtures prove doctor and
bind admission before file creation. Negative mutations independently remove
ownership, exact deliverables, fail-closed policy, meaningful deferral, and the
temporary harness route. Existing post-bind/implemented checks continue to
prove missing artifacts and absent proof fail closed.
