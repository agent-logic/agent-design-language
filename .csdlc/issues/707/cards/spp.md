# Structured Planning Prompt

Template: 1.0.0

Issue: 707

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Canonicalize receipt identity, add a cross-manifest regression, build and install one coherent generation, restart through CSM, and prove real Beacon-to-Ember delivery.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Reproduce and isolate the cross-binary receipt mismatch.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement canonical identity and focused cross-manifest regression proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Build, verify, and install one coherent release generation with rollback retained.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run live Beacon-to-Ember A2A acceptance and inspect distinct delivery evidence.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Complete bounded review and typed publication.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- All binaries agree on identity without sharing mutable state
- Receipt mismatch always fails closed
- Only one launchd-managed Runtime owns the ports
- Rollback generation remains intact
- Operator reply is not evidence of agent-to-agent delivery

## Risks

- Changing canonical encoding could invalidate retained receipts
- Independent Cargo graphs may conceal further behavioral drift
- Live A2A model selection may decline an action even when transport works

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/707/design.md

Digest: c7489b245a9f56f3bd65a8458fc03cc3541cdbccb2144dd6f656ae373e8bb5b0

## Diagram

.csdlc/prepared/issues/707/diagram.mmd

Digest: 75c60391e6ad84271f10f460dffa80bc66b15bc93c4246795939268c6c36e752

## Stop Conditions

- Repair requires weakening identity validation
- The live init must change to pass
- A competing Runtime owns the listener
- Focused tests expose broader incompatible persisted-state migration
- Subagent review finds unresolved P1 or P2 defects

## Handoff

Proceed only after doctor readiness.
