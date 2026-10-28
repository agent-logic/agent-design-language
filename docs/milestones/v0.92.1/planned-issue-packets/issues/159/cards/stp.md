# Structured Task Prompt

Template: 1.0.0

Issue: 159

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only CORP-07 within its exact owned paths and authority boundary.

## Deliverables

- Company-owned infrastructure-state and deployment-authority manifest.
- Proven deployment and rollback runbooks executable by an authorized company operator.

## Acceptance

1. Terraform state, locks, plans, applies, and recovery operate under company custody.
2. CI uses company-controlled OIDC or equivalent short-lived identity and least privilege.
3. A clean deployment and rollback complete without founder-local credentials or unrecorded manual steps.
4. Runbooks name prerequisites, single commands, expected outputs, rollback, cleanup, and escalation without exposing secrets.

## Dependencies

- CORP-05: issue #157
- CORP-06: issue #158

## Inputs

- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#corp-07
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Embedding credentials in workflows or runbooks
- Replacing proven infrastructure during authority migration
- Treating an unexecuted plan as deployment proof
