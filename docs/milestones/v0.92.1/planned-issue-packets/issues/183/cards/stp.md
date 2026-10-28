# Structured Task Prompt

Template: 1.0.0

Issue: 183

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only DRT-03 within its exact owned paths and authority boundary.

## Deliverables

- Exact-revision production launch and scenario runner.
- Node, agent, authority, quorum, commit, lease, snapshot, Observatory, resource, replay, and cleanup receipts for every phase.

## Acceptance

1. The exact #142 merge SHA is ancestral to the tested revision and its retained Guardian/API/WSS/WP-04.16 proof passes.
2. Three independently started voters commit governed work; two voters preserve quorum; one voter cannot mutate.
3. The old Observatory lease expires before successor binding and stale-owner writes are denied.
4. Snapshot restore, voter restart, agent continuity, replay, and cleanup pass without shared state roots or direct executor bypass.

## Dependencies

- DRT-01: issue #181
- DRT-02: issue #182
- RUNTIME-142: issue #142 terminal exact proof

## Inputs

- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#drt-03
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Testing an open or merely green #142 PR
- Using in-process service objects as voters
- Leaving a failed phase running
