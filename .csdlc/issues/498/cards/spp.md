# Structured Planning Prompt

Template: 1.0.0

Issue: 498

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Initialize CORP-D, keep execution gated on CORP-C, classify diligence blockers and private boundaries, prepare the repository-safe acceptance packet, and validate the result.

## Plan

Revision 3

## Steps

[
  {
    "id": "STEP-498-001",
    "action": "Verify CORP-A, CORP-B, and CORP-C are closed, merged, and ancestral before execution proceeds.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "STEP-498-002",
    "action": "Inventory repository-safe diligence inputs and private evidence boundaries.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "STEP-498-003",
    "action": "Classify blockers and residual risks with dispositions.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "STEP-498-004",
    "action": "Write and validate the corporate diligence acceptance packet and truthful output/review records.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Typed C-SDLC v2 remains lifecycle authority.
- CORP-D execution waits for CORP-C terminal truth.
- Private legal advice, private diligence material, credentials, tokens, and account secrets are never printed or committed.
- Repository artifacts distinguish public acceptance evidence from private or deferred evidence.

## Risks

- CORP-D could be started prematurely while CORP-C remains open.
- Diligence acceptance could overstate private or non-public evidence.
- Blockers could be hidden instead of dispositioned.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/498/design.md

Digest: e6de247c0a6747942f2561f103125e244554a8b017db354f9b86b7bc948dc286

## Diagram

.csdlc/prepared/issues/498/diagram.mmd

Digest: ddd06dc5d45ee33926fa6e4b707a49a70d2016ca2269aa8f44a3bcce01cd63c2

## Stop Conditions

- CORP-C #497 is not closed, merged, and ancestral.
- A blocker or risk lacks a disposition.
- A required artifact would expose private legal advice, private diligence material, credentials, tokens, or account secrets.

## Handoff

Proceed only after doctor readiness.
