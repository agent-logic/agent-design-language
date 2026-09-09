# Structured Planning Prompt

Template: 1.0.0

Issue: 518

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Correct the 15 source-grounded findings from the 737-document audit now under explicit operator authorization. Use bounded v2 transition remediation. Preserve historical proof and concurrent owner paths. Final candidate freeze, acceptance and external handoff wait for issue 517 passing reviewed merge.

## Plan

Revision 7

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
    "id": "S4",
    "action": "After issue 517 passing reviewed merge, refresh the exact candidate and finalize the context-free external handoff.",
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

- Do not finalize acceptance or external handoff before issue 517 passing reviewed merge.
- Do not claim a feature or release proved without exact-revision evidence.
- Revalidate if the candidate changes.
- Do not edit another issue owner quality evidence.

## Handoff

Proceed only after doctor readiness.
