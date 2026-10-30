# AWS-D audit and security baseline

Issue #487 owns the AWS audit and security baseline for the approved Agent Logic business account after #486 provides the Terraform bootstrap prerequisite.

The baseline must define CloudTrail/account activity visibility, configuration posture, detection finding ownership, access-analysis readback, alert routing, retention, encryption, cost scope, and redacted evidence handling.

Every enabled finding producer must declare:

- `finding_owner`: a stable team or role identifier accountable for triage.
- `finding_destination`: a readback-verifiable destination such as SNS, EventBridge, ticket queue, or security mailbox.

Retained readback output must be redacted by default: counts and booleans are acceptable; raw account IDs, ARNs, email addresses, analyzer names, trail names, and credential material are not.

No AWS mutation is authorized by this preparation file. Live readbacks and Terraform apply must use the approved `agent-logic-admin` business profile and retain no credential material. The issue-owned readback script rejects any other `AWS_PROFILE`, including static mode, so accidental personal-account targeting fails before AWS calls.
