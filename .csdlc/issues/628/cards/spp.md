# Structured Planning Prompt

Template: 1.0.0

Issue: 628

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Consume #627's command manifest, implement the local route dispatcher and construction-state flows, prove real issue startup and typed failure paths, then publish only after exact-head review.

## Plan

Revision 2

## Steps

[
  {
    "id": "628-1",
    "action": "Verify #627 command manifest availability and current non-authority boundary.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "628-2",
    "action": "Implement local route request/response handling and construction-state persistence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "628-3",
    "action": "Add real issue local canary and three-minute measurement proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "628-4",
    "action": "Add focused positive and negative route tests plus issue-owned validator.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "628-5",
    "action": "Run typed validation, exact-head review, and publication readiness without GitHub mutation beyond typed publish.",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- One binary remains named csdlc.
- C-SDLC v3 remains non-authoritative before #505.
- No csdlc-v2 source changes.
- Real issue canaries must not hide defects behind v2 fallbacks.

## Risks

- Local lifecycle scope can sprawl into GitHub/publication work owned by #629.
- Construction-state convenience could accidentally claim live authority before #505.
- Canary measurement can become non-reproducible if it depends on network or host-local state.

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/628/design.md

Digest: 7231104c12cd8148c993281ec2ce0d62e5f6d5bedd081699073327de8dc0b289

## Diagram

.csdlc/prepared/issues/628/diagram.mmd

Digest: 3d1bd2cd6f796c81e9af39e8bd4598975ff079cd04ff21b61c8d444eefe4c1de

## Stop Conditions

- Need to edit csdlc-v2 source.
- Need GitHub mutation before #629.
- Need #505 cutover approval.
- Cannot reproduce real issue canary locally.

## Handoff

Proceed only after doctor readiness.
