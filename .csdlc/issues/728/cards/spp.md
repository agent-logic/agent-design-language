# Structured Planning Prompt

Template: 1.0.0

Issue: 728

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Prepare the disposable AWS-F proof envelope, verify dependencies and AWS identity, save exact Terraform plans, execute only after explicit mutation authorization, prove target health and external receipt, then destroy and prove zero residue before exact-head review and publication.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Verify dependency terminal ancestry, AWS profile/account/region identity, Terraform root/module revisions, backend/workspace isolation, and route/certificate/input readiness without mutation.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Produce the explicit operator authorization packet naming every mutable selector, deadline, cost ceiling, and reverse-destroy requirement; stop before apply if authorization is absent or mismatched.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run the bounded disposable ALB-origin/private-node deployment, attach target, prove target health and external HTTP receipt tied to the exact instance, and record redacted evidence.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Prove the exact target becomes healthy and that a bounded external request through the operator-approved non-production route returns the expected status plus instance/artifact identity marker.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Record exact source/module revisions, saved-plan digests, state/workspace identities, timing, cost, target-health, HTTP receipt, and production-traffic=false without credentials or sensitive retained values.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Destroy issue-owned disposable resources in reverse order, perform absence readbacks, enumerate retained shared resources, and fail closed on residue.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Keep cleanup trap and deadline enforcement active so failure still routes to destroy and any residue is reported as a release blocker with exact selectors and owner.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S8",
    "action": "Validate issue truth, obtain fresh exact-head review, publish with Closes #728, and route #516 immutable proof reference through its owner.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- No direct public Runtime ingress
- No production traffic
- No retained credential or sensitive evidence values
- Every applied resource is issue-owned or explicitly retained as shared
- Reverse destroy is mandatory before completion
- Exact target identity is tied to the external receipt

## Risks

- Terraform state or workspace ambiguity could mix disposable and retained resources
- ALB or API route could bypass the intended non-production edge boundary
- A failed run could leave residue if cleanup trap/deadline is incomplete
- External receipt could prove only ALB health rather than exact target identity
- Evidence could accidentally retain sensitive AWS or certificate values

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/728/design.md

Digest: e1d163878fdbe95f9bcee6d0898f44bd7950c36307bbb45ae1c3328a85095cf7

## Diagram

.csdlc/prepared/issues/728/diagram.mmd

Digest: 3b7378ae6f9ccf540efac8c84a5476b8e9ee338704f82ef40690ad5df5fb1988

## Stop Conditions

- Active profile/account/region differs from the approved envelope
- Any Terraform plan digest differs from authorization
- VPC, subnet, certificate, route, artifact, backend, or workspace identity is ambiguous
- Plan includes production cutover, Route53/ACM ownership, world-open ingress, or unrelated resources
- Cost ceiling, deadline, cleanup trap, or reverse-destroy selectors are absent
- Target cannot be bound to the response receipt
- Any residue remains unexplained

## Handoff

Proceed only after doctor readiness.
