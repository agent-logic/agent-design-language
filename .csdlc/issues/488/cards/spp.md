# Structured Planning Prompt

Template: 1.0.0

Issue: 488

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #488 after #487 terminal ancestry, approve a design centered on an adoption-register denominator, bind a FastWork worktree, implement register/readback/validator/evidence surfaces, run focused proof, obtain exact-head review, publish with closing linkage, and finish when green.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and approve the AWS-E adoption-register design from current main containing #487.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Bind the #488 FastWork execution worktree and preserve dependency/collision truth.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement the adoption register, redacted readback/evidence packets, and issue-owned validation scripts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Validate, obtain fresh exact-head review, publish with closing linkage, and finish when green.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Each admitted durable resource has one and only one management authority
- Register rows distinguish retain, import, replace, retire-later, ephemeral, and frozen-unknown
- Website resources and retained historical evidence are preserved unless a later issue explicitly owns their migration
- CloudFormation retirement remains #496 authority and cannot be implied by #488
- Runtime module implementation remains #489/#495 authority and cannot be implied by #488
- Evidence is redacted and never includes credential material

## Risks

- A broad inventory denominator can accidentally classify unrelated or website-owned resources
- Dual management can occur if live resources are both imported and separately recreated
- Cleanup evidence can become destructive if non-use and retention recovery are not exact
- Cloud account state can drift between readback and review
- Sensitive ARNs, account identifiers, or logs may leak if evidence is not redacted

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/488/design.md

Digest: 7769d45d6158532926412015e20c086e5fae8247732864efae35c562f0ccac5c

## Diagram

.csdlc/prepared/issues/488/diagram.mmd

Digest: 0214eb9b13c71b88970156f1a9f22813a93d173b3241d28b422261f60aee0947

## Stop Conditions

- A resource may belong to website or retained evidence
- Dual management is possible
- Deletion authority is missing
- Live AWS readback would require unavailable credentials or mutation
- A proposed change would implement #489, #495, or #496 scope

## Handoff

Proceed only after doctor readiness.
