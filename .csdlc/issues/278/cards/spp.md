# Structured Planning Prompt

Template: 1.0.0

Issue: 278

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Build #278 history read/export/redaction and Observatory transcript restoration on top of terminal #276/#277/#271 surfaces, with per-read Runtime re-authorization and focused stale-cursor/revocation/restart proof.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap #278 from live issue truth and prove #276/#277/#271 terminal dependency gates.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Bind the dedicated FastWork branch/worktree after fresh design review approval.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement Runtime history pagination/search/export/redaction and restart restoration primitives.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Integrate Observatory transcript restoration from Runtime-owned durable history.",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run focused proof, strict relevant Rust validation, lifecycle validation, fresh exact review, publication, CI, and finish.",
    "acceptance_ids": [
      "AC-10"
    ],
    "status": "pending"
  }
]

## Invariants

- #278 consumes but does not redefine #276, #277, #271, or #270
- Every read/search/export/redaction/restore path re-authorizes at Runtime
- Stale cursor, revoked access, stale browser state, and private-memory reads fail closed
- Redaction affects all later read surfaces
- #114 parent and #115 governed rooms remain out of scope

## Risks

- History APIs can accidentally become public/private-memory search unless authorization and scope checks remain explicit
- Redaction can drift between page/search/export/restore surfaces
- Observatory restore can accidentally trust browser cache instead of Runtime durable history

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/278/design.md

Digest: a1cd90456f932404f358e8d02d680eaa67202eef39f7f2189d5f0bc2fb635411

## Diagram

.csdlc/prepared/issues/278/diagram.mmd

Digest: 03e5bf94a4e3d1506211fabd10452b37eb1c58153ab0f195c1fdc90e652eefc7

## Stop Conditions

- #276, #277, or #271 terminal cache or ancestry validation fails
- Bind target is not the dedicated #278 FastWork worktree
- Design/readiness review reports unresolved actionable findings
- Scope expands into #114 parent, #115 room routing, #276/#277 semantic rewrites, #271 authority presentation changes, cloud exposure, provider transcript scraping, or private-memory search

## Handoff

Proceed only after doctor readiness.
