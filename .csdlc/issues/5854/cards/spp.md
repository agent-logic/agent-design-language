# Structured Planning Prompt

Template: 1.0.0

Issue: 5854

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Validate the current child wave, preserve completed WP-24 product truth, route only dependency-satisfied #5835, #5836, #5838, #5839, and #5840 to separate FastWork sessions, treat WP-24A as an independent out-of-band stream, and synthesize one integrated sprint review after the five operative children complete.

## Plan

Revision 7

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
    "status": "pending"
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

.csdlc/prepared/issues/5854/design.md

Digest: a0a6e1b058b5e13de3776447dc39c940a3bf51a0d1b80b51e31fa1fbeb16a477

## Diagram

.csdlc/prepared/issues/5854/diagram.mmd

Digest: 18ce9d40dd473f07a2fee4d52ebcdeae0635b197e62c970ec5d507c77ca279d8

## Stop Conditions

- Any overlapping child write ownership
- Any missing child issue or card bundle
- Any required dependency not represented in the packet

## Handoff

Proceed only after doctor readiness.
