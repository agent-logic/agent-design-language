# Structured Planning Prompt

Template: 1.0.0

Issue: 608

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Make a narrow provider patch, prove endpoint/body rendering with unit tests, then run live provider workflows for regional 2.5 and global 3.x Gemini models.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Implement global endpoint/trust behavior and config-backed thinking body rendering.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Add focused provider tests for endpoint derivation, trusted host policy, thinking rendering, and invalid config rejection.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run live provider proof using approved company GCP key routing without exposing credentials.",
    "acceptance_ids": [
      "AC-9",
      "AC-10"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run final validation, fresh exact-head review, publish, CI, and finish.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "status": "completed"
  }
]

## Invariants

- Existing regional Vertex behavior is preserved
- No credential contents are exposed
- No provider dependencies are added
- Polis integration remains outside #608

## Risks

- Model catalog availability can drift
- Thinking config differs by Gemini model family
- Live proof may incur small Vertex API cost

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/608/design.md

Digest: 5a64409f42f933caa94256a02d1495f4efb154dee5c2c08e31da64751dbb6843

## Diagram

.csdlc/prepared/issues/608/diagram.mmd

Digest: f4f07e12be754dca44423d89d24bd9d2f286cf80168b581742a4008d99d405e9

## Stop Conditions

- Credential material would need to be printed or committed
- GCP project/model authority becomes ambiguous
- Fix would require #592 Polis integration or provider redesign

## Handoff

Proceed only after doctor readiness.
