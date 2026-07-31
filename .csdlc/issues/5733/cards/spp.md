# Structured Planning Prompt

Template: 1.0.0

Issue: 5733

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #5733, inspect current issue/proof truth, reconcile the two canonical matrices with explicit claim boundaries, add a focused validator, record validation and review, then publish a ready PR.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Initialize and bind issue-local C-SDLC v2 lifecycle state.",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Inventory current matrix rows, feature-proof rows, #5354 convergence evidence, and issue-wave owner truth.",
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
    "id": "S3",
    "action": "Update the canonical docs and add deterministic validation for owners, evidence, dispositions, and contradictions.",
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
    "id": "S4",
    "action": "Run focused validation, exact-head review, publish, and shepherd the PR.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- every live/proven claim has an owner and evidence
- explicit blockers and non-claims are preserved rather than hidden
- planned work and runtime proof remain separate
- the #5354 convergence packet is consumed, not rerun
- public claim boundaries stay narrower than internal evidence

## Risks

- existing docs may contain stale issue status or proof language
- validator may need to encode only stable document structure to avoid brittle prose coupling

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5733/design.md

Digest: 3b0932e443cf6ed676cab582187dc83379097dfaf74cca4611a5dcc891108b16

## Diagram

.csdlc/prepared/issues/5733/diagram.mmd

Digest: 113d43e863ff002fc23bd0326e3c05055552b5f73afc7539b94edca4b384dd4d

## Stop Conditions

- protected-path collision is reported by typed v2 binding
- required #5354 evidence is absent or contradictory
- focused validator fails after repair attempt
- exact-head review returns actionable in-scope findings

## Handoff

Proceed only after doctor readiness.
