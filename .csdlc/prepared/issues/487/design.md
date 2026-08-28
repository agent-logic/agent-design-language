# Issue #487 Design — AWS-D audit and security baseline

## Intent

Issue #487 produces the AWS audit and security baseline for the approved Agent Logic business account after #486 supplies the Terraform bootstrap backend. The baseline must make account changes durably observable, give security findings an owner and destination, make retention and encryption explicit, and preserve redacted proof without leaking secrets.

## Scope

Owned surfaces:

- `infra/aws/account-foundation/**`
- `docs/operations/cloud/aws/audit-security/**`
- `docs/milestones/v0.92.1/evidence/cloud/aws-d/**`
- `.csdlc/prepared/issues/487/**`

The implementation may add Terraform account-foundation modules or roots for AWS-native audit/security controls, plus operator runbooks and readback scripts that prove the exact controls. The issue must consume #486 only as the backend/deployment prerequisite; it must not re-own AWS-C bootstrap resources.

## Baseline controls

The reviewed baseline should cover, at minimum:

1. CloudTrail/account activity visibility, including durable retention and encryption posture.
2. AWS Config or equivalent configuration-recorder posture where cost and regional scope are explicit.
3. Security findings destinations and owner routing for enabled detection services.
4. IAM Access Analyzer or equivalent access-analysis readback for account trust edges.
5. CloudWatch/EventBridge/SNS or successor alert routing for actionable findings.
6. Redaction rules for retained evidence so account IDs, ARNs, emails, and resource names are retained only when necessary and credentials are never retained.

## Findings owner and destination contract

Every enabled finding producer must declare an explicit `finding_owner` and `finding_destination` in the #487-owned Terraform/runbook proof. Acceptable owners are stable team or role identifiers, not personal ad-hoc notes. Acceptable destinations are retained channels or resources that can be read back, such as an SNS topic, EventBridge target, ticket queue, or documented security mailbox. The validator must fail post-bind implementation proof when a producer lacks either field, when the destination cannot be read back, or when retained evidence would expose credentials or unnecessary account identifiers.

## Dependency gates

- #486 must be terminal and ancestral before #487 binds or implements account-foundation changes.
- Any AWS readback or apply must use the approved `agent-logic-admin` business profile.
- Any cloud mutation requires reviewed Terraform plan evidence and explicit operator authorization for the mutation step.

## Non-goals

- Multi-account or organization-wide rollout.
- Application security redesign.
- Website, DDNS, public-edge, runtime workload, GPU, GCP, or Unity work.
- Removing existing break-glass/admin access.
- Speculative cleanup or deletion of existing resources.

## Validation model

Local validation must prove design and static contract truth before cloud use:

- issue-owned validator checks required paths, dependency text, redaction rules, no credential-bearing evidence patterns, and Terraform/static runbook shape;
- after binding, the same validator must additionally require #487-owned implementation files and fail if real Terraform `resource` blocks for CloudTrail, AWS Config, Access Analyzer, KMS encryption, SNS alert routing, retention/lifecycle, `finding_owner`, or `finding_destination` declarations are absent;
- Terraform formatting/validation applies only to #487-owned account-foundation paths;
- live readback lanes are separately recorded and may remain gated until cloud authorization.

Cloud validation, when authorized, must read back the enabled audit/security controls and record bounded, redacted evidence under `docs/milestones/v0.92.1/evidence/cloud/aws-d/`. Live readback scripts must emit redacted status/count summaries by default, not raw account IDs, ARNs, email addresses, analyzer names, trail names, or unfiltered AWS JSON.
The readback script must fail closed if `AWS_PROFILE` is set to anything other than the approved `agent-logic-admin` business profile, including static mode.

## Failure policy

Fail closed if audit gaps remain unexplained, findings lack an owner/destination, retention or encryption is implicit, logging cost is uncontrolled, retained proof would expose secrets, or the exact #486 dependency is not terminal and current.
