# Structured Planning Prompt

Template: 1.0.0

Issue: 518

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Complete final documentation handoff against the merged issue 517 baseline. Operator clarified nothing is blocked: passing reviewed merge means predecessor review and integration, not a release PASS decision. Preserve TAIL-01 blocked release truth in the handoff. Refresh current inventory and snapshots, correct remaining scope prose, validate exact content and obtain independent review before typed publication.

## Plan

Revision 12

## Steps

[
  {
    "id": "S1",
    "action": "Confirm the authorized v2 transition route and verify audit findings against the bound baseline.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Correct the bounded current documentation surfaces and retain historical proof unchanged.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Validate inventory, links, claim dispositions and diff hygiene; obtain independent review.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Complete the additional 24-manifest Cargo audit, document its limits and obtain independent review.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "After issue 517 passing reviewed merge, refresh the exact candidate and finalize the context-free external handoff.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  }
]

## Invariants

- One exact revision anchors the packet
- Every claimed input is resolvable
- Documentation never overstates product or release state
- The issue does not publish the candidate

## Risks

- A canonical document could be omitted
- A local-only path could make the handoff non-portable
- Candidate drift could stale the review
- Documentation could conceal a residual risk

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/518/design.md

Digest: 8f4ca2061dfb5453165a1a01bb2c5423da24424e4ac188640d521a9914efb46b

## Diagram

.csdlc/prepared/issues/518/diagram.mmd

Digest: 9025468b12687281f293fbcb9b0c9155d8aa6aaed3360937fb9ed29e7a5e9958

## Stop Conditions

- Unsupported release claims or omitted residual risks.
- Candidate content drift invalidates review and requires revalidation.
- Do not mutate retained machine-readable TAIL-01 proof or implement product work.

## Handoff

Proceed only after doctor readiness.
