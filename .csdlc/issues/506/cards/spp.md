# Structured Planning Prompt

Template: 1.0.0

Issue: 506

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Implement the smallest deterministic DRT-A contract, prove ACIP identity/authority/replay behavior, and retain exact evidence without cloud or UI scope.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Map requirements 181 and 182 into a committed DRT-A qualification denominator.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement deterministic identity and authority checks without synthetic provenance.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Implement replay and duplicate-denial receipts that cannot mutate authority.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Exercise the fail-closed negative matrix for stale, duplicate, reordered, malformed, unsigned, wrong-domain, cross-Polis, and authority-mutation inputs.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "completed"
  }
]

## Invariants

- Identity provenance is not synthetic.
- Replay cannot change authority.
- Duplicate denial is receipt-exact rather than string-only.
- The issue remains independent from paid AWS execution and Observatory redesign.

## Risks

- Replay behavior could mutate authority if the contract boundary is weak.
- Synthetic identity provenance could make qualification evidence non-proving.
- Duplicate-denial evidence could become string-based instead of receipt-exact.
- Negative coverage could miss a recipient or authority variant.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/506/design.md

Digest: 88120bc83fdbe01e9af1298bb6dd178c8639b6a041c2b0a2f7239b2ecd8bd744

## Diagram

.csdlc/prepared/issues/506/diagram.mmd

Digest: 37f9fb27c6cc7258c306155f658c74be781bdcbdd4a141ab6d4fc65e0e3f780d

## Stop Conditions

- Identity provenance is synthetic.
- Replay can change authority.
- Duplicate-denial receipts are not exact.
- Paid AWS execution is requested without explicit operator authorization.

## Handoff

Proceed only after doctor readiness.
