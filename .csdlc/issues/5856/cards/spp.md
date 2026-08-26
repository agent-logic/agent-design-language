# Structured Planning Prompt

Template: 1.0.0

Issue: 5856

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Verify the complete final-sprint child wave, begin with WP-20 only after its proof producers are terminal, preserve the strict WP-20 through WP-30 dependency chain, and synthesize one integrated release-tail review after child completion.

## Plan

Revision 4

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
    "status": "pending"
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

.csdlc/prepared/issues/5856/design.md

Digest: ffb2155a6bd9d980c3a587e716f9643e5b08ea84980b0c04ecb885d38ad95441

## Diagram

.csdlc/prepared/issues/5856/diagram.mmd

Digest: c1817d53601997245eb63ec05c5b94d57d7520beaa4ab96860d3e5cf8b70a172

## Stop Conditions

- Any overlapping child write ownership
- Any missing child issue or card bundle
- Any required dependency not represented in the packet

## Handoff

Proceed only after doctor readiness.
