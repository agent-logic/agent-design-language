# Structured Planning Prompt

Template: 1.0.0

Issue: 277

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Build the #277 continuity layer on top of the #276 journal and #270 trusted acknowledgement protocol, then prove restart/idempotency/replay/receipt behavior without absorbing downstream history/UI work.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap #277 from live issue truth and prove #276/#270 terminal dependency gates.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Bind the dedicated FastWork branch/worktree after fresh design review approval.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-8",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Implement Runtime continuity primitives for watermarks, idempotency, replay decisions, ambiguous dispatch, and receipts.",
    "acceptance_ids": [
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
    "action": "Run focused proof, strict relevant Rust validation, lifecycle validation, fresh exact review, publication, CI, and finish.",
    "acceptance_ids": [
      "AC-10"
    ],
    "status": "completed"
  }
]

## Invariants

- #277 consumes but does not redefine #276 and #270
- Duplicate completed attempts cannot execute twice after restart
- Ambiguous dispatch remains ambiguous until a trusted receipt or reconciliation resolves it
- Definite pre-dispatch failures remain retryable
- Receipts survive restart through the #276 durable journal
- #278/#114/#115/UI/API/cloud work remains out of scope

## Risks

- Replay logic can accidentally grow into #278 public history integration
- Acknowledgement-watermark storage can accidentally redefine #270 trust if provenance boundaries are blurred
- Idempotency state must distinguish ambiguous and pre-dispatch retryable cases precisely

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/277/design.md

Digest: 21c173f854c1a447865801b8992fed4658993f12c7f19ef68953d81317161b2e

## Diagram

.csdlc/prepared/issues/277/diagram.mmd

Digest: 54db214860da4ea91991631e5167956fd9f70b3c40649c44229d80b356424f4b

## Stop Conditions

- #276 or #270 terminal cache or ancestry validation fails
- Bind target is not the dedicated #277 FastWork worktree
- Design/readiness review reports unresolved actionable findings
- Scope expands into #278, #114 parent, #115, #270 trust redefinition, #276 foundation rewrite, API/UI/Observatory, browser, cloud, or provider transcript scraping

## Handoff

Proceed only after doctor readiness.
