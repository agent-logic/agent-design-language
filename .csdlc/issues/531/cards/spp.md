# Structured Planning Prompt

Template: 1.0.0

Issue: 531

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Initialize and bind the sprint umbrella, collect live child issue and PR truth, write one sprint closeout artifact, validate it, run sprint-end review, then publish and finish only if the typed gates pass.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Initialize and bind #531 to a FastWork issue worktree after confirming main is clean and current.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Collect live GitHub and local C-SDLC disposition evidence for roster children #495, #489, #496, and #494.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Write the Sprint 3 closeout evidence artifact and static validator.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused validation and sprint-end review, then record review truth.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Use typed v2 publication and finish routes if review and validation gates pass.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- The umbrella never upgrades skipped, deferred, or missing child evidence to pass
- Live GitHub state is rechecked before sprint-state transitions
- No paid provider command runs without separate authorization
- No child implementation file is changed by the umbrella
- C-SDLC v2 remains the live lifecycle authority

## Risks

- A child issue may be closed before typed local finish or cleanup is complete
- Some child PR/check evidence may require explicit unavailable or deferred disposition
- Main may advance while the sprint closeout work is underway

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/531/design.md

Digest: b1a08651c35e236bc401e396cb2ba55d6db49bd88dfcc3e8cd91e52dd676abb8

## Diagram

.csdlc/prepared/issues/531/diagram.mmd

Digest: 28772d4fd6432d7758ce3c904c6a275f36140de01f34b9f735e75243b40b665f

## Stop Conditions

- Any declared roster child is reopened or rerouted without a typed membership update
- A child merge or PR disposition cannot be classified truthfully
- The closeout result would require paid cloud or production proof
- Typed v2 validation rejects the issue record or review state

## Handoff

Proceed only after doctor readiness.
