# Structured Planning Prompt

Template: 1.0.0

Issue: 686

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Define the receipt and active-reference contract, wire it through the existing transactional startup/reload path and readiness identity, then prove all interruption boundaries and prior-generation restoration using isolated fixtures.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Define the immutable receipt, redacted secret-reference contract, compatibility checks, and atomic active reference.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Propagate and validate generation identity through CSM, Guardian, kernel, status, and readiness while reusing existing transaction primitives.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add deterministic failpoint and restoration proof, run focused validation, obtain exact-head independent review, and publish a ready PR.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "completed"
  }
]

## Invariants

- One committed configuration generation is authoritative
- Activation never exposes a partial candidate
- All service participants agree on generation and digest
- Receipts contain no secret values
- Recovery preserves the prior committed generation
- Validation never touches the live Runtime

## Risks

- A pointer can advance before candidate state is durable
- Mutable files can diverge from recorded identity
- Secret material can leak into receipts
- Binary/config incompatibility can be detected too late

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/686/design.md

Digest: ca33efced30f5574eee1c97e51ed73e21ba0799cdd4a54b41d7fb3167735aad0

## Diagram

.csdlc/prepared/issues/686/diagram.mmd

Digest: 98c905c37eb98e035fd0c8c4d579e72c6abaff90728fdac3b3e5719d824e115f

## Stop Conditions

- Implementation would mutate the live Runtime
- Scope expands into binary installation convergence providers or Observatory
- Secret values would enter retained artifacts
- Primary main becomes tracked dirty

## Handoff

Proceed only after doctor readiness.
