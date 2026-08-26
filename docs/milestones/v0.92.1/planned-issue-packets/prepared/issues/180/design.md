# V3-R01 Design

Issue: #180

## Objective

Remove v2 operational authority only after v3 has satisfied the approved stability window and every retained record has a terminal disposition.

## Scope

Eligibility proof, retained importer decision, forbidden-path inventory, binary/skill/selector removal, historical evidence preservation, documentation cleanup, and final no-v2-authority verification.

## Dependencies

- V3-16: issue #179

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Deletion manifest, eligibility decision, retained-evidence inventory, reviewed removal diff, clean installation inventory, and final authority audit.

## Owned Paths

- `csdlc-v2/**`
- `csdlc-v2 operational binaries`
- `docs/tooling/**`
- `.csdlc/issues/180/**`
- `.csdlc/prepared/issues/180/**`
- `.csdlc/prepared/issues/180/validate-outcome.rb`
- `.csdlc/evidence/180/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Every deletion target is classified before mutation.
2. Historical Gate and migration evidence remains readable and immutable.
3. No v2 executable, operator skill, selector route, or writable state authority remains after removal.
4. V3 can install, validate, review, publish, finish, and clean from a fresh checkout without v2 artifacts.

## PVF Lanes

- `v3-r01-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/180/validate-outcome.rb`.
- `v3-r01-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Exact deletion list, pre/post authority inventories, fresh-install journey, forbidden-path scan, full v3 regression, historical evidence readability check, and independent deletion review.

## Authority Boundary

- Issue V3-R01 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- V3 feature work, migration repair hidden inside deletion, removal of immutable historical evidence, or waiver of unresolved stability findings.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- The rollback window is active, any issue still requires v2 writes, eligibility evidence is stale, deletion touches historical evidence, or the operator has not explicitly approved removal.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-r01`
