# #485 design — AWS access and billing baseline

## Purpose

Produce one accepted access and billing control baseline for the approved Agent Logic AWS business account, without removing the existing proven administrator path or capturing credential material.

## Scope

Issue #485 owns:

- `infra/aws/account-foundation/**`
- `docs/operations/cloud/aws/access-billing/**`
- `docs/milestones/v0.92.1/evidence/cloud/aws-b/**`
- `.csdlc/prepared/issues/485/**`
- `.csdlc/evidence/485/**`

The baseline records recoverable corporate administration, distinguishes human, workload, and agent-initiated access, constrains agent posture to read-only by default, and captures cost visibility and attribution readbacks.

## Approach

1. Reuse #484 as the accepted AWS account and inventory predecessor; do not reclassify resources in this issue.
2. Verify the configured AWS CLI version and approved business profile identity without printing credentials.
3. Capture read-only IAM, CloudTrail, CloudWatch, billing, budget, anomaly, export, and tagging readbacks.
4. Document the Agent Toolkit for AWS approved Codex path, minimum AWS CLI version, read-only default posture, and operator-approval boundary for any future AWS mutation.
5. Add local validation that rejects credentials, rejects unintended AWS mutation verbs in retained proof, and checks that all AWS-B acceptance sections are present.

## Non-goals

- No Organizations or Control Tower rollout.
- No workload deployment.
- No unrestricted agent mutation.
- No removal of break-glass or existing administrator access.
- No Terraform apply or AWS resource cleanup.

## Review focus

Review should confirm that the baseline is evidence-backed, redacted, issue-scoped, does not overclaim AWS mutation or production readiness, keeps #122 public exposure scope separate, and leaves administrator replacement/removal blocked until proven by later operator-approved work.
