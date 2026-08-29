# Structured Planning Prompt

Template: 1.0.0

Issue: 579

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #579 from the late review FAIL, bind a FastWork corrective worktree, repair AWS-F module/proof/validator/state/Spot surfaces, run local non-mutating Terraform and validator proof, obtain fresh exact-head review, publish with closing linkage, then finish only after green remote state.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and approve the corrective design from current main and issue #579.",
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
    "id": "S2",
    "action": "Bind the #579 FastWork execution worktree and preserve dependency/collision truth.",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Repair AWS-F Terraform, validator, runbook, and evidence surfaces without paid cloud mutation.",
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
    "action": "Validate, obtain fresh exact-head review, publish with closing linkage, and finish only when green.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Runtime hosts have no direct public ingress
- Public edge ownership remains #122
- Terminal #489 state remains historical and unmutated
- State backends, locks, accounts, workspaces, and keys remain isolated by runtime component
- Disposable cleanup selectors are exact and cannot target durable/adopted resources
- Evidence is redacted and never includes credential material

## Risks

- Regex-only Terraform scanning could miss multiline or structural ingress
- Documentation could overclaim deferred live proof as completed proof
- State isolation controls could remain advisory if not validated
- Spot exact-instance target attachment can fail after interruption
- Cloud proof requires explicit paid AWS authorization

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/579/design.md

Digest: 461d7a3e45316c5f894eac2f51f5e5e8d6579832407fe1de193a99c5f2c30f81

## Diagram

.csdlc/prepared/issues/579/diagram.mmd

Digest: baa407238c7793086702139955d9f0ecac07d1815121ba96b278f66e2a92e433

## Stop Conditions

- A proposed change creates Route53 public records/zones or ACM certificates in AWS-F-owned runtime modules
- World-open Runtime ingress can pass the validator
- Proof claims live deployment, cleanup, rollback, observability, or artifact wiring beyond evidence
- Reusable runtime state lacks enforced backend/lock/account/workspace/key isolation
- Spot exact-instance attachment is described as production-resilient
- Live AWS proof would require unavailable credentials, paid mutation, or production traffic

## Handoff

Proceed only after doctor readiness.
