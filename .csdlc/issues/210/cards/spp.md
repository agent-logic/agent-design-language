# Structured Planning Prompt

Template: 1.0.0

Issue: 210

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After all four dependencies merge, bind #210, implement typed bounded resumable transfer and incremental verification, prove forty-five exact cases, resolve review through subagents, and publish a ready unmerged PR before #204.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "After dependencies merge ancestrally, bind #210 and freeze exact token, route, bundle-handle, frame, prefix, verifier and cleanup interfaces.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the typed session, bounded stream, durable prefix/retry, incremental verification, #208 handle integration and exact abort cleanup.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Prove forty-five exact cases, crash/restart/reply-loss/bounds/cleanup/redaction, strict Clippy and merge-safe receipt truth.",
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
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve fresh exact-head review through a subagent, publish a ready PR closing #210, shepherd hosted CI, and wait for operator review and merge authorization.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Only the exact source may read and exact target may stage the token-bound bundle
- A transfer session cannot send Raft, generic, public or unknown messages and those sessions cannot send transfer frames
- The durable accepted prefix advances only after exact bounded bytes are durable and verified
- Target content remains isolated and non-authoritative until #204 later activates it
- No caller path, mock, synthetic snapshot, retained boolean, local absence claim, or transfer receipt creates ownership

## Risks

- Generic transport dispatch could bypass typed session authority
- Whole-bundle buffering or frame queues could exceed memory bounds
- Acknowledgment before durable prefix could corrupt resume
- Cancellation or reply loss could duplicate accepted bytes or false-complete
- The issue could drift into migration/fence/serving/cloud policy

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/210/design.md

Digest: 5c74e5c6075be23562dd8f663819175da7e681fa35b457f9cda80fd535635ad9

## Diagram

.csdlc/prepared/issues/210/diagram.mmd

Digest: 2fe46767191362859600fe156d4946dd2b9d9a4723fa0ded0bc1d952049bce6a

## Stop Conditions

- Any dependency is not externally reviewed, merged, and ancestral
- Merged #191 cannot host a closed typed non-Raft service without reopening generic message authority
- Merged #208 does not expose opaque bounded bundle reader and isolated-stage writer handles
- Incremental verification cannot bind the exact signed catalog without weakening snapshot authority
- Implementation expands into #204 policy, models, AWS, or live qualification
- Any focused proof or independent review has an unresolved actionable finding

## Handoff

Proceed only after doctor readiness.
