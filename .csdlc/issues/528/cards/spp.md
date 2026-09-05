# Structured Planning Prompt

Template: 1.0.0

Issue: 528

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #528 from the live issue, approve a design centered on shared Gemini codec reuse plus a distinct Vertex AI transport, bind a FastWork worktree, implement provider/config/auth/receipt/test/doc changes, run focused deterministic provider proof, obtain exact-head review, publish with Closes #528, and finish when green.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and approve the Vertex AI Gemini provider transport design from current issue and #514 dependency truth.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-7",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Bind #528 in a FastWork worktree and inventory existing provider substrate/Gemini Developer API behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Implement shared Gemini codec reuse and the distinct Vertex AI transport/config/auth boundary.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Add deterministic provider, streaming, UTS tool, error, cancellation, and redaction proof plus operator docs/live-smoke packet.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Record truthful SOR/SRP state, obtain fresh exact-head review, publish with Closes #528, and finish when required checks are green.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "completed"
  }
]

## Invariants

- Existing Gemini Developer API semantics stay compatible
- Vertex AI is distinguishable from Gemini Developer API in configuration and receipts
- No credential material, access tokens, prompt bodies, or raw sensitive responses are logged or committed
- Local deterministic proof does not claim live Vertex AI provider success
- Provider error classification remains common across transports where applicable
- UTS tool names and arguments are preserved exactly through normalization

## Risks

- Codec extraction could regress the existing Gemini Developer API route
- Vertex AI endpoint construction could accidentally hard-code project, location, or publisher defaults
- Auth implementation could leak access tokens or credential paths in logs/receipts
- Tool-call argument normalization could diverge between Gemini transports
- Live Vertex behavior may differ from deterministic fixtures and requires separately governed proof

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/528/design.md

Digest: 3c3bddc9b94c407bbd6e70a2a8e29f15df162ee39493f09787361d7a3007d5f4

## Diagram

.csdlc/prepared/issues/528/diagram.mmd

Digest: e6676e2e88750ea4db2327dfe7187c3e0b43806b6cdc1f17ddbf9959a91d3179

## Stop Conditions

- Implementation requires reading or exposing credential contents
- A live paid Vertex AI call would be needed without explicit operator authorization
- A change rewrites unrelated providers or replaces existing Gemini Developer API behavior without focused compatibility proof
- The transport cannot preserve UTS tool names and arguments through deterministic tests
- The work attempts to mutate GCP IAM/API/billing/organization policy
- The issue cannot be bound under /Volumes/FastWork/adl-worktrees through typed v2

## Handoff

Proceed only after doctor readiness.
