# Structured Planning Prompt

Template: 1.0.0

Issue: 248

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Diagnose the deadline/output observation order, implement one bounded server-owned precedence rule, prove both terminal cleanup paths repeatedly, then run required Runtime proof and exact-head review.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Map and correct process-backend deadline versus output-limit arbitration.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Add and repeatedly run focused pressure and cleanup proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run required Runtime validation, strict Clippy, Observatory proof, and exact-head review.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Timeout remains fail-closed when no oversized output is observable.
- Output limits remain bounded.
- Owned process trees are terminated.
- Output artifacts are removed on every terminal path.

## Risks

- Inspecting output too early could misclassify an ordinary timeout.
- Inspecting after cleanup could erase precedence evidence.
- A fixture-only delay could conceal the production race.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/248/design.md

Digest: 47d4e695d336c08c3e31ed7c55187bb3cb0d27d1fb0d2136f6bbc48fe1176b67

## Diagram

.csdlc/prepared/issues/248/diagram.mmd

Digest: 81cc32fcf11ac677b734370ec3dc0eace18a4820f36de8e73bb352594161c266

## Stop Conditions

- Any required change overlaps #112 authority or #244 cleanup hooks.
- The deterministic rule weakens ordinary timeout, cancellation, or cleanup semantics.
- Required Runtime validation exposes a distinct unrelated blocker.

## Handoff

Proceed only after doctor readiness.
