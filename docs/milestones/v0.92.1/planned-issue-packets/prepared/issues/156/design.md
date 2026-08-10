# CORP-04 Design

Issue: #156

## Objective

Establish company-controlled billing, administration, MFA, recovery, vault, break-glass, and custody for every critical service.

## Scope

Company identities, billing profiles, secure MFA, recovery channels, vault custody, break-glass procedure, least privilege, role separation, and founder-dependency removal.

## Dependencies

- CORP-01: issue #153

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Redacted service custody matrix and recovery test record.
- Company-controlled administrative and billing readback with named role, not credential, ownership.

## Owned Paths

- `docs/milestones/v0.92.1/evidence/corporate/corp-04/**`
- `docs/operations/corporate/corp-04/**`
- `.csdlc/issues/156/**`
- `.csdlc/prepared/issues/156/**`
- `.csdlc/prepared/issues/156/validate-outcome.rb`
- `.csdlc/evidence/156/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Every critical service has a company-controlled administrator, billing owner, secure MFA, recovery route, and vault location.
2. Recovery is exercised without relying solely on a founder-owned phone, email, card, or device.
3. Break-glass access is bounded, audited, and distinct from routine credentials.
4. The repository records names and outcomes only; no credential material is retained.

## PVF Lanes

- `corp-04-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/156/validate-outcome.rb`.
- `corp-04-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Service-by-service live readback, recovery exercise, role-separation check, personal-dependency negative scan, and redaction review.

## Authority Boundary

- Issue CORP-04 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Committing credentials
- Weakening MFA to simplify automation
- Treating personal billing linkage as corporate custody

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- A critical service depends on one personal recovery factor
- Company billing cannot be verified
- Credential handling would cross the repository boundary

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#corp-04`
