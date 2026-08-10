# INT-02 Design

Issue: #189

## Objective

Prepare the exact release candidate, rollback rehearsal, and operator ceremony after integrated review passes.

## Scope

Artifact inventory, version and revision pinning, release checklist, company authority, change window, rollback triggers and commands, rehearsal, communications, and abort conditions.

## Dependencies

- INT-01: issue #188

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Exact release-candidate manifest and signed-off checklist.
- Executed rollback rehearsal and operator ceremony runbook with abort evidence.

## Owned Paths

- `docs/milestones/v0.92.1/evidence/integration/int-02/**`
- `.csdlc/issues/189/**`
- `.csdlc/prepared/issues/189/**`
- `.csdlc/prepared/issues/189/validate-outcome.rb`
- `.csdlc/evidence/189/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Every release artifact, source revision, schema, evidence bundle, and external dependency is pinned.
2. Rollback is rehearsed from the candidate without data loss, dual authority, or hidden personal credentials.
3. Named company authority, observers, timing, abort triggers, communication, and post-release verification are recorded.
4. No release action occurs until the final operator authorization.

## PVF Lanes

- `int-02-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/189/validate-outcome.rb`.
- `int-02-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Manifest digest, release checklist, authority readback, rollback execution, data/authority invariants, abort path, and independent ceremony review.

## Authority Boundary

- Issue INT-02 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Publishing without authorization
- Changing accepted lane outputs
- Treating a written rollback plan as rehearsal

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- Candidate inputs drift
- Rollback fails
- Authority is ambiguous
- An unresolved blocker or external dependency appears

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#int-02`
