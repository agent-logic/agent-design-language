# Structured Planning Prompt

Template: 1.0.0

Issue: 283

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind #283, verify #209 replacement terminal evidence and #5832 historical evidence, then record a narrow issue-local reconciliation packet for the #207/#288 handoff.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bind #283 to its own branch and FastWork worktree after root collision checks.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Verify live #209 / PR #215 and derived terminal cache identity.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Verify #209 local/native manifests and referenced non-empty artifacts.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Inventory #5832 historical evidence and classify superseded/retained status.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Record #283 reconciliation packet, validation evidence, and C-SDLC truth for handoff.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Evidence is issue-local until #288 serializes shared ADR docs
- Historical #5832 artifacts are preserved byte-for-byte
- Replacement proof must be exact-head, merged, and artifact-bound
- ADR 0065 is not Accepted by this issue

## Risks

- Mistaking stale #5832 receipts for terminal current proof
- Treating live GitHub closure as sufficient without typed terminal/evidence artifacts
- Leaking #288 shared-doc scope into #283
- Recording empty or non-revision-bound validation artifacts

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/283/design.md

Digest: 3962d88d374f0e6c6fca4d5760525893a090b40206a94da7a2f86724916dcfa7

## Diagram

.csdlc/prepared/issues/283/diagram.mmd

Digest: 1faedd6e80b633b6c5a952138912e3e04faf300ea7c670a141197eddf8d4037c

## Stop Conditions

- No terminal owner evidence can be found for ADR 0065
- #209 live merge/linkage conflicts with local terminal cache
- Required manifests are empty or not exact-revision-bound
- Shared ADR docs would need editing before #288

## Handoff

Proceed only after doctor readiness.
