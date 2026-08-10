# Structured Planning Prompt

Template: 1.0.0

Issue: 156

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Move service administration, billing, MFA, recovery, vault, and break-glass custody to company roles, prove recovery without sole founder dependency, and retain only redacted role/outcome evidence.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Load CORP-01's critical-service denominator and map each service to company administrator, billing owner, MFA method class, recovery route, vault record identifier, and break-glass posture.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Establish or verify company-controlled administration, billing, secure MFA, recovery, and vault custody without recording credential material.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Exercise each recovery route from an authorized company context and prove it does not rely solely on a founder phone, email, card, or device.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Exercise the bounded break-glass procedure, verify audit logging and role separation, then restore routine-access posture.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Produce service readback, recovery, and break-glass receipts with provider/service identifiers, role names, timestamps, source revision, and artifact digests only.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Run denominator, personal-dependency, role-separation, and sensitive-field negative checks; stop on any sole personal factor or ambiguous billing/custody.",
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
    "action": "Complete exact-head review and publish the redacted custody matrix and operational recovery runbook after reverting any temporary recovery-test changes.",
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

- Issue CORP-04 owns only its declared repository paths and named external operation/evidence boundary.
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

.csdlc/prepared/issues/156/design.md

Digest: 5278f7197c066e6ff57422e1d8e825008a2c371f1c4c02852f7a43dcb60a2fec

## Diagram

.csdlc/prepared/issues/156/diagram.mmd

Digest: ddf11e4f8a62d7fc71b72ab9d355669d3e0ac2316a99748d0d802376f0e1158d

## Stop Conditions

- A critical service depends on one personal recovery factor
- Company billing cannot be verified
- Credential handling would cross the repository boundary
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
