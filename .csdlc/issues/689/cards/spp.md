# Structured Planning Prompt

Template: 1.0.0

Issue: 689

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Rewrite the runbook around the stable current-generation Rust CSM command, make legacy shell Runtime verbs refuse with exact replacement guidance, preserve Observatory commands, and add narrow guards.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Replace stale Runtime lifecycle guidance with canonical current-generation Rust CSM status, start, stop, reload, path, label, and identity proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Make legacy CSMctl Runtime verbs refuse with canonical replacement commands while preserving Observatory-only behavior.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused routing/docs guards, run canonical ownership tests and quality checks, obtain exact-head review, and publish a ready green PR.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- The Rust csm runtime-v3 controller remains sole permanent Runtime authority
- Legacy shell commands never claim a different service as canonical
- Observatory control remains separate
- Validation never changes the live Runtime

## Risks

- Legacy callers may depend on CSMctl start
- Documentation may accidentally mix demo and permanent layouts
- A wrapper delegation could recurse or resolve the wrong binary

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/689/design.md

Digest: 7069adcc2ed6664ad1582c3ea43340c842b7179f132c98a76b6098b99bee2137

## Diagram

.csdlc/prepared/issues/689/diagram.mmd

Digest: 36966b883f7d8c54c8e812c97ad43c412e33123fb8b8861b82b71ab33597ab8e

## Stop Conditions

- Implementation would alter canonical ownership behavior
- Live Runtime mutation becomes necessary
- Scope expands beyond routing docs and tests
- Primary main becomes tracked dirty

## Handoff

Proceed only after doctor readiness.
