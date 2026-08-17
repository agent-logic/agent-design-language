# Structured Planning Prompt

Template: 1.0.0

Issue: 194

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #194 lifecycle around the implemented private AWS qualification harness, retain truthful partial live proof, then complete the missing serial hybrid recovery behavior before review/publication.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Validate fail-closed private AWS preflight, template invariants, dry-run denial/fault paths, and cleanup discovery.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run and retain redacted live private two-voter mesh and single-GPU model-health proofs with zero cleanup.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Extend the harness to execute the serial hybrid Wuji/AWS recovery proof, including snapshot recovery, partition, continuity, heal/demotion, and one-of-three halt.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Record truthful SOR/SRP, obtain independent exact-head review, publish PR, and shepherd required checks without merge.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- No Internet gateway, NAT gateway, EIP, public subnet route, public IP, or public Runtime/model endpoint in the private qualification stack
- All launched resources are issue/run tagged with TTL and cleanup_required
- Cleanup/assert-zero runs after success, failure, and interruption before any new AWS run
- Receipts distinguish proved profiles from non-claims and never combine quota-split proofs into an overclaim

## Risks

- AWS quota prevents simultaneous two-GPU voter model health
- Serial hybrid Wuji recovery behavior may require additional runtime harness integration beyond current AWS substrate
- Raw live evidence contains local AWS identifiers and must not be committed accidentally
- CloudFormation endpoint/instance deletion tails can extend wall time after compute has stopped

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/194/design.md

Digest: 10b16da040fe5f38afea07ec21af3193b8970293736e16a7eb9cb5a32bf2ecef

## Diagram

.csdlc/prepared/issues/194/diagram.mmd

Digest: ddea3d69e78bab2a5acbfaa48154d77bc77150b498871ce557c59bb7f2924379

## Stop Conditions

- Wrong AWS account/profile or unapproved role identity
- Any public IP/public route/public ingress on Runtime/model resources
- Any nonzero resource after cleanup/assert-zero
- Missing current independent exact-head review before publication
- Attempt to claim serial hybrid recovery without live proof

## Handoff

Proceed only after doctor readiness.
