# V3-10B Design

Issue: #172

## Objective

Deliver semantic card operations and a specific read-only doctor without making rendered Markdown authoritative.

## Scope

`card show/edit/render`, `doctor`, capability-matrix-driven command availability, schema-aware repair planning, projection drift, stranded-state detection, finding taxonomy, next-valid-operation derivation, and human/JSON presentation.

## Dependencies

- V3-05: issue #166
- V3-06: issue #167
- V3-07: issue #168
- V3-08: issue #169
- V3-09: issue #170

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Card command modules and semantic edit operations generated from or mechanically checked against the V3-01 capability matrix, doctor finding registry, read-only repair recommendations, stranded-state detector, projection repair fixtures, and typed result schemas.

## Owned Paths

- `csdlc-v3/src/commands/card/**`
- `csdlc-v3/src/commands/doctor/**`
- `csdlc-v3/tests/commands/card/**`
- `csdlc-v3/tests/commands/doctor/**`
- `.csdlc/issues/172/**`
- `.csdlc/prepared/issues/172/**`
- `.csdlc/prepared/issues/172/validate-outcome.rb`
- `.csdlc/evidence/172/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Card edits mutate semantic values and regenerate all affected projections.
2. Rendered Markdown and stale projections never become input authority.
3. Doctor is read-only, specific, and identifies the next valid operation.
4. Doctor reports a dedicated invariant failure when a wrong or stale acceptance-bearing field has no authorized correction path; ordinary healthy states always receive a capability-derived next operation.
5. Projection drift, invalid schema, unsupported import fields, and topology blockers remain distinguishable.
6. `card show`, `card edit`, and doctor enforce the V3-06 per-phase required and optional field table and its one declared placeholder.

## PVF Lanes

- `v3-10b-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/172/validate-outcome.rb`.
- `v3-10b-focused-rust`: Run the focused C-SDLC v3 implementation tests owned by this work package. Command: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`.
- `v3-10b-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Card schema/structure checks, semantic-edit round trips, matrix-to-command parity, every-phase correction fixtures, stranded-state injection, projection drift/repair fixtures, no-write doctor assertions, finding snapshots, and v2 normalized parity.

## Authority Boundary

- Issue V3-10B owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Binding, PVF execution, formal review, GitHub mutation, automatic repair without typed edit authority, finish, cleanup, or cutover.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- Commands hand-edit rendered files, doctor mutates state, repair invents missing authority, or findings collapse distinct blockers.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-10b`
