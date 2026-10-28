# Structured Task Prompt

Template: 1.0.0

Issue: 174

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only V3-11B within its exact owned paths and authority boundary.

## Deliverables

- Scheduler, process registry, cancellation wiring, evidence model, result renderer, interruption fixtures, and representative local journeys.

## Acceptance

1. Parallel tasks are bounded and every Tokio task is awaited after cancellation.
2. Every OS child is registered with root cancellation; Unix termination uses bounded `SIGTERM`/kill escalation and Windows uses the reviewed termination primitive, followed by handle wait and output drain.
3. Every sleep and network/process await participates in `tokio::select!` with cancellation.
4. Incomplete, cancelled, timed-out, or tampered evidence cannot appear passed.
5. Each captured stream records `truncated`, `captured_bytes`, and `original_bytes_if_known`; human and JSON output distinguish an enforced cap from naturally short process output.
6. Passing validation cannot authorize review, publication, or merge.

## Dependencies

- V3-08: issue #169
- V3-09: issue #170
- V3-11A: issue #173

## Inputs

- docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-11b
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Planning-policy invention, embedded product test logic, hidden CI routing, implicit cloud runners, background queues, review, or publication.
