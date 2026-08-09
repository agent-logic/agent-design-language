# Structured Planning Prompt

Template: 1.0.0

Issue: 5855

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify the full child batch, route safe independent lanes to separate sessions, preserve serial gates, and synthesize one integrated sprint review after child completion.

## Plan

Revision 2

## Steps

[
  {
    "id": "readiness",
    "action": "Validate the Sprint Execution Packet and all child issue cards before handoff",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "coordinate",
    "action": "Route child sessions according to declared lanes and gates",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "review-close",
    "action": "Review integrated results and close the umbrella only after child terminal truth",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Child issues retain all code and proof authority
- No child begins before its declared dependencies
- No umbrella closeout substitutes for child closeout
- Parallel lanes use separate child worktrees and issue-bound goals

## Risks

- A session could mistake coordination authority for child implementation authority
- A parallel lane could start before a serial dependency is complete
- An umbrella could overstate sprint completion while a child remains nonterminal

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5855/design.md

Digest: d5fa7a5b5238ac65773c861a4ddd37e60d85600a156e3091ba8db78883b72d4e

## Diagram

.csdlc/prepared/issues/5855/diagram.mmd

Digest: 143bcc094efbbc095712351c5619ccb059308940448052fe6e3b8e9a3f483904

## Stop Conditions

- Any overlapping child write ownership
- Any missing child issue or card bundle
- Any required dependency not represented in the packet

## Handoff

Proceed only after doctor readiness.
