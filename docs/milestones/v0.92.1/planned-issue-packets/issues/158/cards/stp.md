# Structured Task Prompt

Template: 1.0.0

Issue: 158

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only CORP-06 within its exact owned paths and authority boundary.

## Deliverables

- Service-by-service infrastructure migration manifest and company-account readback.
- Cutover and rollback receipts for DNS, certificates, email, storage, delivery, monitoring, and workloads.

## Acceptance

1. Every AWS operation verifies the approved Agent Logic business account and uses the permanent business profile.
2. Public TLS uses ACM or another publicly trusted issuer; production paths contain no self-signed certificate.
3. DNS, email, storage, CDN, workload, monitoring, backup, budget, and rollback checks pass from company authority.
4. Temporary resources are inventoried, tagged, bounded, and deleted with provider readback after each phase.

## Dependencies

- CORP-04: issue #156

## Inputs

- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#corp-06
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Using the founder personal AWS account
- Introducing a second permanent IAM profile
- Leaving temporary resources running after failure
