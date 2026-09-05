# Structured Planning Prompt

Template: 1.0.0

Issue: 578

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind #578, add the profile and narrow Z.ai request-parameter support, update docs/evidence, prove deterministic profile/request/reviewer selection paths, run focused validation, complete independent review, and publish a draft PR if current.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bind issue #578 to the FastWork worktree and preserve #446/#455 non-overlap.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement the GLM-5.3-Flash profile and bounded Z.ai parameter materialization.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add docs/evidence and reviewer-selection proof with credential-gated live smoke semantics.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused validation, complete independent review, fix findings, and publish through typed v2 if ready.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "completed"
  }
]

## Invariants

- Credentials are never serialized into profile or invocation evidence.
- Profile selection remains deterministic and redacted.
- Invalid parameters fail before network dispatch.
- Direct Z.ai route remains distinct from OpenRouter route.
- Issue #578 does not modify #446 or #455 scoped files unless already declared here.

## Risks

- Provider documentation may diverge quickly because GLM-5.3-Flash is newly released.
- Existing Z.ai provider wrapper may not yet support nested `thinking.clear_thinking`.
- Reviewer live smoke may be credential-gated locally.
- A lifecycle/tooling issue may slow the desired single-issue onboarding flow.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/578/design.md

Digest: c2a4d1c268973af755effa609acc511fd726ecb3f6d05755448fae3c693242dd

## Diagram

.csdlc/prepared/issues/578/diagram.mmd

Digest: b38a50d30487f6cca132894d4a6b7ddc5f981b9698964d7a07b49cb8d466de45

## Stop Conditions

- The profile cannot be represented in #514 machinery without hidden ad hoc provider logic.
- Parameter validation cannot fail closed before dispatch.
- The reviewer path cannot select a named provider profile deterministically.
- Lifecycle tooling blocks the issue flow and requires a durable tooling-regression packet.
- Implementation would require touching #446 or #455 scope.

## Handoff

Proceed only after doctor readiness.
