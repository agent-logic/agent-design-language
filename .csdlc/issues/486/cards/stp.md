# Structured Task Prompt

Template: 1.0.0

Issue: 486

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #486 AWS-C only; implement Terraform bootstrap, evidence, and runbook after bind. Do not perform website state migration, Runtime deployment, CloudFormation retirement, or unrelated AWS hardening.

## Deliverables

- infra/aws/bootstrap Terraform bootstrap root/module
- docs/operations/cloud/aws/terraform-bootstrap operator runbook
- docs/milestones/v0.92.1/evidence/cloud/aws-c retained proof
- .csdlc/prepared/issues/486/validate-aws-c-bootstrap.sh
- docs/milestones/v0.92.1/evidence/cloud/aws-c/run-terraform-bootstrap-readbacks.sh

## Acceptance

1. AC-1: Existing website and DDNS states are inventoried first
2. AC-2: The new backend is encrypted, versioned, locked, and recoverable
3. AC-3: Deployment identity is least privilege for account-foundation bootstrap work
4. AC-4: No existing state is copied or dual-owned
5. AC-5: Provider pins and saved-plan review are explicit
6. AC-6: Fresh exact-head review has no actionable findings before publication

## Dependencies

- AWS-B #485 terminal access and billing baseline

## Inputs

- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#AWS-C
- docs/operations/cloud/aws/access-billing/AWS_ACCESS_BILLING_BASELINE.md
- docs/milestones/v0.92.1/evidence/cloud/aws-b/
- infra/aws/
- docs/operations/cloud/aws/

## Non Goals

- Website state migration
- Runtime deployment
- CloudFormation retirement
- Importing existing state
- Paid workload launch
