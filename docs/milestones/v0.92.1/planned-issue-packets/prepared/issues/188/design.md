# INT-01 Design

Issue: #188

## Objective

Run one independent integrated review over terminal corporate, C-SDLC v3, and Runtime qualification evidence and remediate every blocker.

## Scope

Exact terminal lane revisions, proof inventories, cross-lane assumptions, release gates, findings, dispositions, residual risks, and remediation verification.

## Dependencies

- CORP-08: issue #160
- V3-16: issue #179
- DRT-07: issue #187

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Findings-first integrated review at exact lane revisions.
- Disposition and remediation ledger with a bounded release recommendation.

## Owned Paths

- `docs/milestones/v0.92.1/evidence/integration/int-01/**`
- `.csdlc/issues/188/**`
- `.csdlc/prepared/issues/188/**`
- `.csdlc/prepared/issues/188/validate-outcome.rb`
- `.csdlc/evidence/188/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. CORP-08, V3-16, and DRT-07 are terminal and exact revisions are ancestral to the review revision.
2. Every required lane artifact and quality gate is independently recomputed or explicitly rejected.
3. All P1/P2 findings receive verified terminal dispositions before recommendation.
4. The review does not treat one lane's success as evidence for another lane.

## PVF Lanes

- `int-01-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/188/validate-outcome.rb`.
- `int-01-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Terminal ancestry, evidence inventory, validator rerun, finding/disposition closure, cross-lane independence, and exact-head review checks.

## Authority Boundary

- Issue INT-01 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Implementing undisclosed remediation
- Waiving blockers
- Publishing a release

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- A lane is nonterminal
- Evidence cannot be reproduced
- A blocking finding remains unresolved
- Review independence cannot be established

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#int-01`
