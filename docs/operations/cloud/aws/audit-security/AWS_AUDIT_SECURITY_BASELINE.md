# AWS audit and security baseline

Issue #487 owns the reusable AWS-D account-foundation audit/security baseline.
It is applied after the #486 Terraform backend exists and before later runtime
or provider stacks depend on durable account evidence.

## What this baseline creates

- A KMS key with rotation enabled for audit/security evidence.
- An S3 audit bucket with public access blocked, versioning enabled, KMS
  encryption, and explicit lifecycle retention.
- A CloudTrail trail for management events with log-file validation.
- Optional AWS Config recorder/delivery channel, enabled by default.
- Optional IAM Access Analyzer, enabled by default.
- An SNS topic plus EventBridge route for access-analyzer findings.
- KMS and S3 bucket policies that explicitly authorize CloudTrail, AWS Config,
  SNS, and EventBridge service use for the resources they consume.

## Required operator settings

Set these values in Terraform variables rather than editing resources:

- `environment`
- `name_prefix`
- `aws_profile`
- `finding_owner`
- `finding_destination`
- `log_retention_days`
- `cloudtrail_multi_region`
- `enable_config_recorder`
- `enable_access_analyzer`

The Terraform provider reads `aws_profile`; its validation only permits
`agent-logic-admin` for this root. The default owner is
`agent-logic-cloud-ops`; the default destination is `security-ops-sns-topic`.
Change the owner/destination for production routing before apply if the
operational destination differs.

## Apply discipline

Use the Agent Logic business AWS profile only:

```sh
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/account-foundation plan
```

Do not apply from a personal/default AWS profile. Do not commit `.tfstate`,
provider credentials, raw AWS JSON, or unredacted account identifiers.

## Proof

Static proof:

```sh
bash .csdlc/prepared/issues/487/validate-aws-d-baseline.sh .
```

Live readback proof, when authorized:

```sh
AWS_PROFILE=agent-logic-admin \
  bash docs/milestones/v0.92.1/evidence/cloud/aws-d/run-audit-security-readbacks.sh --lane=aws-readonly
```
