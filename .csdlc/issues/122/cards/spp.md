# Structured Planning Prompt

Template: 1.0.0

Issue: 122

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Implemented after operator authorization: #122 now owns the permanent non-compute CSM public edge plus reusable Terraform roots/modules for disposable Spot EC2 and ALB Runtime-origin smoke proof. Live evidence proved the permanent edge, the external ALB-to-EC2 path, certificate lookup/reuse, teardown, and cleanup of review-identified certificate/DNS residue; publication remains gated on exact-head PASS.

## Plan

Revision 6

## Steps

[
  {
    "id": "S1",
    "action": "Confirm terminal prerequisite truth, separate operator AWS authorization, and #122 non-gating posture for #83 and #111-#117.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Verify the approved business profile and freeze exact hostnames, resource ownership, budget, ingress, security, rollback, and cleanup contracts: permanent edge stays non-compute; disposable Spot/ALB origin smoke is allowed only as bounded proof.",
    "acceptance_ids": [
      "AC-2",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Implement issue-owned Route53, ACM, S3, CloudFront, API Gateway/WSS edge, plus disposable runtime-origin Spot EC2 and ALB Terraform roots/modules for quick create/destroy smoke proof.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run exact local policy proof, Terraform validation, authorized live edge apply, external ALB-to-EC2 receipt proof, teardown proof, ACM/Route53 cleanup proof, and diff hygiene.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Resolve exact-head security and operations review, publish only after PASS, and avoid production marketing launch unless separately authorized.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "completed"
  }
]

## Invariants

- #122 remains non-gating for #83 and #111-#117
- No AWS action occurs without separate operator authorization and verified business-account authority
- Permanent public-edge resources do not create Runtime compute, NAT, GPU, CodeBuild, Kubernetes, or containers
- Disposable Spot EC2 and ALB resources are allowed only for operator-authorized smoke proof and must have teardown/empty-state evidence
- Public reachability never grants write authority or exposes private Runtime or agent state
- Deployed Observatory and Runtime gateway revisions match exactly and rollback remains bounded

## Risks

- Distributed Runtime topology or public-ingress contracts may change before the deferred gate closes
- DNS, certificate, cache, and revision drift can present mixed or stale public surfaces
- Permissive origin, authentication, rate-limit, or redaction policy can widen public authority or disclosure
- Wrong AWS account selection or incomplete ownership tags can make resources unsafe to operate or clean
- A future ingress choice may imply forbidden compute and require operator replanning

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/122/design.md

Digest: 879b89709fc741dabfefcb2931f3ced6af18cc087eaf6bdf4e7ba0b954144652

## Diagram

.csdlc/prepared/issues/122/diagram.mmd

Digest: 370d81d6f8cf0522a0521130f9dd9fd41a41ff14c7a22ef9ac47c4195d532e53

## Stop Conditions

- Distributed Runtime proof is nonterminal, unmerged, non-ancestral, or has unresolved review findings
- Separate operator AWS authorization is absent, ambiguous, or expired
- The approved profile does not resolve to the Agent Logic business account
- Permanent public-edge design requires Runtime compute, NAT, GPU, CodeBuild, Kubernetes, containers, or another unapproved permanent service
- Disposable Spot/ALB proof cannot be created, externally proven, and destroyed with empty-state evidence
- Exact origin, authentication, redaction, rate-limit, ownership, rollback, or cleanup behavior cannot fail closed
- Any action would gate or mutate #83 or #111-#117

## Handoff

Proceed only after doctor readiness.
