# CORP-08 Design

Issue: #160

## Objective

Close the corporate chain-of-title and operational due-diligence package without overstating legal or production completion.

## Scope

All CORP-01 through CORP-07 evidence, critical schedule, exclusions, residual risks, private-custody index, public redacted index, counsel and corporate approvals, and release recommendation.

## Dependencies

- CORP-02: issue #154
- CORP-03: issue #155
- CORP-05: issue #157
- CORP-07: issue #159

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Critical schedule completion matrix with evidence and unresolved exceptions.
- Redacted due-diligence index and private custody map accepted by corporate authority and reviewed by counsel where required.

## Owned Paths

- `docs/milestones/v0.92.1/evidence/corporate/corp-08/**`
- `docs/operations/corporate/corp-08/**`
- `.csdlc/issues/160/**`
- `.csdlc/prepared/issues/160/**`
- `.csdlc/evidence/160/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Every critical asset and service has a terminal transferred, retained, excluded, or blocked disposition.
2. All required counsel and corporate approvals are present and bound to exact evidence.
3. The public index is redacted and recomputable without exposing private instruments or credentials.
4. Any unresolved critical exception blocks the corporate release gate and is not downgraded to residual risk without explicit authority.

## PVF Lanes

- `corp-08-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/160/validate-outcome.rb`.
- `corp-08-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Denominator reconciliation, receipt digest, custody, approval, redaction, exception, and independent diligence-readiness review.

## Authority Boundary

- Issue CORP-08 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Declaring legal sufficiency without counsel
- Publishing private diligence materials
- Waiving critical exceptions through prose

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- A critical schedule row is unresolved
- Private custody cannot be verified
- Counsel or corporate acceptance is missing
- Evidence cannot be independently recomputed

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#corp-08`
