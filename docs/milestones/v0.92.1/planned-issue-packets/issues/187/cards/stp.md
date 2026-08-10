# Structured Task Prompt

Template: 1.0.0

Issue: 187

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only DRT-07 within its exact owned paths and authority boundary.

## Deliverables

- Producer-derived soak and resource receipt bundles for both live windows.
- Independent replay and cleanup verification plus final qualification report with explicit non-claims and residual risks.

## Acceptance

1. Both soak durations complete under declared workload, fault, resource, and error thresholds.
2. Receipts bind exact commands, terms, committed indexes, envelopes, source revisions, model digests, clocks, and cleanup outcomes.
3. Independent replay reproduces the declared deterministic outcomes without live-provider dependence.
4. Provider and process readback proves cleanup after normal completion and every injected or unexpected failure phase.

## Dependencies

- DRT-05: issue #185
- DRT-06: issue #186

## Inputs

- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#drt-07
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Extending soak duration after seeing results
- Replacing failed proof with screenshots
- Claiming release approval
