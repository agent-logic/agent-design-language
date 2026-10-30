# AWS account foundation

This directory contains the #487 AWS-D audit/security baseline for the Agent
Logic business AWS account. It is intentionally small and account-scoped:

- CloudTrail account activity logging with log-file validation.
- KMS-backed S3 retention for audit/security evidence.
- AWS Config recorder and delivery channel.
- IAM Access Analyzer for account trust-edge visibility.
- SNS/EventBridge routing for enabled security findings.
- Explicit `finding_owner` and `finding_destination` tags/outputs.
- Terraform provider execution pinned to the approved `agent-logic-admin`
  business profile.

The root is designed to consume the Terraform backend established by #486. This
issue does not create public-edge, runtime workload, GCP, GPU, Unity, or
multi-account organization infrastructure.

## Local validation

```sh
terraform -chdir=infra/aws/account-foundation fmt -check
terraform -chdir=infra/aws/account-foundation init -backend=false
terraform -chdir=infra/aws/account-foundation validate
bash .csdlc/prepared/issues/487/validate-aws-d-baseline.sh .
```

The Terraform provider is configured with `profile = var.aws_profile`, and the
variable validation permits only `agent-logic-admin` for this root. Override the
profile only by changing reviewed Terraform input truth, not by relying on an
ambient personal/default credential chain.

## Remote state

This root declares an S3 backend and must consume the AWS-C bootstrap backend
before any live plan or apply is treated as authoritative. Use a local, untracked
copy of `infra/aws/account-foundation/backend.hcl.example`, replace only the
redacted account placeholder in the bucket name from the AWS-C bootstrap
readback, and keep the state key separate from the bootstrap and Runtime roots:

```sh
terraform -chdir=infra/aws/account-foundation init \
  -backend-config=backend.hcl
```

Do not commit the raw AWS account id, an unredacted backend file, or Terraform
state.

## Live readback

Live readback must use the approved business profile:

```sh
AWS_PROFILE=agent-logic-admin \
  bash docs/milestones/v0.92.1/evidence/cloud/aws-d/run-audit-security-readbacks.sh --lane=aws-readonly
```

The script emits a redacted summary by default. Do not commit raw AWS JSON,
account IDs, ARNs, emails, access keys, secret values, or unfiltered CLI output.
