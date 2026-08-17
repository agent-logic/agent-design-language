# Structured Planning Prompt

Template: 1.0.0

Issue: 356

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Approve the minimal terminal-#350 accessor design, bind, implement accessors and focused proof, validate, review, publish, and finish before releasing #274.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Review and approve the exact accessor boundary.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Bind and add accessors plus focused tests.",
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
    "action": "Validate, review, publish, and terminally finish.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Only the #350 verifier constructs the projection
- Accessors disclose only already-redacted references and scalar/digest projection values
- No caller input is upgraded into authority

## Risks

- Accessor accidentally exposes raw authority
- Test proves values without proving mismatch denial
- Scope widens into #274

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/356/design.md

Digest: c1f9d45a2937ad2bb69f2a8e1fa8c5dd6bc0d399e84886fc801f196f5919e626

## Diagram

.csdlc/prepared/issues/356/diagram.mmd

Digest: 9f9eefd021002456c2e6a7ddfebabf7032e50efb84494a617b6b53759102b77d

## Stop Conditions

- #350 cache or ancestry is invalid
- A constructor/mutator or raw authority exposure is required
- Any path outside declared scope changes
- Validation or review fails

## Handoff

Proceed only after doctor readiness.
