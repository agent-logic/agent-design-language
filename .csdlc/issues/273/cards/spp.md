# Structured Planning Prompt

Template: 1.0.0

Issue: 273

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap and freshly review the disjoint design, approve/doctor/bind only after PASS, implement the Shepherd-only module/test and serialized registration, prove deterministic fenced lifecycle behavior, review exact head, publish, shepherd required CI, and finish terminally before releasing #274 shared registration.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Validate live issue, exact prerequisite terminal caches and ancestry, typed cards, disjoint ownership, and obtain fresh design review before approval, doctor, and bind.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement only the Shepherd state machine, redacted receipt and projection, bounded persistence behavior, feature-gated focused test, and additive registration.",
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
    "action": "Run feature-enabled focused test and strict Clippy, ordinary build, exact scope and diff proof, and record immutable evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "After required coverage aggregate exposes the unmapped new module, add only the exact coverage-impact classifier, runtime-only internal-test-fixtures route, classifier and runner regressions, focused 97.01 percent coverage, and impact preflight.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Obtain a distinct fresh exact-head review of the repaired mapping and lifecycle truth, repair findings, republish PR #339, shepherd renewed required CI, finish, and validate terminal ancestry before releasing #274.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- At most one Shepherd is eligible
- No Pending or unreconciled state is eligible
- Every grant binds exact terminal #272 foundation state
- Retry cannot mint or revive eligibility
- Projection leaks no raw authority
- #273 and #274 product files remain disjoint

## Risks

- Shared distributed/mod.rs registration can collide with #274
- Stale foundation or permit state could be replayed
- Replacement could transiently expose two owners
- Projection could leak authority material

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/273/design.md

Digest: 5a9aef0fc8b80d891d58f1e59183d044ca97794130cba410574d906629d08e40

## Diagram

.csdlc/prepared/issues/273/diagram.mmd

Digest: 0445aa425312ed6389ea2990dd2dd521d8575e53c6f65a719eaffb043eb30c9e

## Stop Conditions

- Any predecessor cache is noncanonical or nonancestral
- Fresh design review finds unresolved file/module overlap
- Implementation requires any foundation change beyond the bounded verified-cut API or another undeclared path
- #274 or another writer touches the same owned file
- Any proof, review, CI, terminal-cache, or ancestry gate fails

## Handoff

Proceed only after doctor readiness.
