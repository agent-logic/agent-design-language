# Structured Planning Prompt

Template: 1.0.0

Issue: 5883

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Rebase after #5895, classify active versus historical references, delete the duplicate command and active requirements, update current guidance, and prove real installed claim-free creation and binding.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Rebase after #5895 and classify every csdlc-init occurrence as active or historical.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Delete the duplicate Cargo binary and remove it from current installer, coexistence, proof, and skill authority.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Update current non-historical docs and operator adapters to csdlc-issue create without compatibility wording.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused real-binary creation parity, installed inventory, and create/validate/doctor/bind proof.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- One issue-creation entrypoint
- Identical typed request and record semantics
- No claim or preparation concept
- Historical evidence unchanged
- V2 installed provenance exact

## Risks

- Missing a generated or installed-skill reference
- Editing historical evidence
- Overlapping #5895 inventory changes
- Accidentally weakening create rollback or idempotence tests

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5883/design.md

Digest: b1e41dd4edab0055eba3b55ac148d1ff93f188c59bbda718b6fb15badca8ba95

## Diagram

.csdlc/prepared/issues/5883/diagram.mmd

Digest: e4f0c640ff2be56cb72ac61ee6ce89ce4802a3ced636c0262544dd71f8723808

## Stop Conditions

- #5895 has not settled or shared files cannot be cleanly rebased
- Any proposed compatibility alias or wrapper
- Creation semantics require redesign
- Historical evidence would need rewriting

## Handoff

Proceed only after doctor readiness.
