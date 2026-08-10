# CORP-06 Design

Issue: #158

## Objective

Migrate production AWS infrastructure into the Agent Logic business account with DNS, TLS, email, storage, monitoring, workload, and rollback proof.

## Scope

Agent Logic business AWS identity, Route53, ACM, SES, S3, CloudFront, compute, IAM, monitoring, backups, budgets, account contacts, and temporary migration resources.

## Dependencies

- CORP-04: issue #156

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Service-by-service infrastructure migration manifest and company-account readback.
- Cutover and rollback receipts for DNS, certificates, email, storage, delivery, monitoring, and workloads.

## Owned Paths

- `docs/milestones/v0.92.1/evidence/corporate/corp-06/**`
- `docs/operations/corporate/corp-06/**`
- `.csdlc/issues/158/**`
- `.csdlc/prepared/issues/158/**`
- `.csdlc/prepared/issues/158/validate-outcome.rb`
- `.csdlc/evidence/158/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Every AWS operation verifies the approved Agent Logic business account and uses the permanent business profile.
2. Public TLS uses ACM or another publicly trusted issuer; production paths contain no self-signed certificate.
3. DNS, email, storage, CDN, workload, monitoring, backup, budget, and rollback checks pass from company authority.
4. Temporary resources are inventoried, tagged, bounded, and deleted with provider readback after each phase.

## PVF Lanes

- `corp-06-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/158/validate-outcome.rb`.
- `corp-06-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Account identity, Terraform plan/apply, DNS, TLS, SES, S3/CloudFront, workload, monitoring, backup, budget, rollback, and cleanup receipts.

## Authority Boundary

- Issue CORP-06 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Using the founder personal AWS account
- Introducing a second permanent IAM profile
- Leaving temporary resources running after failure

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- AWS resolves to a non-business account
- A certificate is self-signed
- Rollback cannot be rehearsed
- Temporary resources cannot be enumerated or removed

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#corp-06`
