# Structured Planning Prompt

Template: 1.0.0

Issue: 407

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Mirror the existing required_outcome recovery path for SIP goal with narrow authorization and regression proof.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and review #407 design.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Bind in FastWork and implement the narrow SIP Goal recovery operation.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused regression coverage and run csdlc-v2 tests.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Obtain fresh exact review, publish, CI, and finish.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Recovery must be typed, generation/digest gated, and audit-preserving.
- Unrecovered implemented SIP goal mutation must remain rejected.
- No lifecycle publication guard is weakened.

## Risks

- Accidentally authorizing broad implemented-phase SIP edits.
- Missing recovery-provenance guard parity with existing required_outcome repair.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/407/design.md

Digest: 21f11538fd99055fdb1089c53fcf59972bc5e26503446eaf7c5b7b8d49918387

## Diagram

.csdlc/prepared/issues/407/diagram.mmd

Digest: a76d9879e62ccf451ae3864ba83c6ba9018f82edcb111284d206500074693eac

## Stop Conditions

- Regression cannot distinguish recovered from unrecovered implemented state.
- Change requires broad lifecycle reset or raw card edits.

## Handoff

Proceed only after doctor readiness.
