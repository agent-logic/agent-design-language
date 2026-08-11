# Structured Planning Prompt

Template: 1.0.0

Issue: 208

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After PR #197 merges, bind #208, implement the private production continuity listener/client and real checkpoint operations, prove thirty-six cases, resolve review through subagents, and publish a ready unmerged PR before #204.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "After #191 merges ancestrally, bind #208 and freeze exact config, TLS identity, live continuity, journal, stream, and root interfaces.",
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
    "action": "Implement the loopback mTLS listener, opaque Guardian client, real checkpoint export, isolated staging/validation, resume/discard, replay and persistence.",
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
    "action": "Prove exact thirty-six-case behavior, crash/retry/path/bounds/redaction/public-surface denial, strict Clippy, and merge-safe receipt truth.",
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
    "action": "Resolve fresh exact-head review through a subagent, publish a ready PR closing #208, shepherd hosted CI, and wait for operator review and merge authorization.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- The public Runtime/Observatory service never routes internal continuity operations
- Only the exact configured Guardian/kernel generation pair can use an internal continuity session
- Every returned bundle or possession receipt derives from the live kernel coordinator and exact verified bytes
- Source remains quiesced until exact resume or downstream fencing; target remains isolated until downstream activation
- No caller path, mock, synthetic snapshot, cached bool, or receipt creates distributed authority

## Risks

- An internal listener could accidentally bind publicly or share public trust
- Large bundle transfer could allocate without bounds or accept path traversal
- Cancellation or reply loss could disagree with a committed kernel effect
- A retained replay namespace could survive Guardian/kernel rotation
- The issue could drift into #204 distributed policy or final cloud integration

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/208/design.md

Digest: 3b568fa2f3d299fbe5be3b65250f7b0191eb7c36f55c6d460fa3f9f002b844a3

## Diagram

.csdlc/prepared/issues/208/diagram.mmd

Digest: 08144a9e755cf338211914696c5e12a6aedc2afa6d47681e3df79d2f6fe19ca1

## Stop Conditions

- Issue #191 / PR #197 is not externally reviewed, merged, and ancestral
- The live continuity coordinator cannot support safe isolated stage/validate without a narrower kernel prerequisite
- The internal listener cannot be proven loopback-only and identity-distinct
- Any normal-build caller can inject a mock, caller path, synthetic checkpoint, or raw authority
- Implementation expands into #204 policy, public Observatory/API, models, AWS, or live qualification
- Any focused proof or independent review has an unresolved actionable finding

## Handoff

Proceed only after doctor readiness.
