# Issue #727 redacted live apply proof

Issue #727 applied the reviewed `infra/aws/account-foundation` Terraform root
to the approved Agent Logic business AWS account in `us-west-2` using the
`agent-logic-admin` profile.

## Authorization and saved plan

- Durable proof generated at UTC: `2026-09-08T10:06:13Z`
- PR source revision at remediation start:
  `e2f24d2b8f884eea40043f2c63451402e61c5855`
- Account identity evidence: the apply/readback used the
  `agent-logic-admin` profile, matched the operator-approved Agent Logic
  business AWS account, and retained only redacted account/caller evidence in
  the ignored issue-local request directory.
- Region selector: `us-west-2`
- Terraform root: `infra/aws/account-foundation`
- Terraform workspace: `aws-d-account-foundation-live`
- Backend state key:
  `v0.92.1/aws-d/account-foundation/account-foundation.tfstate`
- Saved plan SHA-256:
  `c35b12d920386af64a81ddc17316d10be679b1d717d7ecc93ef7b1da731c264f`
- Saved plan binding: the authorization validator compares the envelope digest
  with `.adl/requests/727/account-foundation.tfplan.sha256`, validates the
  saved plan JSON under `.adl/requests/727/account-foundation.plan.json`, and
  fails closed unless the plan contains exactly the 19 create-only resources
  listed below.
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

- `aws_accessanalyzer_analyzer.account[0]`
- `aws_cloudtrail.account_activity`
- `aws_cloudwatch_event_rule.access_analyzer_findings`
- `aws_cloudwatch_event_target.access_analyzer_findings`
- `aws_config_configuration_recorder.account[0]`
- `aws_config_configuration_recorder_status.account[0]`
- `aws_config_delivery_channel.account[0]`
- `aws_iam_role.config[0]`
- `aws_iam_role_policy_attachment.config[0]`
- `aws_kms_alias.audit`
- `aws_kms_key.audit`
- `aws_s3_bucket.audit_logs`
- `aws_s3_bucket_lifecycle_configuration.audit_logs`
- `aws_s3_bucket_policy.audit_logs`
- `aws_s3_bucket_public_access_block.audit_logs`
- `aws_s3_bucket_server_side_encryption_configuration.audit_logs`
- `aws_s3_bucket_versioning.audit_logs`
- `aws_sns_topic.security_findings`
- `aws_sns_topic_policy.security_findings`

## Redacted AWS readback

The live readback script returned the following redacted durable outcome. This
readback was refreshed at `2026-09-08T10:06:13Z` and proves active controls,
not merely resource presence:

```text
aws_d_readback_lane=aws-readonly
required_profile=agent-logic-admin
cloudtrail_exact=present
cloudtrail_multi_region=true
cloudtrail_logging=true
cloudtrail_kms=present
config_recorder_exact=present
config_recorder_recording=true
config_delivery_bucket_matches=true
config_delivery_status=success
access_analyzer_exact=active
sns_findings_topic_exact=present
eventbridge_findings_route=present
audit_bucket_kms_encryption=present
audit_bucket_versioning=enabled
audit_bucket_retention_days_at_least=365
finding_owner_destination_tags=present
redaction=names_and_arns_not_printed
```

No raw account IDs, ARNs, emails, access keys, secret values, or unfiltered AWS
JSON are committed in this proof packet.
