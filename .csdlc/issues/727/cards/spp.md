# Structured Planning Prompt

Template: 1.0.0

Issue: 727

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Preflight identity and source, initialize isolated state, generate and hash a saved plan, stop for exact authorization, apply only that plan, read back every control, and reconcile failures.

## Plan

Revision 5

## Steps

[
  {
    "id": "step-1",
    "action": "Verify identity, region, merged source, backend and workspace",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "step-2",
    "action": "Generate, inspect and hash the exact saved plan",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "step-3",
    "action": "Obtain exact operator mutation envelope",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "step-4",
    "action": "Apply saved plan and run redacted readbacks",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "step-5",
    "action": "Reconcile failure or retain reviewed proof",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- saved plan is immutable between authorization and apply
- remote state is isolated
- retained output is redacted
- unrelated resources are untouched

## Risks

- uncontrolled logging cost
- partial apply
- sensitive identifiers in raw provider output

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.adl/requests/727/design.md

Digest: b91974a49cda2ab20235799b6fcc9ed8c34030c8a61e007d1fa7d262cb0ec052

## Diagram

.adl/requests/727/diagram.mmd

Digest: af0932d23c487bbd77b0115e4f1f50f277d6ac1ae04b505137b4d334f87a0ced

## Stop Conditions

- identity or region mismatch
- backend ownership ambiguous
- plan contains out-of-scope resources
- authorization incomplete
- readback fails

## Handoff

Proceed only after doctor readiness.
