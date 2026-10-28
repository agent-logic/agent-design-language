# V3-D11 Design

Issue: #163

## Objective

Record the measured operator decision required by architecture Decision 11 before transaction storage implementation begins.

## Scope

Per-platform atomic commit primitives, filesystem durability semantics, supported-platform matrix, Windows mutation or fail-closed read-only posture, evidence, and rollback implications.

## Dependencies

- V3-02: issue #162

## Architecture Decisions

- `V3-D11`

## Deliverables

- Retained Decision 11 record tied to V3-02 measurements.
- Approved platform commit matrix with explicit Windows posture and proof requirements.

## Owned Paths

- `docs/adr/**`
- `csdlc-v3/contracts/platform-commit-matrix.*`
- `.csdlc/issues/163/**`
- `.csdlc/prepared/issues/163/**`
- `.csdlc/prepared/issues/163/validate-outcome.rb`
- `.csdlc/evidence/163/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Every supported platform has a measured commit primitive and durability contract.
2. Windows mutation is either equivalently proven or explicitly fail-closed read-only.
3. The operator decision cites exact V3-02 evidence and cannot be inferred from recommendation text.
4. V3-08 remains blocked until this issue is terminal.

## PVF Lanes

- `v3-d11-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/163/validate-outcome.rb`.
- `v3-d11-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Measurement ancestry, platform-row completeness, unsupported-platform negatives, operator authority, and V3-08 dependency validation.

## Authority Boundary

- Issue V3-D11 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Implementing V3-08
- Accepting a V3-02 recommendation as approval
- Claiming unsupported-platform mutation

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- A supported platform lacks measured semantics
- Windows posture is ambiguous
- The decision is not issued by authorized operator review

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#decisions-required-before-implementation`
