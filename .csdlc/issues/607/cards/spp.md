# Structured Planning Prompt

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Separate slow preparation from fast launch, seal two persistent AZ-local data volumes, attach and validate them through Terraform, remove all cold startup work, then prove repeated warm launch timing and full Runtime/GPU behavior.

## Plan

Revision 11

## Steps

[
  {
    "id": "S1",
    "action": "Implement disjoint storage and compute Terraform ownership plus exact AMI KMS AZ canonical-plan authorization and destroy guards.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-9",
      "AC-10"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Build the complete immutable launch and qualification closure, seal the two sparse 200 GiB warm volumes, retain completed snapshots of both, and time a temporary snapshot restore.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-9",
      "AC-11"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Implement no-install activation, persistent Guardian readiness, separate qualification receipts, stage budgets, deterministic negative tests, and exact zero-residue queries.",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-12"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Use the current-quota one-L4 g6.xlarge shape for the final distinct authorized launch and prove GPU local readiness at or below 120 seconds, Runtime local readiness at or below 30 seconds, controller service readiness at or below 270 seconds, full behavior, cost, and exact residue.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11",
      "AC-12"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Obtain fresh exact-head review, fix every actionable finding, publish through typed lifecycle authority, finish, and clean.",
    "acceptance_ids": [
      "AC-12"
    ],
    "status": "in_progress"
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
- Under the current four-vCPU G-family quota the enforced timing envelope is GPU local_ready at or below 120 seconds, Runtime local_ready at or below 30 seconds, and controller service_ready at or below 270 seconds; 30-second GPU readiness remains a future optimization rather than an issue gate
- Aggregate AWS spend does not exceed USD 20

## Risks

- Base AMI lacks a required facility and tempts launch-time package repair
- EBS attachment or filesystem checks consume the startup budget
- GPU model page-in exceeds the current-quota 120-second local_ready gate
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

Digest: d0e000997d8886e7b6a0d34d7ad328ff13015c3627bc22dd496de63afee1ada9

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
