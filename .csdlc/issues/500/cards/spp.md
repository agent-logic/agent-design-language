# Structured Planning Prompt

Template: 1.0.0

Issue: 500

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Inventory retained predecessor requirements, define the versioned v3 contract and construction decisions, specify rollback and coexistence boundaries, then obtain focused validation and exact-head review.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Inventory requirements #161 through #163 and map them exactly into the retained v3 contract.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Write the versioned v3 contract with explicit v2 coexistence and authority boundaries.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Classify each existing checkpoint, projection, review, and transition as retained, collapsed, derived, or removed by named risk; record construction, rollback, and simplified operator-flow decisions; then run one focused proof and one independent implementation review.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "completed"
  }
]

## Invariants

- C-SDLC v2 remains sole operational authority.
- Every retained requirement from #161 through #163 has exactly one explicit disposition.
- Every v3 checkpoint, projection, review, or transition mitigates a named concrete risk; otherwise it is removed, collapsed, or derived.
- The default path has one meaningful design gate, focused validation, one independent implementation review, and truthful closeout; no duplicate authority or umbrella re-review of child proof.
- A routine three-issue sprint is mechanically prepared and ready in minutes without hand-authored lifecycle JSON or repeated generation and digest choreography.
- Rollback and unsupported-platform behavior fail closed.

## Risks

- A retained predecessor requirement is omitted or duplicated.
- Compatibility language silently grants v3 authority.
- Construction decisions drift from measured #162 or approved #163 evidence.
- Minimal crate work widens into implementation.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/500/design.recovered.md

Digest: 4b504077ccfe28e72ea0aeb68c2536646221b7c4e495586fa454e763ab5a40fd

## Diagram

.csdlc/prepared/issues/500/diagram.recovered.mmd

Digest: 9ff4af50a6468e9208cc8a8b39bc7bac3e7b8b3986ed2cf162d99d32b73a752f

## Stop Conditions

- The v2 coexistence boundary is ambiguous.
- A predecessor requirement is unmapped.
- Construction requires authority cutover or v2 retirement.
- Work expands into V3-B or later implementation.

## Handoff

Proceed only after doctor readiness.
