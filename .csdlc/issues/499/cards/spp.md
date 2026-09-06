# Structured Planning Prompt

Template: 1.0.0

Issue: 499

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Reconcile the issue boundary, produce the single RUST-01 deliverable, validate it, and obtain exact-head review.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Reconcile dependencies and freeze the exact issue-local denominator.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Produce the bounded primary deliverable without widening authority.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run every planned PVF lane and retain bounded redacted evidence.",
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
    "action": "Obtain exact-head review and prepare a truthful publication handoff.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- Issue completion is exactly one behavior-preserving resilience owner-boundary refactor; module extraction and test relocation are internal steps and line movement is not a separate result.
- Baseline API, positive, negative, fault, trace, retry, timeout, cancellation, formatting, Clippy, and exact diff checks pass while the tracked validation-impact denominator is reduced or truthfully unchanged.
- No secret or private material in Git

## Risks

- Behavior changes are required
- Ownership becomes more ambiguous
- Tests are weakened or merely moved
- Refactoring expands into unrelated Rust surfaces

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/499/design.md

Digest: 13d76e45f18eadaa35845d36f0d8a8e34a4ef27675d7d2720f5a3c22d5b3f62a

## Diagram

.csdlc/prepared/issues/499/diagram.mmd

Digest: 38c362aef42ad49d59732236139c96a15952738ff2c24c22745632eb34ef310a

## Stop Conditions

- Behavior changes are required
- Ownership becomes more ambiguous
- Tests are weakened or merely moved
- Refactoring expands into unrelated Rust surfaces

## Handoff

Proceed only after doctor readiness.
