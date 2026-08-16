# Structured Planning Prompt

Template: 1.0.0

Issue: 350

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap and review the sealed interface, bind after PASS, implement durable deadline retention and exact cross-binding, prove focused behavior and compatibility, review exact head, publish, pass CI, and finish before releasing #274.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Freshly review and approve exact design and proof boundaries.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement durable deadline retention and sealed cross-bound projection without widening scope.",
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
    "action": "Run focused tests, strict Clippy, scope and diff proof, then exact-head review.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Publish, shepherd required CI, finish, and prove terminal ancestry before #274 resumes.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- No caller-supplied field can mint projection authority
- Published quorum and exact #272 cut remain cross-bound
- Deadline authority is committed, durable, and restart-safe
- Projection is deterministic and redacted
- #274 remains unbound until terminal #350

## Risks

- Durable schema evolution could reject or misread legacy checkpoints
- A weak cross-binding could combine unrelated valid authorities
- Projection could leak signer or identity material
- Changes could regress existing authority consumers

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/350/design.md

Digest: 30c79e11c4d8659d20e6f443a5f6c72ebff57093d3f25e06b58f4a620a44d9ee

## Diagram

.csdlc/prepared/issues/350/diagram.mmd

Digest: 2094f41a435389c054a8b4a5261230f9261eb8cde61e33528ff96fcffaf0b772

## Stop Conditions

- Design review identifies an unresolved authority or compatibility gap
- Implementation requires #274, #273 behavior, #203, #205, #275, or undeclared runtime surfaces
- Legacy durable state gains authority through a default
- Any proof, review, CI, or terminal gate fails

## Handoff

Proceed only after doctor readiness.
