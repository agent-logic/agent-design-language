# Structured Planning Prompt

Template: 1.0.0

Issue: 387

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap, design-review, bind, implement the narrow typed repair route, validate with focused gate5 regression, obtain exact review, publish, and finish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and approve the narrow #387 design.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Bind #387 to a FastWork worktree.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Implement typed authorization and guard changes.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Add focused regression for #114-shaped sequence.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Run focused validation and exact review before publication.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- Publication still requires current exact-head review
- CAS and audit sequence remain mandatory
- No reviewed/published mutation without typed recovery

## Risks

- Over-broad implemented-phase edits could weaken lifecycle truth guards
- Regression must prove negative reviewed/published cases still fail

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/387/design.md

Digest: 1694a81f4a60ff232dc49e3730de01b09ad73f47a48b08dea36e4f4ef8baf554

## Diagram

.csdlc/prepared/issues/387/diagram.mmd

Digest: cac872df5ac7932e44545adf4c7debf83c3378a14dc11eee099afb2ac9b99ad5

## Stop Conditions

- Any source change outside csdlc-v2 tooling scope
- Any guard weakening that allows publication without review
- Any mutation of #114 cards from #387 before #387 is terminal

## Handoff

Proceed only after doctor readiness.
