# Structured Planning Prompt

Template: 1.0.0

Issue: 288

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap and bind #288, serialize shared ADR docs from terminal #283-#287 evidence, retain an internal review handoff packet and validator, prove documentation/evidence truth, obtain fresh exact review, publish, shepherd CI, and finish #288 if green.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Validate #283-#287 terminal caches and bind #288 to a FastWork worktree after design approval.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Update the ADR index, ADR plan, review evidence manifest, handoff packet, and issue-owned serialization validator.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused docs/evidence proof, typed validation, diff hygiene, fresh exact review, publication checks, and typed finish.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- No ADR is marked Accepted by #288
- Every changed shared ADR status matches the issue-owned evidence manifest
- Residual gaps from #284, #285, #286, and #287 remain visible
- All #283-#287 terminal merge commits are ancestral to the implementation head
- Handoff review lanes are requests for review, not proof of completed review

## Risks

- Over-promoting Deferred ADRs despite residual gaps
- Accidentally implying #207 or ADR acceptance closeout
- Changing review evidence manifest claims without exact terminal cache binding
- Letting stale #287 or child projections override canonical terminal cache truth

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/288/design.md

Digest: b3443502549f134d4d38c2803842384ccc8389b81bc41a2f75edd17537586e31

## Diagram

.csdlc/prepared/issues/288/diagram.mmd

Digest: 6612c61bc13fcc1ed3f9877a003a5b200263098e92f616a561fe9ce4c4cf60c2

## Stop Conditions

- Any #283-#287 terminal cache is missing, non-canonical, or not ancestral
- A shared ADR surface would need to mark an ADR Accepted
- A status change would require provider, Unity, cloud, or implementation proof outside #288
- Fresh exact-head review finds an actionable issue

## Handoff

Proceed only after doctor readiness.
