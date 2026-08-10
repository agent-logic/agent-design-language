# Structured Task Prompt

Template: 1.0.0

Issue: 184

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only DRT-04 within its exact owned paths and authority boundary.

## Deliverables

- Single-command hybrid qualification runner using the Agent Logic business AWS profile.
- Placement, transport, snapshot, quorum, election, commit, fence, partition, heal, halt, resource, cost, and cleanup receipts.

## Acceptance

1. AWS identity resolves to the approved Agent Logic business account before provisioning.
2. AWS voters use separate AZs, private authenticated transport, distinct state and independently materialized snapshots.
3. Isolating Wuji preserves AWS-only quorum continuity while the isolated stale voter cannot mutate; loss of quorum halts mutation.
4. Healing converges term, commit index, state digest, fence, and Observatory ownership before traffic resumes; every phase cleans up.

## Dependencies

- DRT-03: issue #183

## Inputs

- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#drt-04
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Public control endpoints
- Shared state or manually copied snapshots
- Dynamic IAM profile creation
- Leaving cloud resources after failure
