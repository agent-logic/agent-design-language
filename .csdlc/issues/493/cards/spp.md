# Structured Planning Prompt

Template: 1.0.0

Issue: 493

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #493 after #492 terminal ancestry, approve a design centered on private GCP platform foundation, bind a FastWork worktree, implement Terraform/docs/evidence/validation surfaces, run focused proof, obtain exact-head review, publish with closing linkage, and finish when green.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and approve the GCP-D private platform design from current main containing #492.",
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
    "action": "Bind the #493 FastWork execution worktree and preserve dependency/collision truth.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement private platform Terraform, operator runbook, redacted proof packet, and issue-owned validation script.",
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

- No public external IP or broad public ingress is introduced
- Operator access uses IAP and OS Login posture, not static checked-in keys
- Human identity and workload identity are separate
- State, artifacts, models, continuity evidence, and logs have distinct owner surfaces
- Disposable workload cleanup selectors are deterministic and issue-owned
- Evidence is redacted and never includes credential material

## Risks

- A firewall or NAT shortcut could accidentally create public exposure
- Broad IAM can collapse human and workload identity boundaries
- Shared storage ownership can hide provenance or retention failures
- Cleanup selectors can miss residual resources
- Live GCP proof can leak credential/account details if evidence is not redacted

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/493/design.md

Digest: b407bc4d9dec406148b8d63f040f34cc4b84bacdb8ee347a2f6c938b78962103

## Diagram

.csdlc/prepared/issues/493/diagram.mmd

Digest: c45538f51fb071826d23c7467a96b353c344c647abc1a3e8c031c0399a118a4d

## Stop Conditions

- Public route address or ingress appears
- Required IAM is broad or key-based
- Cleanup selectors are incomplete
- A proposed proof would read or retain credential material
- A proposed change would implement GPU qualification, production traffic, Shared VPC, Observatory, Unity, XCL-01, or AWS scope

## Handoff

Proceed only after doctor readiness.
