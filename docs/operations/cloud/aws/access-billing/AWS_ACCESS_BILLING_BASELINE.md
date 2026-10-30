# AWS access and billing baseline

Issue: #485 (`AWS-B`)

Status: draft implementation evidence, pending typed bind, exact-head review, and publication. Live readbacks completed, and AC-3 now has an AWS CLI `2.36.32` readback, above the required AWS CLI 2.35+ floor.

## Scope and authority

This baseline covers the approved Agent Logic business AWS account only. It depends on #484 for the accepted resource ownership inventory and does not re-own inventory classification. It prepares the access, audit-attribution, and cost-visibility controls needed before #486 creates any separate Terraform account-foundation bootstrap.

Owned paths:

- `infra/aws/account-foundation/**`
- `docs/operations/cloud/aws/access-billing/**`
- `docs/milestones/v0.92.1/evidence/cloud/aws-b/**`

Out of scope:

- AWS Organizations or Control Tower rollout.
- Workload deployment.
- Terraform apply, resource import, resource cleanup, or state migration.
- Unrestricted agent mutation.
- Removing or weakening the existing proven administrator path.
- Public WSS, Route53, ACM, or CloudFront exposure owned by #122.

## Acceptance mapping

| Acceptance | Baseline control | Evidence path |
| --- | --- | --- |
| AC-1 corporate recovery does not depend on one personal factor | Existing administrator access is retained; replacement/removal is explicitly blocked until independently proven. Account alias and IAM account-summary readbacks are retained when available. | `docs/milestones/v0.92.1/evidence/cloud/aws-b/readbacks/root-recovery.md` |
| AC-2 identities are distinguishable | Human, workload, and agent candidates are separated by IAM user/role readbacks and naming/path heuristics; unknowns remain explicit gaps. | `docs/milestones/v0.92.1/evidence/cloud/aws-b/readbacks/identity-census.md` |
| AC-3 Agent Toolkit path uses AWS CLI 2.35+ | Local AWS CLI version is checked; Agent Toolkit setup is recorded as documentation/configuration only until an operator-approved AWS mutation lane exists. | `docs/milestones/v0.92.1/evidence/cloud/aws-b/readbacks/agent-toolkit-configuration.md` |
| AC-4 agent IAM guardrails bind read-only default posture | Agent posture is read-only by default; write elevation requires later typed approval and context policy evidence. Existing policies are read back when permissions allow. | `docs/milestones/v0.92.1/evidence/cloud/aws-b/readbacks/agent-iam-guardrails.md` |
| AC-5 CloudWatch and CloudTrail requests are attributable | CloudTrail trail status/event lookup and CloudWatch metric listing are read back when permissions allow. | `docs/milestones/v0.92.1/evidence/cloud/aws-b/readbacks/agent-activity-audit.md` |
| AC-6 billing and budget ownership is visible | Cost Explorer, budgets, anomaly monitor, cost category, tag, and export readbacks are retained when permissions allow. | `docs/milestones/v0.92.1/evidence/cloud/aws-b/readbacks/billing-readback.md` |
| AC-7 existing administrator access remains | This issue performs no delete, detach, deactivate, remove, or replacement operation against administrator access. | `docs/milestones/v0.92.1/evidence/cloud/aws-b/readbacks/root-recovery.md` |
| AC-8 retained evidence is redacted and non-mutating | The validator rejects credential-like material and mutation verbs in retained evidence and scripts. | `.csdlc/prepared/issues/485/validate-aws-b-baseline.sh` |

## Agent Toolkit boundary

Agent Toolkit for AWS is allowed here only as an approved Codex path and documentation/configuration baseline. The required local CLI floor is AWS CLI 2.35 or newer. Toolkit use does not authorize infrastructure creation, IAM writes, billing mutations, Terraform apply, or unrestricted agent AWS access. Any later write path must be a separate typed lane with an exact operation, operator approval boundary, evidence target, and rollback/recovery story.

Default agent posture:

1. Read-only AWS API access only.
2. Explicit `agent-logic-admin` business profile, never default/personal profiles.
3. IAM context policy constraints for any future elevated operation.
4. CloudTrail/CloudWatch attribution evidence retained before using the lane as production proof.
5. Credentials, token files, raw environment dumps, payment data, and secret material excluded from retained evidence.

## Current gaps policy

If a readback is denied or unavailable, the evidence records the gap as `READBACK_UNAVAILABLE` with the command shape and sanitized error. A gap does not become proof. Publication remains blocked until the accepted baseline either proves the lane or explicitly records a reviewed residual risk/non-claim.

## Current readiness note

AC-3 is no longer blocked on the local Agent Toolkit prerequisite: the retained readback shows AWS CLI `2.36.32`. This baseline is still not complete until typed bind, validation, exact-head review, publication, and hosted shepherding gates are satisfied.
