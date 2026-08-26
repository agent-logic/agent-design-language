# Structured Planning Prompt

Template: 1.0.0

Issue: 513

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind #513 in FastWork, add a runtime authority topology document, machine-readable manifest, and executable validator, run focused proof, obtain fresh exact-head review, publish, shepherd green checks, and merge through typed C-SDLC v2.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bind #513 and establish issue-local lifecycle state.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Author the Runtime v2/v3 authority topology and manifest.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement executable validation for denominator, reverse references, compatibility, rollback, migration, and Runtime v4 exclusion.",
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
    "action": "Run focused validation and finalize implemented truth.",
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
    "id": "S5",
    "action": "Obtain fresh exact-head review, publish, shepherd, merge, and finish.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Every source root has exactly one declared owner.
- Reverse references are either owned, compatibility-only, documentation-only, or evidence-only.
- Migration and rollback are explicit dry-run contracts.
- Runtime v4 does not appear as a source owner or migration target.
- #483 remains untouched.

## Risks

- Reverse-reference census can drift as sibling PRs land.
- A documentation-only topology could overclaim behavior if not backed by executable checks.
- Runtime v4 mentions in historical planning docs could be mistaken for current authority.
- Source ownership could become ambiguous if Runtime v2 imports Runtime v3 or vice versa.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/513/design.md

Digest: db6c89dfbece755904892e6322d9666cf958ff46280a8829fa6450ceecdcc692

## Diagram

.csdlc/prepared/issues/513/diagram.mmd

Digest: e627b2cd99b14ac2e4dc9d16adf09d51b0830434cce80c48cad7365109750b04

## Stop Conditions

- A supported consumer is unclassified.
- Either runtime silently acquires the other runtime's authority.
- Runtime v4 scope is required.
- The issue branch or worktree identity drifts.
- Fresh review finds unresolved actionable issues.
- Required checks are red.

## Handoff

Proceed only after doctor readiness.
