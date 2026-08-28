# Structured Output Record

Template: 1.0.0

Issue: 487

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the #487 AWS-D audit/security account-foundation baseline as local Terraform, runbook, and readback proof surfaces. Local validation is complete; live AWS apply/readback remains explicitly gated on operator-approved cloud mutation/readback authority with AWS_PROFILE=agent-logic-admin.

## Artifacts

- infra/aws/account-foundation/main.tf
- infra/aws/account-foundation/variables.tf
- infra/aws/account-foundation/outputs.tf
- infra/aws/account-foundation/versions.tf
- infra/aws/account-foundation/.terraform.lock.hcl
- infra/aws/account-foundation/terraform.tfvars.example
- infra/aws/account-foundation/README.md
- docs/operations/cloud/aws/audit-security/README.md
- docs/milestones/v0.92.1/evidence/cloud/aws-d/run-audit-security-readbacks.sh
- .csdlc/prepared/issues/487/validate-aws-d-baseline.sh
- .csdlc/issues/487

## Execution

- Added the account-foundation Terraform root for CloudTrail account activity, encrypted audit S3 retention, AWS Config recorder/delivery, IAM Access Analyzer, SNS findings topic, and EventBridge routing.
- Pinned the account-foundation AWS provider to the repo-proven 5.100 provider line instead of floating to an unvalidated 6.x plugin.
- Restored the reviewed #487 issue-owned preparation validator into the bound worktree.
- Added the AWS-D audit/security operations runbook and redacted readback script.
- Verified the readback script fails closed unless AWS_PROFILE=agent-logic-admin and performs no AWS calls in static mode.

## Validation

[]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
