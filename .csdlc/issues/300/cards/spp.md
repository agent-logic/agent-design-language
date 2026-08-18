# Structured Planning Prompt

Template: 1.0.0

Issue: 300

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Approve the test-only design now; after exact #299 terminal and #298/#299 ancestry proof, bind #300, implement one production-backed table-driven integration target, execute the complete failpoint/adversarial matrix and regressions, record exact evidence, and obtain fresh exact-head review.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Repair and independently approve the fully authored test-only design without binding.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "After typed #299 terminal and #298/#299 ancestry proof, bind #300 and inventory exact production failpoints.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Implement the complete deterministic before/after, adversarial, idempotency, evidence, sentinel, cleanup, and later-commit matrix in the new integration target only.",
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
    "action": "Run focused and regression proof, record only observed results, and obtain exact-head review.",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "completed"
  }
]

## Invariants

- Only production APIs and production-generated receipts provide authority
- Every declared boundary has deterministic before/after and restart proof
- No sleep or scheduler luck provides ordering
- Unsafe or ambiguous state and all unrelated evidence are preserved
- Scope remains one new integration test target plus issue-local records

## Risks

- #299 is not terminal
- Production failpoint registry may be incomplete
- Some mount or ownership negatives may require hosted-platform proof
- A required hook could imply a separate production defect

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/300/design.md

Digest: 68001b6bad9eecd3695b504b61b70fe8f8ab757030c05f3ccd966e4f36d282ac

## Diagram

.csdlc/prepared/issues/300/diagram.mmd

Digest: ca70de12c02aebbab46c0c1416c2ad330199d2c6936b77417a7c2b1a594f1d15

## Stop Conditions

- #299 terminal authority is absent or invalid before bind
- #298 or #299 is not ancestral to execution base
- Required proof needs a production or shared-test edit
- Owned path collision
- Fabricated authority or nondeterministic timing
- Unresolved design or exact-head review finding

## Handoff

Proceed only after doctor readiness.
