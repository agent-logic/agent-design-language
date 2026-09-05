# Structured Planning Prompt

Template: 1.0.0

Issue: 624

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Read existing corporate acceptance evidence, create a redacted hardening denominator, map every row to proof or follow-on action, validate hygiene and completeness, then obtain exact-head review.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Inventory #624 hardening denominator from issue body and #497/#634 seed evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Author the redacted operational hardening register and machine-readable receipt.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add focused validation for denominator completeness, evidence references, row dispositions, and secret hygiene.",
    "acceptance_ids": [
      "AC-4",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused validation and diff hygiene, then record SOR truth.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Request and satisfy bounded exact-head review before publication.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "completed"
  }
]

## Invariants

- No secrets or account IDs in tracked artifacts
- No live/admin mutation
- #497 acceptance remains separate from #624 hardening
- Every denominator row has proof or follow-on disposition
- Validation evidence is retained and exact-head scoped

## Risks

- Overclaiming hardening completion from read-only seed evidence
- Leaking sensitive control-plane or custody details
- Silently executing live account mutations
- Leaving an unproven row without a concrete owner/action
- Conflating #624 with unrelated issue 5624

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/624/design.md

Digest: 6aa84e4c78bdd78f4717d3788df93bf40cddede933ca16ec19e60e5af915dd9b

## Diagram

.csdlc/prepared/issues/624/diagram.mmd

Digest: 1d1d6f64c905049e3f871bc9958698aa6164ec27cb7af78e1464f2f760ee2660

## Stop Conditions

- A required row needs live mutation and lacks explicit operator authorization
- A redacted public receipt cannot be written without sensitive details
- The #497/#634 sidecar denominator conflicts with #624 issue scope
- Validation finds missing dispositions or secret-like material
- An actionable review finding remains unresolved

## Handoff

Proceed only after doctor readiness.
