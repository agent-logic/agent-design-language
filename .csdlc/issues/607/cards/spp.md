# Structured Planning Prompt

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Separate slow preparation from fast launch, seal two persistent AZ-local data volumes, attach and validate them through Terraform, remove all cold startup work, then prove repeated warm launch timing and full Runtime/GPU behavior.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Implement disjoint storage and compute Terraform ownership plus exact AMI KMS AZ and destroy-plan guards.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Build the complete immutable launch and qualification artifact closure and dm-verity sealed warm volumes.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement no-install activation service and qualification receipts stage budgets and deterministic negative tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-12"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Bind three single-use authorizations and aggregate cost then prepare and launch twice proving timing full behavior and exact residue.",
    "acceptance_ids": [
      "AC-5",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11",
      "AC-12"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Obtain fresh exact-head review fix every actionable finding and publish through typed lifecycle authority.",
    "acceptance_ids": [
      "AC-12"
    ],
    "status": "pending"
  }
]

## Invariants

- Normal launch has no compilation installation Git or model download
- Warm-volume identity is immutable and fail-closed
- Both nodes retain SSH with one key
- Ollama remains private
- SSM remains recovery-only
- Compute cleanup never deletes warm volumes
- Timing denominators remain explicit
- Aggregate AWS spend does not exceed USD 20

## Risks

- Base AMI lacks a required facility and tempts launch-time package repair
- EBS attachment or filesystem checks consume the startup budget
- GPU model page-in exceeds the 30-second guest target
- Terraform destroy deletes or strands a warm volume
- Timing receipts omit EC2 provisioning or use incomparable clocks
- Preparation artifact identity drifts from reviewed source

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/607/design.md

Digest: 8274edf9cc3606a40f6a284bf17c2c1ab75bd50b2ab614038dc0fe9c7f4c7d29

## Diagram

.csdlc/prepared/issues/607/diagram.mmd

Digest: 075917071aa994387a05fef10743b0e62d1c5a2dfdbf91b3b1258327402ca953

## Stop Conditions

- A normal launch requires compilation package installation Git or model download
- Warm volume identity or cleanup ownership is ambiguous
- Ollama becomes public
- Either node lacks SSH
- A paid operation would exceed USD 20
- Timing cannot distinguish Terraform EC2 and guest denominators
- Fresh review reports an unresolved actionable finding

## Handoff

Proceed only after doctor readiness.
