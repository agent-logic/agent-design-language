# Structured Planning Prompt

Template: 1.0.0

Issue: 122

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Hold beyond v0.92; after terminal distributed Runtime proof and separate operator authorization, verify the business profile, freeze the bounded public contract, implement issue-owned non-EC2 infrastructure, prove exact public behavior and rollback, and complete exact-head review.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Wait for terminal distributed Runtime proof and separate operator AWS authorization; confirm #122 remains non-gating for #83 and #111-#117.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Verify the approved business profile and freeze exact hostnames, resource ownership, budget, ingress, security, rollback, and cleanup contracts without EC2, Spot, or CodeBuild.",
    "acceptance_ids": [
      "AC-2",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement only issue-owned Route53, ACM, S3, CloudFront, and approved non-EC2 Runtime ingress targets.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run exact local policy and authorized live browser, HTTPS, WSS, revision, ownership, rollback, cleanup, and negative forbidden-compute proof.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Resolve exact-head security and operations review and hand off without production marketing launch unless separately authorized.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- #122 remains deferred beyond v0.92 and non-gating for #83 and #111-#117
- No AWS action occurs without separate operator authorization and verified business-account authority
- No EC2, Spot, or CodeBuild resource is created or operated
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
- The design requires EC2, Spot, CodeBuild, or an unapproved service
- Exact origin, authentication, redaction, rate-limit, ownership, rollback, or cleanup behavior cannot fail closed
- Any action would gate or mutate #83 or #111-#117

## Handoff

Proceed only after doctor readiness.
