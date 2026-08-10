# Structured Task Prompt

Template: 1.0.0

Issue: 181

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only DRT-01 within its exact owned paths and authority boundary.

## Deliverables

- Versioned topology and scenario manifest for both live windows.
- Producer-derived receipt, resource, timing, cleanup, and claim schema with negative-case denominator.

## Acceptance

1. The contract names exactly three voters, three governed agents, one non-voting Shepherd, and one quorum-leased Observatory.
2. Every node has distinct identity, credential, port, state root, storage, and failure-domain placement.
3. Each scenario has setup, action, expected commit/election/fence behavior, timeout, receipt fields, cleanup, and fail-closed outcome.
4. The contract distinguishes production proof from harness orchestration and forbids in-process substitutes or hard-coded success counts.

## Dependencies

- No child dependency; setup issue #146 and umbrella readiness only

## Inputs

- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#drt-01
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Running live nodes
- Changing Runtime behavior
- Treating a topology diagram as proof
