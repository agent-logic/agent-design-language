# Structured Planning Prompt

Template: 1.0.0

Issue: 5411

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Clarify selector claims, harden the guardian process boundary, connect weather pressure to signed continuity and graceful shutdown, classify release evidence, then run exact focused and full Runtime v3 proof.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "Normalize selector and release-proof semantics around reporting and evidence classes",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "step-2",
    "action": "Harden guardian process-tree and bounded capture behavior with descendant tests",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "step-3",
    "action": "Integrate periodic pressure sampling with signed checkpoint and graceful kernel stop",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "step-4",
    "action": "Run focused, full, lint, inventory, and independent review proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Runtime v3 remains independent of Runtime v2
- Continuity is committed before a pressure stop is called clean
- Descendants do not outlive guardian containment
- Non-executed evidence never becomes live completion truth

## Risks

- Platform-specific process-group behavior
- Shutdown races between weather, continuity, and control API
- Release records overstating ignored or contract-only proof
- Code growth above the Runtime v3 budget

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

docs/reviews/v0.91.7/runtime-v3-5411/DESIGN.md

Digest: 091f92699f7b7466b81dbc1354da01dd26db42fd2138079457b2221a6ec4316c

## Diagram

docs/reviews/v0.91.7/runtime-v3-5411/DIAGRAM.mmd

Digest: 9ee22bae790ee4bef0e47ca9b727ff62d55b83daa9f253fa71bf301da7908c19

## Stop Conditions

- Any required change enters Runtime v2
- #5409 protected paths must change
- Signed continuity cannot complete before shutdown
- Focused process or pressure tests remain nondeterministic

## Handoff

Proceed only after doctor readiness.
