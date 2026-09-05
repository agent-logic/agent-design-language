# Structured Planning Prompt

Template: 1.0.0

Issue: 495

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #495 after #488 and #493 terminal ancestry, approve a design centered on the exact #194/#268 denominator and provider-neutral contract, bind a FastWork worktree, implement Terraform/docs/evidence/validation surfaces, run focused static and authorized-live proof where available, obtain exact-head review, publish with closing linkage, and finish when green.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and approve the XCL-01 design from current main containing #488 and #493 terminal truth.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Bind the #495 FastWork execution worktree and preserve dependency/collision truth.",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Implement the portable contract plus explicit AWS/GCP Terraform, runbooks, proof packet, and issue-owned validator.",
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
    "action": "Validate, obtain fresh exact-head review, publish with closing linkage, and finish when green.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- The portable contract stays provider-neutral while preserving explicit provider-specific identity, IAM, networking, and state differences
- No Terraform implementation silently substitutes one provider's security model for another
- CloudFormation remains rollback authority until AWS-G accepts retirement
- No production cutover, DNS/public exposure, or GPU qualification is introduced
- Paid/live apply and destroy proof stays separately authorized and redacted
- Evidence never includes credential material

## Risks

- A false abstraction could hide AWS/GCP security differences
- Existing CloudFormation behavior could be incompletely mapped
- Terraform parity could be claimed from static files without adequate plan/deploy proof
- Cleanup proof could miss residual resources
- Credential-bearing live proof could leak account or key material

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/495/design.md

Digest: 400a5443b0af9aa3e3232fcda2a2d6412366b7d137a84b2940c46066e875e1f0

## Diagram

.csdlc/prepared/issues/495/diagram.mmd

Digest: 983842e336b78e648ed0e338fcf4f13e79c207a9471843037a1945bd00a8a594

## Stop Conditions

- A #194 or #268 template behavior is unmapped
- Provider-specific security or identity differences are hidden behind a false abstraction
- Either provider cleanup proof fails or is unrepresented
- Paid authorization is absent for apply/destroy proof
- A proposed change would implement AWS-G retirement, GCP-E GPU smoke, DRT-D qualification, production cutover, or credential-bearing proof outside the approved lane

## Handoff

Proceed only after doctor readiness.
