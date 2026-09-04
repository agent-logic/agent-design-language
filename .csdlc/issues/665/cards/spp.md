# Structured Planning Prompt

Template: 1.0.0

Issue: 665

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #665, approve the design, bind an execution worktree, add the typed adoption request/result and bind-owner recovery path, cover positive and negative topology cases plus downstream lifecycle eligibility, document the operator sequence, then exact-head review and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Add the typed adoption request/result contract and bind-owner command route.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement fail-closed topology, HEAD, base ancestry, dirty state, and collision verification without mutating adopted commits.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Record durable adoption evidence and prove adopted issues remain eligible for ordinary finalization, exact-head review, and publication gates.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Document the emergency recovery command sequence, stop conditions, and boundary that emergency product action is not lifecycle authority.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run focused C-SDLC v2 bind/adoption tests, issue-owned validation, and diff hygiene.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Adoption advances only ready to bound
- Adoption never mutates existing product commits or tracked content
- Publication and exact-head review guards remain strict
- Main remains inspection-only for implementation work
- All topology identity is exact and fail-closed

## Risks

- A permissive adoption path could import an unrelated branch
- A stale HEAD could let reviewed evidence drift
- Dirty worktree handling could accidentally hide untracked emergency evidence
- Collision detection could confuse old issue-number branches with current issue ownership
- Downstream publish eligibility could be overclaimed without finalization/review proof

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/665/design.md

Digest: 0d9d58bf4f6c639d9471dc59e735b973aaf4b1479ffd95853d62bed60e8b1834

## Diagram

.csdlc/prepared/issues/665/diagram.mmd

Digest: 37669ced4633e6b67e042a92c409ec56e2c80b2ed2e858d563fda6c67fbb87f9

## Stop Conditions

- The implementation requires weakening review or publication gates
- The operation cannot distinguish current #665 from stale branch names or other issue ownership
- Recovery would require reset, force checkout, rebase, merge, overwrite, or copying through main
- Validation produces zero-test proof for the adoption denominator
- A live cloud mutation or raw GitHub lifecycle write would be required

## Handoff

Proceed only after doctor readiness.
