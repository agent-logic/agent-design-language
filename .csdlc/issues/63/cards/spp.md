# Structured Planning Prompt

Template: 1.0.0

Issue: 63

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Freeze the narrow operation and authorization contract, implement through the existing typed AST/render/atomic-commit pipeline, prove accepted correction and all adjacent fail-closed states, validate narrowly, resolve exact-head review, and publish a qualified closing PR.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Confirm the dedicated correction operation, exact phase/card/truth guards, implemented-review recovery seam, and old/new audit contract.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Extend typed review recovery for implemented records carrying review truth, then implement SIP scope correction through the existing AST, render, validation, and atomic commit path.",
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
    "action": "Add focused real-editor and recovery regressions for accepted correction, exact audit, stale input, adjacent phases/cards, recovery, and projection drift.",
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
    "id": "S4",
    "action": "Run focused validation and exact-head review, resolve findings, publish, shepherd, and finish truthfully.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- Only SIP declared_scope changes through the new operation
- No correction occurs with stale concurrency input or retained review/publication/readiness truth
- Every correction exposes its actor, reason, previous scope, and replacement scope in canonical audit truth
- Values and rendered Markdown remain generated and cross-card coherent
- Normal planning edits and recovery authority remain unchanged

## Risks

- An overly broad match could authorize adjacent implemented-phase SIP fields
- Phase-only authorization could permit mutation while stale review or publication evidence remains
- Generic audit serialization would omit the previous scope
- A test that calls only a helper could miss real csdlc-edit and validator behavior

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/63/design.md

Digest: 37ddb9edd4c1afef9770c2e6aed4deac1df265eec9269adacfc103808c35c24f

## Diagram

.csdlc/prepared/issues/63/diagram.mmd

Digest: b8f7e332c3b73b50f5124efa9b69be76547178f9ec6bc1c3e751a54185596975

## Stop Conditions

- The route requires direct Markdown or raw record mutation
- Review/publication truth must be cleared by the editor rather than typed csdlc-review recovery
- Scope expands beyond cards.rs, review.rs, store.rs, and the two focused existing regression files
- Old/new scope and reason cannot be retained atomically

## Handoff

Proceed only after doctor readiness.
