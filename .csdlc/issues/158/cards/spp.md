# Structured Planning Prompt

Template: 1.0.0

Issue: 158

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Migrate production infrastructure only through the verified Agent Logic business AWS profile, use publicly trusted TLS, prove each service cutover and rollback, and delete every temporary resource with provider readback.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Require terminal CORP-04, verify the permanent agent-logic-admin profile resolves to the approved business account, and freeze the service/resource denominator and rollback owners.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Capture the pre-change DNS, certificate, SES, S3, CloudFront, compute, IAM, monitoring, backup, budget, and account-contact baseline from company authority.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Migrate one bounded service phase at a time, using ACM or another publicly trusted issuer for public TLS and recording exact provider/resource identities without credentials.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "After each phase, run external DNS/TLS/email/storage/CDN/workload/monitoring/backup/budget checks and rehearse the declared rollback before advancing.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Inventory and tag every temporary resource before creation, then delete it after success or failure and retain provider readback proving absence.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Recompute cutover, rollback, and cleanup receipt digests; fail on wrong account, self-signed certificates, incomplete rollback, or residual temporary resources.",
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
    "action": "Complete exact-head review and publish the redacted infrastructure manifest and single-command migration runbook after all phases are clean.",
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

- Issue CORP-06 owns only its declared repository paths and named external operation/evidence boundary.
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

.csdlc/prepared/issues/158/design.md

Digest: 09207f654b2b2b005054c87d646c726f580126d965a397e819ae211d4479d823

## Diagram

.csdlc/prepared/issues/158/diagram.mmd

Digest: 5c1ededc3626d128894cedd84fe85bd5c753c7d61265cb5ddf5f94aca4fdd67a

## Stop Conditions

- AWS resolves to a non-business account
- A certificate is self-signed
- Rollback cannot be rehearsed
- Temporary resources cannot be enumerated or removed
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
