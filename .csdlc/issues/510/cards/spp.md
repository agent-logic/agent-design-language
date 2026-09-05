# Structured Planning Prompt

Template: 1.0.0

Issue: 510

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Reconcile dependencies, implement the HOT-01 Axum hot-reload surface, validate all planned PVF lanes, obtain exact-head review, and publish a PR without merge.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Reconcile live issue, sprint, readiness, ownership, and dependency gates before implementation.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement atomically swappable configuration state with valid reload replacement and invalid-update retention.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add debounced file watching and clean shutdown behavior.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Prove concurrent readers only observe complete configurations and run all HOT-01 validation lanes.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Obtain exact-head review, fix actionable findings, and publish a PR without merge.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- Readers observe whole configuration snapshots only.
- A failed parse or validation attempt leaves the last-known-good state active.
- File notification bursts collapse through a debounce boundary.
- Watcher shutdown has an explicit cancellation path.
- DEC-01 #513 must wait before editing the #510 runtime hot-reload files.

## Risks

- Partial config state could leak to concurrent readers.
- Invalid reload input could clear or poison the active config.
- File watcher tasks could survive test/runtime shutdown.
- Debounce timing could make tests flaky if wall-clock assumptions are too tight.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/510/design.md

Digest: a00e20d94c57a7bc2cec5b2dc2d1fae9bf6f2a5bf0a7f2297d183e6cd5d75798

## Diagram

.csdlc/prepared/issues/510/diagram.mmd

Digest: bbb357108f9dcd0cb06d3bc08c2bef41f83e0f56b8cc02521e90773e939d3736

## Stop Conditions

- Reload requires process restart.
- Partial configuration can become visible.
- The typed lifecycle projection cannot be recovered without direct state mutation.
- DEC-01 #513 or another session starts editing the same runtime hot-reload files.

## Handoff

Proceed only after doctor readiness.
