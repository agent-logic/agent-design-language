# V3-07 Design

Issue: #168

## Objective

Encode lifecycle transitions and authorization predicates as a pure, exhaustive, side-effect-free state machine.

## Scope

Phases, transition commands, preconditions, topology ownership, design/readiness/review/publication/terminal predicates, capability-derived field authorization, recovery reachability, idempotent outcomes, and stable domain errors.

## Dependencies

- V3-06: issue #167

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Pure transition API, transition-and-correction table generated from the V3-01 capability matrix, recovery graph, invariant/property tests, negative transition corpus, idempotency model, and v2 behavior mapping.

## Owned Paths

- `csdlc-v3/src/lifecycle/**`
- `csdlc-v3/tests/lifecycle/**`
- `.csdlc/issues/168/**`
- `.csdlc/prepared/issues/168/**`
- `.csdlc/prepared/issues/168/validate-outcome.rb`
- `.csdlc/evidence/168/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Every state/command pair has an explicit allowed or rejected outcome.
2. The compiler enforces exhaustive closed-state handling.
3. Branch/worktree topology is the only local ownership authority.
4. Review staleness, publication gates, terminal truth, and cleanup eligibility remain fail-closed.
5. Every accepted recovery transition preserves a reachable typed correction or typed terminal-disposition command; no supported state is a lifecycle dead end and no abstract operator-required sink satisfies reachability.
6. The generated transition table accepts `review recover` only from `reviewed`, `published`, or `merge_ready`, returns to `implemented`, rejects `merged` and `closed_out`, and proves the matrix-declared atomic invalidations.
7. Removing or changing any authorization predicate causes mutation/property tests to fail, including correction invalidation and stale-CAS predicates.
8. Cleanup eligibility requires committed `closed_out` state and a retained terminal receipt; remote merge observation alone is insufficient.

## PVF Lanes

- `v3-07-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/168/validate-outcome.rb`.
- `v3-07-focused-rust`: Run the focused C-SDLC v3 implementation tests owned by this work package. Command: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`.
- `v3-07-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Complete transition-and-correction table tests, graph reachability for every supported recovery state, property tests for invariants and idempotency, mutation testing of authorization and rejection predicates, and normalized v2 parity cases including every retained v2 recovery defect.

## Authority Boundary

- Issue V3-07 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- File writes, Git commands, GitHub calls, clock reads, prompting, process execution, or retry policy.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- A transition needs ambient I/O, an unknown state falls through, or claims, leases, heartbeats, or protected-path ledgers reappear as authority.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-07`
