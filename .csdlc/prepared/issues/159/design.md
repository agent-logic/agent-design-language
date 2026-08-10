# CORP-07 Design

Issue: #159

## Objective

Move Terraform state, CI/CD, deployment identity, rollback, and operational runbook authority to company-controlled systems.

## Scope

Remote state and locks, OIDC and deployment roles, GitHub environments, secrets by name, workflow permissions, release and rollback commands, monitoring escalation, and operator runbooks.

## Dependencies

- CORP-05: issue #157
- CORP-06: issue #158

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Company-owned infrastructure-state and deployment-authority manifest.
- Proven deployment and rollback runbooks executable by an authorized company operator.

## Owned Paths

- `docs/milestones/v0.92.1/evidence/corporate/corp-07/**`
- `docs/operations/corporate/corp-07/**`
- `.csdlc/issues/159/**`
- `.csdlc/prepared/issues/159/**`
- `.csdlc/evidence/159/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Terraform state, locks, plans, applies, and recovery operate under company custody.
2. CI uses company-controlled OIDC or equivalent short-lived identity and least privilege.
3. A clean deployment and rollback complete without founder-local credentials or unrecorded manual steps.
4. Runbooks name prerequisites, single commands, expected outputs, rollback, cleanup, and escalation without exposing secrets.

## PVF Lanes

- `corp-07-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/159/validate-outcome.rb`.
- `corp-07-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

State readback, lock contention, OIDC identity, plan/apply, deployment, rollback, clean-operator rehearsal, and secret-redaction checks.

## Authority Boundary

- Issue CORP-07 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Embedding credentials in workflows or runbooks
- Replacing proven infrastructure during authority migration
- Treating an unexecuted plan as deployment proof

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- State custody is ambiguous
- Deployment requires personal credentials
- Rollback is not executable
- Workflow permissions exceed the reviewed role

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#corp-07`
