# Structured Planning Prompt

Template: 1.0.0

Issue: 504

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #504, prepare the V3-E design and validation packet, wait for #503 terminal/ancestral truth before implementation, then build the construction-only remote delivery workflow with exact review, publication-linkage, terminal-finish, and cleanup-safety refusal proof.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap #504 and prepare the V3-E design, diagram, and dependency gate.",
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
    "action": "After #503 is terminal and ancestral, bind the #504 execution worktree.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement construction-only v3 remote delivery review, publication, finish, and cleanup modeling.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Prove positive and refusal behavior for requirements #174 through #178.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Obtain fresh exact-head review and publish with visible `Closes #504` linkage.",
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

- Every remote delivery step is modeled as construction-only before #505
- Review cannot self-authorize publication or terminal truth
- Publication mode is explicit and cannot silently degrade closing linkage
- Finish derives terminal truth from governed publication state
- Cleanup is only available after terminal truth and remains a separate transition

## Risks

- Remote workflow modeling could accidentally imply live v3 authority before #505
- Review, publication, finish, and cleanup could collapse into one unreviewable helper
- Refusal cases for requirements #174 through #178 could become string assertions instead of behavioral proof
- The #503 dependency could be bypassed if preparation work drifts into implementation

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/504/design.md

Digest: 6eb4d3c5e97fb57e8cf22a8d2d689e54a012aa5c0dd8f8df9a1e7c2c206413c6

## Diagram

.csdlc/prepared/issues/504/diagram.mmd

Digest: 5421dcacb842a64acf16491135479b08ed32764db8b008dabea548a16db052e6

## Stop Conditions

- #503 is not terminal, reconciled, and ancestral when implementation would begin
- Any v3 command path performs live GitHub, finish, or cleanup mutation
- Review can be self-authorized
- Publication or finish can bypass typed gates
- Cleanup can run before terminal truth

## Handoff

Proceed only after doctor readiness.
