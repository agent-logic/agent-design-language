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

Digest: c610e2ddfe9484a8be12c6ae5bc0a5f8e5d39fa5247a771bdb34efb74c54a7c1

## Diagram

.csdlc/prepared/issues/22/diagram.mmd

Digest: 7172e260db3474a20353d51e23fea1917bb7b6056a8c10f7d0cb3106666fc447

## Stop Conditions

- Official archive checksum cannot be verified
- The change requires host-time installation or unrelated workflow redesign
- Focused tests reveal regression outside the declared builder surface

## Handoff

Proceed only after doctor readiness.
