# Structured Task Prompt

Template: 1.0.0

Issue: 487

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #487 AWS-D only; produce the audit/security baseline and proof. Do not perform multi-account rollout, application security redesign, website/DDNS/public-edge/runtime workload changes, GCP work, or speculative cleanup.

## Deliverables

- infra/aws/account-foundation audit/security Terraform surfaces
- docs/operations/cloud/aws/audit-security operator runbook
- docs/milestones/v0.92.1/evidence/cloud/aws-d retained proof
- .csdlc/prepared/issues/487/validate-aws-d-baseline.sh
- docs/milestones/v0.92.1/evidence/cloud/aws-d/run-audit-security-readbacks.sh

## Acceptance

1. AC-1: Account changes are durably observable
2. AC-2: Security findings have an owner and destination
3. AC-3: Retention and encryption are explicit
4. AC-4: Sensitive values are excluded from retained proof
5. AC-5: Logging cost and regional scope are explicit
6. AC-6: Fresh exact-head review has no actionable findings before publication

## Dependencies

- AWS-C #486 terminal Terraform bootstrap backend and deployment-role prerequisite

## Inputs

- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#AWS-D
- infra/aws/account-foundation/README.md
- infra/aws/bootstrap/
- docs/operations/cloud/aws/terraform-bootstrap/
- docs/operations/cloud/aws/access-billing/

## Non Goals

- Multi-account rollout
- Application security redesign
- AWS resource adoption register
- Runtime platform module set
- Website, DDNS, public-edge, or workload deployment
- GCP work
- Speculative cleanup
