# Structured Planning Prompt

Template: 1.0.0

Issue: 179

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Prove normalized v2/v3 safety parity, migrate a bounded v3-only canary cohort with archived snapshots and writer fences, run every known-defect regression, and perform an independently reviewed operator-approved single authority cutover with rollback.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Freeze the complete normalized parity matrix, retained defect register, canary cohort, migration map, rollback criteria, and no-dual-write invariant.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5",
      "AC-8",
      "AC-11"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run shadow parity for cards, lifecycle, PVF, review, both linkage modes, finish, cleanup, and unsupported import fields; disposition every mismatch.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Archive exact v2 snapshots, install durable writer fences, remove canonical v2 indexes, and prove supported v2 rejection before any v3 mutation.",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run the approved v3-only cohort through normal and post-review correction for every card family plus issue #73 denominator recovery.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Execute freeze, final delta, independent exact-head review, explicit operator approval, selector switch, source archival, stable install, and post-cutover audit.",
    "acceptance_ids": [
      "AC-9",
      "AC-10",
      "AC-11"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Retain measured effect/canary/rollback receipts and stop on any mismatch, unsupported field, dual writer, stale review, failed canary, or unapproved selector mutation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Reconcile canary resources and writer fences, retain rollback and authority receipts, remove migration scratch output, and prove exactly one writable generation remains.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue V3-16 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.
- No unsupported completion, legal, production, or release claim
- No mutation outside exact owned paths

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/179/design.md

Digest: e066d29f10ba847d685fe564266ae728403f784184cea1033bff0380a9d5321f

## Diagram

.csdlc/prepared/issues/179/diagram.mmd

Digest: be770d266c729ace1e9b13a9d39891bf1db48058e2641ff04ae18c92c8e991cb

## Stop Conditions

- Any unexplained parity mismatch, unsupported field, dual writer, stale review, failed canary, missing rollback evidence, or unapproved selector mutation.
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
