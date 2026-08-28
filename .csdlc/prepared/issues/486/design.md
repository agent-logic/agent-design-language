# #486 AWS-C Terraform bootstrap design

Issue: #486

## Goal

Establish a recoverable AWS Terraform account-foundation bootstrap for the Agent
Logic business AWS account, isolated from existing website, DDNS, public-edge,
and workload Terraform states.

## Scope

- Add `infra/aws/bootstrap/**` Terraform for the account-foundation backend and
  deployment role boundary.
- Add `docs/operations/cloud/aws/terraform-bootstrap/**` operator runbook and
  decision notes.
- Add `docs/milestones/v0.92.1/evidence/cloud/aws-c/**` retained proof for
  backend identity, locking/versioning, recovery rehearsal, provider pinning,
  and state isolation.

## Design

The bootstrap lane creates a small, explicit Terraform foundation:

1. A dedicated encrypted S3 state bucket with versioning enabled.
2. A dedicated DynamoDB lock table for Terraform state locking.
3. A least-privilege deployment role policy boundary for future account
   foundation work.
4. A provider/version pin set so later plans are reproducible.
5. A read-only inventory and state-isolation register proving existing website,
   DDNS, public-edge, and workload states are not copied into or dual-owned by
   this bootstrap.

The first implementation may be split into reusable Terraform plus runbook and
validation proof. Live AWS mutation is allowed only through the approved
`agent-logic-admin` profile and only for the explicit bootstrap resources in
this issue. If a reviewed plan differs before apply, stop.

## Validation

- `terraform fmt -check -recursive infra/aws/bootstrap`
- `terraform init -backend=false` and `terraform validate` for the bootstrap
  module/root.
- Read-only AWS readbacks with `AWS_PROFILE=agent-logic-admin` proving:
  - backend bucket encryption and versioning,
  - DynamoDB lock table identity,
  - recovery rehearsal evidence,
  - provider pins,
  - no existing website/DDNS/public-edge/workload state ownership collision.
- Issue-owned validator under `.csdlc/prepared/issues/486/`.

## Non-goals

- Website state migration.
- Runtime deployment.
- CloudFormation retirement.
- Importing or copying existing Terraform state.
- Broad AWS account hardening beyond the bootstrap boundary.

## Stop conditions

- Existing backend owner is unknown.
- A reviewed plan differs at apply.
- Recovery rehearsal fails.
- State ownership is ambiguous or dual-owned.
- Credentials would enter retained evidence.
