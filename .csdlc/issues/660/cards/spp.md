# Structured Planning Prompt

Template: 1.0.0

Issue: 660

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Contain the live public exposure first, correct the hidden preview route, retain exact evidence, run the issue-local validator and diff hygiene, then obtain exact-head review before publication.

## Plan

Revision 2

## Steps

[
  {
    "id": "contain-public-route",
    "action": "Hide the exact unintended public podcast keys with S3 delete markers and invalidate CloudFront.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "repair-hidden-preview",
    "action": "Deploy the current The Cognitive Stack page under /_preview/podcast/ with noindex and without public feed/media links.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "retain-proof",
    "action": "Record exact manifest, delete marker IDs, invalidation IDs, live HTTP checks, and negative authority evidence.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "validate-review",
    "action": "Run the issue-local validator, diff hygiene, and exact-head review before publication.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- No provider submission without future explicit operator authority.
- No private archive deletion.
- No credential or private receipt retention.
- Public launch remains blocked until separately approved.
- The hidden preview route remains noindex.

## Risks

- The public route may already have been indexed before emergency containment.
- Separate slash and index S3 keys can drift if only one is updated.
- Preview links can accidentally reintroduce public feed or media fetches.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/660/design.md

Digest: cc7338ad86109c91e2d3d058e86547cbfd72727db264441dc6ed990e652cd052

## Diagram

.csdlc/prepared/issues/660/diagram.mmd

Digest: 4f3e09153e9d1e896e59e15f39f34f90bce217c8699b20cd449cd0551773d6ef

## Stop Conditions

- Any unexpected non-podcast S3 delete target appears.
- Any public route still serves current podcast content after invalidation.
- Any provider-directory or credential action would be required.
- Any proof requires retaining private account material.

## Handoff

Proceed only after doctor readiness.
