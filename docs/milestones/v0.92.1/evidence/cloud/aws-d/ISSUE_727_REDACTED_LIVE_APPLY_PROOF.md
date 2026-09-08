# Issue #727 redacted live apply proof

Issue #727 applied the reviewed `infra/aws/account-foundation` Terraform root
to the approved Agent Logic business AWS account in `us-west-2` using the
`agent-logic-admin` profile.

## Authorization and saved plan

- Terraform root: `infra/aws/account-foundation`
- Terraform workspace: `aws-d-account-foundation-live`
- Backend state key:
  `v0.92.1/aws-d/account-foundation/account-foundation.tfstate`
- Saved plan SHA-256:
  `c35b12d920386af64a81ddc17316d10be679b1d717d7ecc93ef7b1da731c264f`
- Operator authorization source: current chat authorization to complete #727
  inside the bounded issue window.
- Raw authorization, plan, apply, and provider logs are retained only under the
  ignored issue-local request directory `.adl/requests/727/`.

## Apply result

Terraform applied the exact saved plan successfully:

- 19 resources added
- 0 resources changed
- 0 resources destroyed

The planned and applied resource set was limited to the #727 audit/security
foundation surface:

- CloudTrail account activity logging
- KMS-backed audit log bucket controls
- S3 versioning, encryption, public-access block, lifecycle, and bucket policy
- AWS Config recorder, delivery channel, recorder status, and Config IAM role
- IAM Access Analyzer
- SNS security findings topic and policy
- EventBridge Access Analyzer findings rule and target

## Redacted AWS readback

The live readback script returned:

```text
aws_d_readback_lane=aws-readonly
required_profile=agent-logic-admin
cloudtrail_exact=present
cloudtrail_kms=present
config_recorder_exact=present
config_delivery_bucket_matches=true
access_analyzer_exact=active
sns_findings_topic_exact=present
eventbridge_findings_route=present
audit_bucket_kms_encryption=present
audit_bucket_retention_days_at_least=365
finding_owner_destination_tags=present
redaction=names_and_arns_not_printed
```

No raw account IDs, ARNs, emails, access keys, secret values, or unfiltered AWS
JSON are committed in this proof packet.
