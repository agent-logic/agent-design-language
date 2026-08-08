# Structured Planning Prompt

Template: 1.0.0

Issue: 22

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind issue 22, add a checksum-pinned Ruby source build and provenance, extend preflight and focused regressions, obtain independent review, publish, merge when green, and close out.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Add pinned Ruby and provenance to the immutable builder image",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Add fail-closed Ruby and validator preflight",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused contracts, review, publish, and merge",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Builder images remain content-digest pinned
- Validation tools remain inside the immutable image
- The requested validation command never runs after preflight failure

## Risks

- Ruby source build dependencies can increase image build time
- A weak fake-Docker contract could miss ordering regressions

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/22/design.md

Digest: 7b2526ad87abd5e2c78030750eef8b0f98b3e3eb0b64f8c4a502398c2705f1bf

## Diagram

.csdlc/prepared/issues/22/diagram.mmd

Digest: 44d8b30becc3ca642911dff89d91ae1bf822bd876f37c99e3aa8da654813dff9

## Stop Conditions

- Official archive checksum cannot be verified
- The change requires host-time installation or unrelated workflow redesign
- Focused tests reveal regression outside the declared builder surface

## Handoff

Proceed only after doctor readiness.
