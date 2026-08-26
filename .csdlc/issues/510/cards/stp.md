# Structured Task Prompt

Template: 1.0.0

Issue: 510

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue completion is exactly one production-ready hot-reload implementation; behavioral cases are proof inputs.

## Deliverables

- One production-ready Axum hot-reload implementation with last-known-good retention.
- Focused runtime hot-reload tests covering valid reload, invalid retention, debounce, concurrent-read consistency, and watcher shutdown.
- Bounded validation evidence.
- Exact-head review receipt.

## Acceptance

1. AC-1: Reads use atomically swappable state.
2. AC-2: Invalid updates preserve the last valid configuration.
3. AC-3: File events are debounced.
4. AC-4: Concurrent requests observe complete configurations only.
5. AC-5: Watcher shutdown is clean.

## Dependencies

- WP-01 #480 merged opening gate.
- Sprint 1 umbrella #529 owns coordination only.
- DEC-01 #513 is gated behind #510 for overlapping runtime hot-reload files.

## Inputs

- https://github.com/agent-logic/agent-design-language/issues/510
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#HOT-01
- docs/milestones/v0.92.1/SPRINT_v0.92.1.md
- .csdlc/prepared/issues/510/design.md
- .csdlc/prepared/issues/510/diagram.mmd

## Non Goals

- Database pool replacement.
- HTML template reload.
- Admin mutation API.
- Runtime v2/v3 authority topology work owned by DEC-01 #513.
- Provider profile work.
- Merge, closeout, or unrelated sprint execution.
