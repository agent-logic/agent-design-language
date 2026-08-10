# Structured Planning Prompt

Template: 1.0.0

Issue: 100

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Recover and verify the complete 34-file Medium launch package: 10 canonical articles, 10 editorial reviews, 10 source packets, and 4 series records. Preserve variants and exact provenance, use only the approved Agent Logic company Drive folder 1hCVwqDLetD9Q8tWEDB8e3nTYzvI1Q-rd, retain the original issue folder as obsolete and inaccessible to the company credential, and limit remote overwrites to the three operator-authorized title-only corrections for What is ADL?.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Inventory the destination and search approved Drive, repository history, and registered FastWork worktrees for all ten article titles and variants.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Digest and classify candidates, preserve substantive variants, and select one evidence-backed canonical revision per title.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Upload ten stable canonical drafts without destructive replacement and verify credentialed readability.",
    "acceptance_ids": [
      "AC-4",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Retain the provenance manifest, completeness guard, and focused validation; independent exact-head review remains pending until the repaired head passes.",
    "acceptance_ids": [
      "AC-5",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Every canonical draft is traceable to retained source evidence
- Distinct substantive variants remain preserved
- No synthetic prose is represented as recovered material
- Drive publication and sharing state remain unchanged
- Empty or partial destination state cannot be reported as complete

## Risks

- Title and filename drift can hide candidates
- A newer revision may be less complete than an older reviewed draft
- Drive duplicates may be mistaken for canonical copies
- Upload could accidentally overwrite a distinct draft
- A manifest could claim completeness without live readability proof

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/100/design.md

Digest: d1ce2cd06e2a0913c437f608c1452a6ce74ab0f0aec94814638d30446e4ea0c0

## Diagram

.csdlc/prepared/issues/100/diagram.mmd

Digest: 4dfd6f906d9392e8ce2d4d60fe548f02e6859457143e10a2ee83966f3a376733

## Stop Conditions

- A required credential cannot access the approved Drive corpus
- Canonical selection would require inventing prose or discarding an unresolved substantive variant
- The only available upload path would overwrite, delete, publish, or change sharing
- Any tracked edit appears on primary main
- Execution would touch PR #98

## Handoff

Proceed only after doctor readiness.
