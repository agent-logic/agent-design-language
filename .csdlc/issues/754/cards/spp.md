# Structured Planning Prompt

Template: 1.0.0

Issue: 754

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Validate reviewed registry fix, preserve fail-closed tests, publish separate tooling PR. Correct stale guidance/canonical-authority test for completed cutover, preserving isolated pre-cutover fixture coverage.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Implement explicit compatible registry identities and negative regressions.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Validate full standalone checks and independent review; publish separate repair.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  }
]

## Invariants

- Native compact1.0.0 unchanged
- Reject incompatible authority before writes

## Risks

- Unsupported future registry must remain rejected.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/754/design.md

Digest: 557ab4fb1539c0b89deb53c4993f92cec4aaa683cef4a2b025b71d56bdc764aa

## Diagram

.csdlc/prepared/issues/754/diagram.mmd

Digest: 212b8f8ec158c68d18a3d80c00f4c31d7cb1f9e1f69dc923b1ee685bee158ae7

## Stop Conditions

- Identity or native shape drift
- Failed required checks

## Handoff

Proceed only after doctor readiness.
