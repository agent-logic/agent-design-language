# Structured Planning Prompt

Template: 1.0.0

Issue: 217

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Freeze the retained-proof contract, restore the exact source packet, implement a merge-safe validator with adversarial fixtures, correct typed proof commands, obtain fresh reviews, and publish without merging.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Independently review and approve the retained-proof design, exact artifact inventory, and ancestry/equivalence authority boundary.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Restore the exact ten-file packet and implement the confined retained-proof validator without production changes.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused positive/adversarial validation and update typed VPP/SOR final-head proof truth.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve independent exact-head review, publish the visible qualified PR, and shepherd checks without merging.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- The retained packet bytes and recorded source-run identities remain immutable
- Every referenced artifact is repository-relative and confined beneath issue evidence
- Current protected-source equality is mandatory in both ancestry and equivalence modes
- Squash/rebase changes may alter commit identity but cannot excuse protected-byte drift
- No production runtime behavior changes

## Risks

- An ancestry-only rule would fail after squash even when source bytes are identical
- A digest-only rule could omit a protected path or fail to bind runner provenance
- Restoring packet files without a final-head validator would repeat the false-green contract
- Editing #209 rendered cards directly could corrupt immutable lifecycle history
- A preparation PR could be mistaken for implementation completion if its non-claims are unclear

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/217/design.md

Digest: 8a11ce10e9cc06b3c5bbbeec53378c9f831e813c30052988aaefc50be66d371f

## Diagram

.csdlc/prepared/issues/217/diagram.mmd

Digest: 296185d2d19ae92d242f357b658f462130b520af52c5dfc2a1eedbcca33a87f0

## Stop Conditions

- Any required artifact differs from the independently validated run 31453636709 packet
- The validator cannot reject protected-source drift without changing production code
- Typed card routes cannot express the truthful final-head command without rewriting history
- Independent design or implementation review finds unresolved P1/P2 defects
- Any action would merge the PR or mutate main outside typed bootstrap/binding

## Handoff

Proceed only after doctor readiness.
