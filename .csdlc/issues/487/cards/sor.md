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

[
  {
    "command": [
      "terraform -chdir=infra/aws/account-foundation fmt -check",
      "terraform -chdir=infra/aws/account-foundation validate",
      "bash .csdlc/prepared/issues/487/validate-aws-d-baseline.sh .",
      "AWS_PROFILE=agent-logic-admin bash docs/milestones/v0.92.1/evidence/cloud/aws-d/run-audit-security-readbacks.sh --lane=static",
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate issue --issue 487",
      "git diff --check"
    ],
    "purpose": "Prove the #487 account-foundation Terraform root, issue-owned validator, profile-gated static readback, lifecycle card shape, and diff hygiene without live AWS mutation.",
    "outcome": "passed",
    "evidence_ref": "Local output: Terraform validate success; aws-d static contract validation passed; static readback cloud_calls=disabled; csdlc-validate issue pass; git diff --check pass."
  },
  {
    "command": [
      "terraform -chdir=infra/aws/account-foundation fmt",
      "terraform -chdir=infra/aws/account-foundation validate",
      "bash .csdlc/prepared/issues/487/validate-aws-d-baseline.sh .",
      "AWS_PROFILE=agent-logic-admin bash docs/milestones/v0.92.1/evidence/cloud/aws-d/run-audit-security-readbacks.sh --lane=static",
      "git diff --check"
    ],
    "purpose": "Prove #487 R4 remediation for KMS service consumers, AWS Config audit-bucket delivery, Terraform profile enforcement, static readback profile gating, and diff hygiene without live AWS mutation.",
    "outcome": "passed",
    "evidence_ref": "Local output: Terraform validate success; aws-d static contract validation passed; static readback cloud_calls=disabled; git diff --check pass."
  },
  {
    "command": [
      "terraform -chdir=infra/aws/account-foundation fmt",
      "terraform -chdir=infra/aws/account-foundation init -backend=false",
      "terraform -chdir=infra/aws/account-foundation validate",
      "bash .csdlc/prepared/issues/487/validate-aws-d-baseline.sh .",
      "AWS_PROFILE=agent-logic-admin bash docs/milestones/v0.92.1/evidence/cloud/aws-d/run-audit-security-readbacks.sh --lane=static",
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate issue --issue 487",
      "git diff --check"
    ],
    "purpose": "Prove #487 R4/R5 remediation, Terraform formatting/validation, exact redacted readback contract, profile-gated static readback, lifecycle validation, and diff hygiene without live AWS mutation.",
    "outcome": "passed",
    "evidence_ref": "Local output: terraform fmt changed main.tf; init -backend=false installed pinned aws v5.100.0; Terraform validate success; aws-d static contract validation passed; static readback cloud_calls=disabled; csdlc-validate issue pass; git diff --check pass."
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
