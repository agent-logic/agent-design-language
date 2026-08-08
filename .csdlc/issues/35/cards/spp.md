# Structured Planning Prompt

Template: 1.0.0

Issue: 35

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Establish the canary and timeout contract, run one project-discovery and one projectless-dispatch probe, reconcile live task ownership, classify the owning component, retain portable evidence and guidance, then obtain independent review.

## Plan

Revision 5

## Steps

[
  {
    "id": "step-1",
    "action": "Define the disposable canary, timeout, terminal result schema, and pre-dispatch live task inventory.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "step-2",
    "action": "Run one bounded project-discovery probe and one bounded projectless task-creation probe.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "step-3",
    "action": "Read back any returned task identity and reconcile pre/post live inventories before transferring or retrying ownership.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "step-4",
    "action": "Retain portable evidence, classify the owning component, and write bounded operator guidance or an actionable upstream report.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "step-5",
    "action": "Run focused validation and independent ownership/safety review.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Ownership transfers only after a returned task ID is derived from complete task inventory receipts and verified through a digest-bound codex.read_thread receipt.
- A timeout, indeterminate result, missing task ID, incomplete inventory, or unverified new task prohibits retry and is never implicit success.
- Only an explicit typed failure with a complete empty task-ID delta may authorize retry.
- Evidence contains no credentials, personal data, raw prompt content, or machine-local absolute paths.
- Independent review authority comes from the canonical issue-35 review assignment and completed review, not a self-authored evidence assertion.

## Risks

- A late task creation response could appear after the caller times out.
- Project discovery and task creation may fail in different components.
- A canary could accidentally target a live issue if selection is not fail closed.
- The owning defect may be upstream and not repairable in this repository.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/35/design.md

Digest: 87b8cf57616431515a1cf5967d3aed328fdcb8b64bfed397cf2fcdff68922a0b

## Diagram

.csdlc/prepared/issues/35/diagram.mmd

Digest: c7bac06fe66dfd1d4d31d440f38a2be93ea825eb4cc8daf9732a36991b9cbbf2

## Stop Conditions

- No disposable canary target is available.
- The task API cannot provide a live inventory for ownership reconciliation.
- A probe returns indeterminate state and duplicate ownership cannot be excluded.
- The proposed change would modify unrelated ADL lifecycle or product code.

## Handoff

Proceed only after doctor readiness.
