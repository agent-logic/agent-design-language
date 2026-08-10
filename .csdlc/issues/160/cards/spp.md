# Structured Planning Prompt

Template: 1.0.0

Issue: 160

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Reconcile CORP-01 through CORP-07 into a redacted, digest-verifiable chain-of-title and operational diligence package; preserve private custody; and fail the release gate on every unresolved critical exception.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Require terminal evidence from CORP-02, CORP-03, CORP-05, and CORP-07 and collect the complete CORP-01 through CORP-07 receipt denominator without copying private originals.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Build the critical completion matrix with stable asset/service identifiers and exactly one transferred, retained, excluded, or blocked disposition per row.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Bind required counsel and corporate approvals to exact public receipt digests and private custody identifiers, recording missing authority as blocking.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Build the public redacted diligence index and private custody map so an independent reviewer can recompute every public digest without accessing instruments or credentials.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Reconcile exclusions and critical exceptions; keep every unresolved critical exception release-blocking unless explicit authorized disposition evidence exists.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Run denominator, digest, custody, approval, redaction, exception, and negative private-material checks; stop on any unresolved row or unrecomputable evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Complete independent diligence-readiness and exact-head review, then publish only the redacted index and truthful blocked-or-ready recommendation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue CORP-08 owns only its declared repository paths and named external operation/evidence boundary.
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
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/160/design.md

Digest: cf4ce9b3d8ca4ab1655fa17f19588550781a898a2797776583a2d834cc84213a

## Diagram

.csdlc/prepared/issues/160/diagram.mmd

Digest: 4bb9931b6877c88772e77f790a918042231a87fe080a732a17c20a1b8d7b3305

## Stop Conditions

- A critical schedule row is unresolved
- Private custody cannot be verified
- Counsel or corporate acceptance is missing
- Evidence cannot be independently recomputed
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
