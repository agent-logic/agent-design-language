# Structured Planning Prompt

Template: 1.0.0

Issue: 143

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind issue 143, inventory accepted ADRs and landed evidence, correct the stale plan, author one Proposed or Deferred document per reserved number, build an evidence/disposition index and focused validator, then resolve fresh exact-head review before publishing.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Bind issue 143 and inventory accepted ADR numbers, candidate conventions, and landed v0.92 evidence",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Correct the v0.92 ADR plan and author ADR 0059 through 0071 as source-grounded Proposed or Deferred candidates",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Create the reviewer index and focused ADR packet validator",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused validation and fresh exact-head independent review; resolve every actionable finding",
    "acceptance_ids": [
      "AC-9",
      "AC-10"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Publish a documentation-only PR that closes issue 143 without accepting any ADR",
    "acceptance_ids": [
      "AC-10"
    ],
    "status": "completed"
  }
]

## Invariants

- Candidate status remains Proposed or Deferred
- Accepted ADR files remain unchanged
- Every architectural claim is bounded by landed evidence and explicit non-claims
- Planned feature contracts are not misrepresented as executable proof
- Cross-polis operational migration remains deferred beyond v0.92

## Risks

- A candidate may overstate behavior beyond its cited proof
- Forward-looking WP-18A or WP-18B plans may be mistaken for landed evidence
- A new ADR may duplicate or silently supersede an accepted record
- Cross-polis language may imply production migration
- A candidate number may collide with current registry truth

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/143/design.md

Digest: 0894b6fc8938c94fc13c44f9258cbfe7168995f5550907f83ebe274d4fb9c39e

## Diagram

.csdlc/prepared/issues/143/diagram.mmd

Digest: 452985f0a9f483748e9e7a20058fff994d332b9215a524b891b41d17ecf8c125

## Stop Conditions

- Accepted ADR registry or candidate numbering is ambiguous
- A required decision cannot be grounded in repository evidence
- A candidate requires an unlanded implementation claim rather than Deferred status
- Any proposed text implies production cross-polis migration, personhood, citizenship, or accepted authority
- Focused validation or exact-head review reports an unresolved actionable finding

## Handoff

Proceed only after doctor readiness.
