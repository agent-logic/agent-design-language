# Structured Planning Prompt

Template: 1.0.0

Issue: 522

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After #521 merges, census every internal/external finding, preserve deduplication provenance, route each to a reviewed fix or explicit deferral, prove every disposition, and validate zero unresolved release blockers.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Verify #521 reviewed merge and ingest every raw #520/#521 finding.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Deduplicate only identical root causes while preserving all source IDs and disagreements.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Route every accepted finding to a bounded fix or explicit owned deferral.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Verify exact-head review and proving validation for fixes and full metadata for deferrals.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Validate census parity, provenance, dispositions, and zero unresolved release blockers.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Every accepted finding is accounted for exactly once
- Every substantive fix is independently reviewable
- Every deferral is explicit and owned
- Release blockers cannot remain unresolved

## Risks

- Deduplication could hide distinct failures
- Green checks could be mistaken for semantic proof
- A deferral could omit release consequence
- Late fixes could stale review evidence

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/522/design.md

Digest: 4692a9546dd100ef9ab4dba437640f0b8300315e2dc45be653623f4c39b6c857

## Diagram

.csdlc/prepared/issues/522/diagram.mmd

Digest: 7c6c137dd6eccc07ef3e3844d07d8305f1d6720d876ddfaf7ebb9a7ed82fc49f

## Stop Conditions

- #521 is not reviewed and merged
- Finding denominator mismatch
- A fix lacks exact-head review or meaningful proof
- A release blocker remains unresolved

## Handoff

Proceed only after doctor readiness.
