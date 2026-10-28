# V3-11A Design

Issue: #173

## Objective

Implement the pure governed model for validation manifests, classification, resource profiles, dependencies, and lane selection.

## Scope

`validate plan`, lane manifest schema, PVF classification, proof roles, determinism and live/deferred posture, resource profiles, budgets, parallel-group DAG rules, and planning results.

## Dependencies

- V3-01: issue #161
- V3-06: issue #167

## Architecture Decisions

- `V3-D09`

## Deliverables

- Pure validation-planning domain, manifest schema, exhaustive classification tables, DAG validator, typed errors, and representative plans.

## Owned Paths

- `csdlc-v3/src/pvf/plan/**`
- `csdlc-v3/tests/pvf/plan/**`
- `.csdlc/issues/173/**`
- `.csdlc/prepared/issues/173/**`
- `.csdlc/prepared/issues/173/validate-outcome.rb`
- `.csdlc/evidence/173/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Every lane declares proof role, determinism, resource profile, gate posture, command, timeout, dependencies, and evidence destination.
2. Pending, deferred, blocked, failed, skipped, and passed cannot be conflated.
3. Cycles, duplicate ownership, missing acceptance coverage, and hidden routing policy fail before execution.
4. Planning has no process, network, clock, or filesystem side effects beyond declared input loading.

## PVF Lanes

- `v3-11a-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/173/validate-outcome.rb`.
- `v3-11a-focused-rust`: Run the focused C-SDLC v3 implementation tests owned by this work package. Command: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`.
- `v3-11a-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Exhaustive classification tables, DAG property tests, schema round trips, invalid-plan corpus, deterministic ordering tests, and v2 normalized parity.

## Authority Boundary

- Issue V3-11A owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Process execution, scheduling runtime, timing behavior, evidence writes, cloud runners, review, publication, or authority from planned tests.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- Ordinary test code acquires routing policy, classification depends on ambient state, or a malformed plan can reach execution.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-11a`
