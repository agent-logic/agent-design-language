# Structured Planning Prompt

Template: 1.0.0

Issue: 153

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Freeze a redacted, denominator-complete corporate asset register before any transfer: reconcile authoritative asset classes, record custody and rollback fields, prove exclusions and redaction fail closed, then hand the immutable inventory to dependent corporate work.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Enumerate the asset classes from the promoted corporate source and reconcile each class to inventoried rows or an approved not-applicable exclusion.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Author the redacted asset register with stable identifiers, current controller role, target owner, custodian, recovery authority, dependency, transfer method, verification method, rollback posture, disposition, and evidence references.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Author the exclusion matrix and inventory-freeze runbook without executing transfers, rotating credentials, or retaining secret/private material.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Produce inventory-export and denominator receipts whose digests and source revision bind the register, exclusions, and runbook.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run the issue validator and negative fixtures for duplicate identifiers, missing custody/recovery fields, unapproved exclusions, and forbidden sensitive keys.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "On any denominator, custody, or redaction failure, stop without changing external control; retain the unresolved row and rollback/no-action disposition for review.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Complete exact-head review of the frozen denominator and publish only the redacted artifacts after all findings are resolved.",
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

- Issue CORP-01 owns only its declared repository paths and named external operation/evidence boundary.
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

.csdlc/prepared/issues/153/design.md

Digest: 88d6d59ad72f7278dbaa5af794214c0c79ea60fc33c7057fda417f9436be9047

## Diagram

.csdlc/prepared/issues/153/diagram.mmd

Digest: 54dd8c804937b881ad0f67a3fa357a0b9ec6b8331814875d8ff130839d5e120f

## Stop Conditions

- A critical asset has unknown ownership or custody
- A secret or private instrument would enter the repository
- The denominator cannot be reconciled with company and founder accounts
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
