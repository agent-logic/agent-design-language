# Structured Planning Prompt

Template: 1.0.0

Issue: 149

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Coordinate the eight corporate-transfer children through their legal, custody, control, infrastructure, and diligence gates while preserving child ownership and deriving lane closure from exact terminal records. Fail closed on missing terminal, producer, ancestry, dependency, cleanup, or path-ownership proof.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Freeze the exact CORP-01 through CORP-08 ledger from the milestone wave; verify all six cards, approved designs, owned paths, dependency edges, and stop conditions before authorizing any child.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Sequence CORP-01 first; release CORP-02, CORP-03, and CORP-04 only after its terminal proof; then gate CORP-05 and CORP-06, CORP-07, and finally CORP-08 while recording every handoff without editing child product paths.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "For each terminal child, recompute card and index digests, verify the exact terminal revision and dependency ancestry, inventory redacted producer artifacts, and stop and route findings on any mismatch or incomplete custody proof.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Prove the umbrella changed only its coordination surfaces, synthesize the eight-child diligence ledger, obtain independent exact-head review, and publish closeout only when every child is terminal.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- The umbrella may coordinate and synthesize but cannot modify child-owned product paths.
- Children retain exclusive implementation and review ownership.
- No unsupported completion, legal, production, or release claim
- No mutation outside exact owned paths

## Risks

- Umbrella scope could absorb child work
- A stale status could start a child early

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/149/design.md

Digest: 0761a79e6b2170ca001322d1e3fd6f6e7f2406977868b41aae7d513d85b03ad6

## Diagram

.csdlc/prepared/issues/149/diagram.mmd

Digest: c181b01623ee38123e43049e765f3e8b8ec664096c84fdf9133728ef0f83e589

## Stop Conditions

- A child lacks complete readiness
- A dependency or serialization gate is ambiguous
- Coordination would require a product-path edit
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
