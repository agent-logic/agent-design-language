# Structured Planning Prompt

Template: 1.0.0

Issue: 154

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Execute only counsel-approved IP instruments outside the public repository, verify corporate acceptance and company custody, and publish digest-bound redacted chain-of-title receipts that reveal no private instrument content.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Verify terminal CORP-01 evidence, reconcile the asset and contributor schedules, and block execution until qualified counsel approves the final instrument set.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Execute the approved private instruments through the authorized private channel and record each required party or an explicit blocking disposition.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Obtain corporate acceptance from the identified authority and verify private originals reside in company-controlled custody outside the repository.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Generate the redacted chain-of-title index, corporate acceptance receipt, and custody readback using role identifiers and cryptographic digests only.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Recompute instrument and schedule digests from the private custody boundary and prove every schedule row has an executed, excluded, or blocking disposition.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Run redaction and forbidden-field checks; if counsel approval, authority, party coverage, custody, or redaction fails, stop and publish no completion claim.",
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
    "action": "Complete exact-head review of the public receipt package without importing signatures, addresses, privileged advice, or private instruments.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue CORP-02 owns only its declared repository paths and named external operation/evidence boundary.
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

.csdlc/prepared/issues/154/design.md

Digest: 565698465771f319302d30f3724370f3dedc9e427e589044799ec366c3c7bcee

## Diagram

.csdlc/prepared/issues/154/diagram.mmd

Digest: 3abd106d9b41673ebd4635fa448584d57deabca62224528bea570ae99bd3b12c

## Stop Conditions

- Counsel has not approved the final form
- Corporate acceptance authority is unclear
- An asset or contributor lacks a disposition
- Private material would be committed
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
