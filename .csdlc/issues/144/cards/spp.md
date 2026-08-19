# Structured Planning Prompt

Template: 1.0.0

Issue: 144

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Freeze the signed authority and rotation contract, implement full-chain verification, add adversarial proof, retain exact local/native evidence, resolve fresh review, publish a qualified ready PR, and merge through typed authority before resuming #5831.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inspect existing Runtime v3 signing/governance APIs and freeze the trusted cognitive authority, full-chain, and rotation contract.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-5",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement signed authority anchoring, complete chain verification, and governed rotation in cognitive_profile owned paths.",
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
    "id": "S3",
    "action": "Add exact positive and adversarial tests plus local and native receipt validation.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Resolve fresh exact-head review, publish a qualified ready PR, shepherd native and standard CI, and merge through typed finish.",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "status": "completed"
  }
]

## Invariants

- No caller-supplied policy or evidence becomes its own authority
- Every accepted revision has one verified chain to genesis
- Authority epochs are monotonic and rotation is signed by the currently trusted authority
- Public projection remains strictly narrower and privacy-safe
- All retained evidence is exact-revision and repository-relative

## Risks

- A wrapper could appear signed while allowing caller-chosen verifying keys
- Immediate-predecessor checks could still hide a forged older revision
- Rotation could accidentally authorize rollback or same-epoch key replacement
- Native evidence could retain stale #5830 identities or machine-local paths

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/144/design.md

Digest: abbc7763f0b9b76e506eba224c054896d486104250412c28dc6634ce4507a41b

## Diagram

.csdlc/prepared/issues/144/diagram.mmd

Digest: ed3e101b563b88645cb09dc5fcfac4b7cab69bddcbf590c9a90ec4fc41ce214d

## Stop Conditions

- Trusted authority cannot be anchored without widening beyond existing Runtime v3 governance APIs
- Full-chain verification would require mutating legacy #5830 evidence
- Any edit targets adaptive-learning, Sprint 3, global CI, or primary main
- Focused proof or exact-head review finds unresolved authority bypass

## Handoff

Proceed only after doctor readiness.
