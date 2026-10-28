# Structured Task Prompt

Template: 1.0.0

Issue: 177

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only V3-14 within its exact owned paths and authority boundary.

## Deliverables

- Typed mutation operations, durable intent integration, publication command with explicit `closing | part_of` linkage selection, mode-bound publication evidence and reconciliation, foreground watch with 30-minute default, 24-hour maximum, 15-second default poll interval and stderr progress, idempotency/readback fixtures, and bounded live publication canary.

## Acceptance

1. No remote mutation begins before its durable intent commit.
2. Every mutation is idempotent and verified by exact remote readback.
3. `closing` requires the exact closing relation; `part_of` requires the exact non-closing relation and proves the target issue remains open after PR publication and checkpoint merge observation.
4. Same-repository shorthand normalizes to a qualified identity, while split repositories reject unqualified linkage in either mode.
5. `pr watch` is foreground, cancellable by root signals, bounded, and leaves no persistent job or unjoined task.
6. Fake-adapter tests prove that a `part_of` watch cannot report checkpoint-ready unless exact REST issue readback still observes the qualified target issue open; closed, missing, stale, or contradictory observations produce reconciliation-required.
7. Every watch sleep and network await is selected against root cancellation; cancellation drains and joins the watch scope before exit 130.
8. Default and overridden timeout/poll values remain within the V3-01 bounds and timeout exits without a persistent job or unjoined task.
9. If `now + max(poll_interval, retry_after)` exceeds the fixed deadline, watch exits immediately without sleeping past the deadline.
10. Merge occurs only when the approved explicit policy and operator authority are both present.

## Dependencies

- V3-04: issue #165
- V3-08: issue #169
- V3-09: issue #170
- V3-12: issue #175
- V3-13: issue #176

## Inputs

- docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-14
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Finish, cleanup, detached watchers, polling daemons, implicit merge, remote rollback, or terminal issue closure reconciliation.
