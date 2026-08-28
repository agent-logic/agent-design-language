# Structured Planning Prompt

Template: 1.0.0

Issue: 487

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #487, obtain design approval, wait for #486 terminal ancestry, bind a FastWork worktree, implement the AWS audit/security baseline surfaces and redacted proof, run focused static/readback validation, obtain exact-head review, publish, and finish when green.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and approve the AWS-D audit/security design.",
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
    "action": "After #486 terminal ancestry, bind the #487 FastWork execution worktree.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement account-foundation audit/security Terraform, runbook, redaction, and proof surfaces.",
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
    "action": "Validate, obtain fresh exact-head review, publish with closing linkage, and finish when green.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Audit/security baseline does not re-own #486 backend resources
- Security findings always have an explicit owner and destination
- Retention, encryption, and cost posture are machine-checkable
- Retained evidence never includes credentials or token material

## Risks

- CloudTrail, Config, or detection services can create uncontrolled recurring cost if scope is too broad
- Detection findings can be enabled without a real owner or destination
- Retained AWS evidence can accidentally expose sensitive identifiers or credentials
- Account-foundation Terraform can drift into AWS-E adoption or AWS-F runtime-platform scope

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/487/design.md

Digest: 4c5997596cbdef6333c5e42fecd935b6698ee0c810f0b96af70c2586b75e76f0

## Diagram

.csdlc/prepared/issues/487/diagram.mmd

Digest: c01e4f717001a00c40f7904ac7d82a4b71c51676ce72ad9fb56f242609d1f02f

## Stop Conditions

- Audit gaps remain unexplained
- Findings lack ownership
- Logging creates uncontrolled cost
- Credentials would enter evidence
- #486 is not terminal and ancestral before bind/implementation

## Handoff

Proceed only after doctor readiness.
