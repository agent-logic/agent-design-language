# Structured Planning Prompt

Template: 1.0.0

Issue: 675

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Inspect the existing #662 kernel primitive and live Observatory conversation path, add a first-class A2A action bridge for model-backed agent initiation, update UI/activity projections as needed, and prove the live-style Beacon-to-Ember route deterministically.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Map current operator conversation, A2A primitive, resident Shepherd/provider, and Observatory rendering paths.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the first-class model/shepherd A2A action bridge while preserving Runtime authority checks.",
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
    "action": "Update Observatory/UI handling so A2A initiation and terminal response are visible and distinct from operator chat.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused deterministic validation covering live-style success, failure semantics, and #662 regression preservation.",
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

- Runtime, not model text, decides whether A2A initiation is accepted
- A2A delivery records distinct sender and recipient identities
- Provider responses cannot be confused with the initiating agent's own reply
- Layer8 authority/signing and roster eligibility are not weakened
- No live external side effects occur without operator authorization

## Risks

- Prompt-only routing could mask missing action authority
- Sender eligibility could incorrectly require recipient eligibility semantics
- UI could render accepted dispatch as delivered truth before terminal result
- A2A response could be attributed to the wrong agent

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/675/design.md

Digest: 5e787ecedf1ff89ef8cc9d20ed325e778e8028fa97dccb26f58dadca83250628

## Diagram

.csdlc/prepared/issues/675/diagram.mmd

Digest: 00674d0782ab1e5022f79edeb41010c83cadfce7689adcb227452e85c83624d2

## Stop Conditions

- The bridge requires live provider/AWS execution without authorization
- Layer8/admission authority would need to be weakened
- A2A response attribution cannot be made deterministic
- Implementation requires broad autonomous messaging beyond #675

## Handoff

Proceed only after doctor readiness.
