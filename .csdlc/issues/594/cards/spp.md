# Structured Planning Prompt

Template: 1.0.0

Issue: 594

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Define the Terraform archive boundary, add bounded Vector delivery, prove failure isolation locally, then run separately authorized live AWS delivery and retrieval proof.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Implement the private encrypted versioned S3 archive and exact-prefix publisher policy in Terraform.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Add bounded identity-partitioned Vector S3 delivery without changing Runtime readiness.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused configuration, redaction, retry, exhaustion, and Runtime-survival validation.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "After explicit authorization, run live AWS upload, control inspection, retrieval, and cleanup proof.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Runtime health and recovery never depend on S3
- Archive buffering, retry, disk use, and failure reporting are bounded
- Only redacted records and non-sensitive identity appear in S3
- Infrastructure is Terraform-owned and least privilege
- Paid-cloud proof is separately authorized

## Risks

- Unbounded buffering exhausts the host disk
- Retry behavior couples Vector degradation to Runtime startup or shutdown
- Object keys or payloads leak secrets or machine-local data
- IAM or bucket policy is broader than the exact archive prefix
- Live proof incurs unauthorized spend or leaves resources behind

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/594/design.md

Digest: 4ac3cc1666ee202af1481487b36a95cb723f2732eed7793c208fb4fd0dc7ef42

## Diagram

.csdlc/prepared/issues/594/diagram.mmd

Digest: 66de3337b41d7f1d7c4138d327ac51bf31b082a76a7f31bf7f276b7d9b47291a

## Stop Conditions

- Runtime readiness becomes dependent on Vector or S3
- A secret or unredacted payload enters archive output
- Buffer or retry bounds cannot be proved
- CloudFormation or manual console state would be required
- Live AWS action lacks explicit authorization or business-account identity proof
- Validation selects zero tests

## Handoff

Proceed only after doctor readiness.
