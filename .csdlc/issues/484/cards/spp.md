# Structured Planning Prompt

Template: 1.0.0

Issue: 484

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind #484 from current main, run only read-only Agent Logic AWS profile discovery, normalize the inventory under owned docs/evidence paths, validate redaction and no-mutation posture, obtain fresh review, and publish one closing PR.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Bind issue #484 from current main in a FastWork worktree.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Verify approved business account identity and enabled-region denominator using read-only commands.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run read-only resource discovery and classify every discovered resource by owner/lifecycle disposition.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Validate redaction, no-mutation command posture, diff hygiene, and lifecycle truth.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Obtain fresh exact-head review, fix findings, publish, shepherd green checks, finish, and leave cleanup async.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- No AWS mutation commands are run.
- No credential material is captured.
- Every discovered resource receives an explicit disposition.
- Unknown ownership is frozen-unknown, never silently disposable.
- Downstream AWS-B/AWS-C scope remains separate.

## Risks

- The active AWS profile could point at the wrong account.
- Some resource families may require service-specific list calls beyond tag APIs.
- Readbacks could expose sensitive identifiers that need redaction before commit.
- Inventory could overclaim completeness if the service denominator is not explicit.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/484/design.md

Digest: 771576b6ba818c90b5bfed83067c0b52fdae423f49f72351f48053ab23217f4f

## Diagram

.csdlc/prepared/issues/484/diagram.mmd

Digest: be6f1a129f272200c04e3541cfbaca08f7da4f3ae4d3b1820e0bced631a8c9cb

## Stop Conditions

- Approved business AWS account identity is ambiguous.
- A discovery command would mutate AWS state.
- A discovered resource cannot be classified or frozen-unknown.
- Evidence would require credential disclosure.
- Fresh review finds unresolved actionable issues.

## Handoff

Proceed only after doctor readiness.
