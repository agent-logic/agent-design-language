# CORP-01 Design

Issue: #153

## Objective

Freeze the complete critical asset, account, owner, custodian, recovery, exclusion, and dependency inventory before any transfer begins.

## Scope

Repositories, domains, brands, source and model IP, cloud and SaaS accounts, billing, credentials, recovery paths, data stores, deployment identities, contracts, and explicit exclusions.

## Dependencies

- No child dependency; setup issue #146 and umbrella readiness only

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Redacted critical-asset register with current and target owner, custodian, recovery authority, transfer method, dependency, and disposition.
- Machine-checkable denominator and exclusion matrix with stable asset identifiers.

## Owned Paths

- `docs/milestones/v0.92.1/evidence/corporate/corp-01/**`
- `docs/operations/corporate/corp-01/**`
- `.csdlc/issues/153/**`
- `.csdlc/prepared/issues/153/**`
- `.csdlc/evidence/153/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Every asset class named by the promoted corporate source has at least one inventoried row or an explicit not-applicable disposition.
2. Each critical row identifies current control, target corporate control, transfer dependency, verification method, rollback posture, and evidence location.
3. The validator rejects duplicate identifiers, missing owners, missing recovery authority, unbounded secret fields, and unapproved exclusions.
4. No transfer or credential rotation occurs in this inventory issue.

## PVF Lanes

- `corp-01-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/153/validate-outcome.rb`.
- `corp-01-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Schema, denominator, duplicate, required-field, exclusion, redaction, and cross-reference validation over the retained inventory.

## Authority Boundary

- Issue CORP-01 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Executing transfers
- Storing secrets or private legal instruments
- Inferring ownership from billing screenshots alone

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- A critical asset has unknown ownership or custody
- A secret or private instrument would enter the repository
- The denominator cannot be reconciled with company and founder accounts

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#corp-01`
