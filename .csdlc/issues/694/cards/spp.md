# Structured Planning Prompt

Template: 1.0.0

Issue: 694

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Trace the current history and UI restore paths, add a complete bounded Runtime history representation for both roles, invoke restoration during fresh connection, deduplicate replay, and prove the full isolated reload behavior.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Trace production conversation admission completion persistence history serialization and Observatory connection flows; establish stable ordered turn identity and privacy boundaries.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement complete conversation_history.v1 production history and wire fresh Observatory reload restoration with exact-once replay handling.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add and run focused deterministic tests plus the isolated production-path reload/reconnect acceptance until both transcript halves restore exactly once.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Obtain independent exact-head review, fix findings, and publish a non-draft PR without merging.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "in_progress"
  }
]

## Invariants

- Every visible transcript entry is Runtime-authorized
- Private agent memory never enters operator history
- Ordering and deduplication are deterministic
- Reload does not depend on browser-local state
- Validation does not touch the permanent Runtime

## Risks

- History could disclose non-public payload fields
- Live/history replay could duplicate turns
- Completion events may lack outbound text
- UI restoration could race with the first feed snapshot

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/694/design.md

Digest: 3f5f4563b2ba93b5e22d80d68fe03e02f2166ed02ec06c128aa041065722cb02

## Diagram

.csdlc/prepared/issues/694/diagram.mmd

Digest: a8b041a57cad8d3bf4f80c9b0302c5b5e4e53eca9459baa5fa5d346228f017dd

## Stop Conditions

- Implementation requires live Wuji mutation
- Scope expands into agent-to-agent behavior
- A duplicate open issue owns the same defect
- Primary main becomes tracked dirty

## Handoff

Proceed only after doctor readiness.
