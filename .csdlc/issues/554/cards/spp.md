# Structured Planning Prompt

Template: 1.0.0

Issue: 554

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Patch the stale docs invariant, diagnose Runtime-v2 timeout cause, make the narrow reliability fix, validate focused tests, review, publish, shepherd, and merge #554 before returning to #549.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "Restore the v0.92 README Memory Palace production-authority wording expected by the retained invariant.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "step-2",
    "action": "Diagnose and repair Runtime-v2 unified-runtime-kernel timeout behavior without weakening tests.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "step-3",
    "action": "Validate, review, publish, shepherd, and merge the #554 repair before rerunning #549.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "in_progress"
  }
]

## Invariants

- Required coverage remains authoritative.
- No #483 or #514 behavior changes.
- Failures remain visible and reviewable.

## Risks

- Timeout fix could mask a real runtime-v2 defect if over-broadened.
- Docs repair could overstate Memory Palace completion if not bounded.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/issues/554/authored/design.md

Digest: c77477c95fd48ab5b2f60f7db680c50e69e3db6cfde4c159f70c4f3c8622d39d

## Diagram

.csdlc/issues/554/authored/diagram.mmd

Digest: 47c42c6d63b7770ce876f72a2b74795e4af59689a4ac0f169e3701b7c7f8c491

## Stop Conditions

- Fix requires changing #483.
- Fix requires weakening coverage or skipping tests.
- Required checks fail for a new non-#554 cause.

## Handoff

Proceed only after doctor readiness.
