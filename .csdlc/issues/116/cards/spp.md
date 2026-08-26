# Structured Planning Prompt

Template: 1.0.0

Issue: 116

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap and review the #116 design, bind a dedicated FastWork worktree, implement the bounded Runtime and Observatory attention inbox seams, prove focused behavior, obtain exact review, publish, watch CI, and finish.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and approve the #116 design packet from current main and terminal dependency truth.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Bind #116 to a dedicated FastWork worktree and implement runtime attention request lifecycle and queue policy.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement Observatory inbox projections, filters, deep links, notification preference state, and explicit intervention outcomes.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused proof, strict hygiene, exact review, publication, CI, and terminal finish.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Attention requests are policy-visible and identity-bound
- Operator responses are communication outcomes unless paired with an explicit authority action
- Queue behavior is bounded, deterministic, and restart-safe
- #117 and downstream proof children remain out of scope

## Risks

- Alert flood if rate/dedup/grouping logic is weak
- Authority confusion if reply/ack/refuse states imply approval
- Spoofing risk if source identity is caller-controlled
- Duplicate notifications after restart/reconnect

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/116/design.md

Digest: fff3d704ae4a24ffc15b6c2d2c1bf29d3fadfd68d539904bacbb273c5caa5351

## Diagram

.csdlc/prepared/issues/116/diagram.mmd

Digest: d3c7c13d82a8af2b3c5803e0f67faba2a8fddacc2362074f0310c274332b2162

## Stop Conditions

- Any missing terminal dependency gate
- Any need to change #270/#271/#276/#277/#278 semantics
- Any scope pull into #117/#279/#280/#281/#282
- Any failed proof, actionable review finding, red CI, or publication guard failure

## Handoff

Proceed only after doctor readiness.
