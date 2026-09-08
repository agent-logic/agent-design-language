# Structured Planning Prompt

Template: 1.0.0

Issue: 518

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After #517 merges, freeze the candidate, inventory canonical documents, resolve documentation findings, and emit one exact-revision review plus context-free external handoff.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Verify #517 reviewed-merge authority and freeze the exact documentation candidate.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Build the canonical document denominator and validate every path, link, issue reference, and authority claim.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Resolve in-scope documentation findings and record residual risks, deferred scope, and non-claims.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Emit and validate the exact-revision documentation review and context-free external handoff packet.",
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

- #517 lacks a reviewed merge
- A release claim lacks evidence
- The candidate changes during review
- A required input cannot be resolved

## Handoff

Proceed only after doctor readiness.
