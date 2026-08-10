# Structured Planning Prompt

Template: 1.0.0

Issue: 90

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Initialize and approve the narrow migration contract, bind issue #90, add the typed request and command with exact topology/origin/CAS/cleanliness guards, prove reviewed publication compatibility and all negative cases, review exact head, and publish one closing PR.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Define the versioned request/report and authorization rules for absent code_repository migration.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Expose the operation through csdlc-issue and retain atomic audit and unchanged review truth.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused positive, negative, idempotency, crash-safety, and reviewed-publication regression coverage.",
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
    "id": "S4",
    "action": "Update operator recovery documentation, validate, obtain exact-head review, and publish a closing PR.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- The issue repository identity never changes
- The adopted code repository equals every effective GitHub origin identity
- The registered branch and canonical worktree match the record and invocation context
- No dirty worktree or stale record can migrate
- Review scope and reviewed revision remain byte-for-byte unchanged
- Publication authorization is not granted by migration

## Risks

- A broad recovery operation could become an arbitrary repository-retargeting escape hatch
- A metadata migration could accidentally invalidate or silently overstate exact-head review truth
- Checking only one remote URL could miss a divergent fetch or push destination
- An idempotent retry could duplicate audit evidence or mutate generation repeatedly

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/90/design.md

Digest: a57c38f6590ae4963fc4546ee3f254c5c1fdf2b99744b3bdd53cc96f377efc8e

## Diagram

.csdlc/prepared/issues/90/diagram.mmd

Digest: a0ea4577be8c7f5c85d0ca3bef41db206fd8446235c4a4a2d431de9c02f70a62

## Stop Conditions

- The requested repository differs from any effective GitHub origin fetch or push identity
- The issue has no exact registered bound worktree and branch
- The worktree is dirty or the generation/digest is stale
- The record already names a different code_repository
- The implementation requires weakening publication, review, or terminal checks

## Handoff

Proceed only after doctor readiness.
