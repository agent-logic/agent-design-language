# Structured Planning Prompt

Template: 1.0.0

Issue: 79

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind issue #79, narrow initialized-phase readiness deferral to exact owned deliverables and a temporary issue-owned test harness, add child-shaped positive and negative proof, validate, independently review, and publish one ready unmerged PR.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Characterize current child blockers and encode exact admission predicates.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement the initialized-phase deferred target and temporary harness admission logic.",
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
    "action": "Add child-shaped positive fixtures and independent negative mutations preserving false-readiness behavior.",
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
    "action": "Run focused tests and strict Clippy, resolve exact-head review, and publish the ready PR.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- Admission exceptions apply only before bind
- Every deferred path is exact, issue-owned, and an explicit deliverable
- Validation remains fail-closed and selects a real issue-owned target
- Production modules require either an existing owned route or the bounded temporary harness route
- Implemented and reviewed phases never accept missing targets or missing proof
- Prose is not interpreted as a filesystem path

## Risks

- A broad deferral admits arbitrary missing modules
- A prose deliverable accidentally satisfies path validation
- A lane appears proving while selecting zero tests
- The temporary harness exception leaks beyond initialized admission
- A child-shaped positive fixture omits a real constraint from #5866, #5871, or #5872

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/79/design.md

Digest: f1681d69a252c91e7c41835000fd59d22d728531ebb467b641e6b6320ad8b7c3

## Diagram

.csdlc/prepared/issues/79/diagram.mmd

Digest: 331af2d8209bef560e5f45eada5572a36f9129dc1e03d7af411edbfe62b120b5

## Stop Conditions

- The fix requires weakening post-bind or implemented validation
- The fix requires child product implementation or shared production registration
- A declared target cannot be distinguished from prose deterministically
- Exact-head independent review finds an unresolved correctness or scope issue

## Handoff

Proceed only after doctor readiness.
