# Structured Task Prompt

Template: 1.0.0

Issue: 5007

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Preparation only: seed v2 cards, concise design and diagram, then run focused doctor.

## Deliverables

- Minimal six-card v2 packet for #5007.
- Concise design and Mermaid diagram.
- Focused csdlc-doctor result.

## Acceptance

1. AC1: All six v2 cards exist for #5007.
2. AC2: Design and diagram identify the later execution boundary and #4760 proof dependency without implementation.
3. AC3: Focused v2 doctor is run and recorded in the handoff report.

## Dependencies

- #4760 complete Memory Palace implementation proof.
- ADR 0051 deferred disposition.
- Chronosense implementation sprint #4765 and Memory Palace temporal index proof #4768.
- Long-running context continuity proof #4771.
- Existing .adl/v0.91.7 task bundle for #5007.
- origin/main at 79c7dccf12540863f6c038e1fd7ef45e2357a55e.

## Inputs

- .adl/v0.91.7/tasks/issue-5007__v0-91-7-wp-14-adr-accept-memory-palace-architecture-after-complete-implementation-proof/stp.md
- docs/adr/0051-chronosense-and-memory-palace-adr-disposition.md
- docs/milestones/v0.91.7/WBS_v0.91.7.md
- docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml
- docs/milestones/v0.91.7/V092_HANDOFF_v0.91.7.md

## Non Goals

- Do not write an accepted ADR from planning intent alone.
- Do not implement Memory Palace in this ADR issue.
- Do not expand v0.92 claims beyond retained proof.
