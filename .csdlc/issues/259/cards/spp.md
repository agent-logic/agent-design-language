# Structured Planning Prompt

Template: 1.0.0

Issue: 259

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Consume #258 authority-store boundary in governed transport, prove positive/negative authorization behavior, review, publish, and finish before #260 starts.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Inspect governed transport certificate authority flow and #258 adapter APIs.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Bind governed transport to authority-bound certificate handles without migrating non-transport callers.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run focused Runtime transport validation and strict Clippy.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Obtain fresh exact-head review and publish through typed v2.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- #258 authority-store boundary remains the only production authority-store seam consumed by transport.
- #259 does not alter #260 non-transport caller migration scope.
- Preserved #203 worktrees remain recovery evidence only and are not mutated.

## Risks

- Transport tests may need fixture authority handles after #258 sealing.
- Authority-store API consumption may reveal compile fallout in transport-adjacent tests; keep repair transport-coupled.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/259/design.md

Digest: aa9f512a7624c4d0f21418012831fb40b1f052cc431222bcf7f101c1de39d67c

## Diagram

.csdlc/prepared/issues/259/diagram.mmd

Digest: a0b0d56fbebcd8825ba181d20a910a356e0eda1b91e274f25c5bea2f8bb1a0fd

## Stop Conditions

- terminal #258 cache or ancestry becomes stale
- implementation requires non-transport caller migration owned by #260
- fresh review finds actionable P1/P2
- required validation or CI fails outside #259 scope

## Handoff

Proceed only after doctor readiness.
