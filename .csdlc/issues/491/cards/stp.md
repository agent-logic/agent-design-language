# Structured Task Prompt

Template: 1.0.0

Issue: 491

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #491 GCP-B only; produce the Terraform backend and deployment-identity bootstrap and proof. Do not deploy Runtime, create GPU compute, alter AWS, or widen into production hierarchy rollout.

## Deliverables

- infra/gcp/bootstrap Terraform root/module for backend and deployment identity bootstrap
- docs/operations/cloud/gcp/terraform-bootstrap operator runbook
- docs/milestones/v0.92.1/evidence/cloud/gcp-b retained proof packet
- issue-owned validator and readback scripts for backend identity, state recovery, approved key-backed execution, provider pins, and local-state cleanup

## Acceptance

1. AC-1: State is versioned private recoverable and auditable
2. AC-2: Deployment uses the approved service-account key-backed bootstrap path by default for this sprint
3. AC-3: Provider and module versions are pinned
4. AC-4: Local bootstrap state is removed or ignored recoverably
5. AC-5: Operator-approved key-backed bootstrap path is documented and verified without exposing key contents
6. AC-6: Fresh exact-head review has no actionable findings before publication

## Dependencies

- GCP-A #490 terminal project/account decision and identity plan

## Inputs

- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#GCP-B
- docs/milestones/v0.92.1/evidence/cloud/gcp-a/gcp-execution-identity-plan.md
- docs/operations/cloud/gcp/
- infra/gcp/

## Non Goals

- Runtime deployment
- Production hierarchy rollout beyond the host project
- AWS changes
- GPU launch
- Long-lived secret management beyond the one operator-approved local key path
- Org-wide policy relaxation as steady state
