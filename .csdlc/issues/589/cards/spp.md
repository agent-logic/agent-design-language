# Structured Planning Prompt

Template: 1.0.0

Issue: 589

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Unify ordinary Guardian startup around supervised process ownership, add reliable CSM lifecycle commands and safe reconciliation, then prove first-start and reload behavior with focused tests.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "Characterize the failed Guardian startup and define the simplified ownership invariant",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "step-2",
    "action": "Implement CSM lifecycle commands and remove the separate startup continuity-channel dependency",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "step-3",
    "action": "Add focused tests and verify persistent local and AWS-facing readiness",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Exactly one live Runtime writer
- Guardian remains authoritative for kernel lifetime
- Reload never replaces a working configuration before candidate validation
- Recovery preserves retained Polis state

## Risks

- Incorrect stale-lock reconciliation could admit concurrent writers
- Removing the startup channel could accidentally weaken migration continuity boundaries
- Reload failure could interrupt an otherwise healthy Runtime

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/issues/589/authored/design.md

Digest: 54d41b6e7a358aa9588eee0ba71de8d6fe9bc385d8b1b6a0636b402b56f8dab8

## Diagram

.csdlc/issues/589/authored/diagram.mmd

Digest: c7217c238ee2a813ef2495c4f17742a13073af133bf3676469c49509daa38a81

## Stop Conditions

- The change requires discarding retained Wuji state
- Single-writer safety cannot be preserved
- The bounded fix expands into distributed migration protocol redesign

## Handoff

Proceed only after doctor readiness.
