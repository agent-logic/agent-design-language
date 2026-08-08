# Structured Planning Prompt

Template: 1.0.0

Issue: 63

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Freeze the narrow operation and authorization contract, implement through the existing typed AST/render/atomic-commit pipeline, prove accepted correction and all adjacent fail-closed states, validate narrowly, resolve exact-head review, and publish a qualified closing PR.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Confirm the dedicated operation, exact phase/card/truth guards, and old/new audit contract.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the semantic operation and store authorization through the existing typed AST, render, validation, and atomic commit path.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused real-editor regressions for accepted correction, exact audit, stale input, adjacent phases/cards, recovery, and projection drift.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused validation and exact-head review, resolve findings, publish, shepherd, and finish truthfully.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
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

Digest: 8b7cd544d1f7d76ed33fccd340b5fd35e329eaeaa26b62f06f7cfb2f473e34ea

## Diagram

.csdlc/prepared/issues/63/diagram.mmd

Digest: ad780692c3e66851a847102b1d81a23270af76c10dae4f62fa308a05bcd82633

## Stop Conditions

- The route requires direct Markdown or raw record mutation
- Review/publication truth must be cleared by this editor rather than typed recovery
- Scope expands beyond the two production modules and focused existing regression surface
- Old/new scope and reason cannot be retained atomically

## Handoff

Proceed only after doctor readiness.
