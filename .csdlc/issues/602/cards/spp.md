# Structured Planning Prompt

Template: 1.0.0

Issue: 602

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add a small authenticated admission contract, atomic store, synchronized roster update, direct csmctl client, focused tests, then deploy and demonstrate on Wuji.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Implement the authenticated Runtime agent lifecycle API, atomic store, synchronized roster, two-phase dehydration, portable bundle, removal, and rehydration.",
    "acceptance_ids": [
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
    "id": "S2",
    "action": "Implement csmctl agent add --config with a strict runtime-loaded declaration containing stable multi-part name, display name, office, provider, model, endpoint, and Runtime connection reference; keep lifecycle artifacts identity-preserving.",
    "acceptance_ids": [
      "AC-1",
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
    "action": "Update API contracts and run focused config parsing, identity separation, lifecycle, persistence, artifact-tamper, roster, formatting, and lint proof.",
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
    "action": "Deploy the exact candidate on Wuji and prove config-driven add, duplicate handling, freeze-dry migration, rehydration, roster identity, Shepherd preservation, and restart recovery with gemma4:e4b-mlx.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run bounded exact-head review, fix actionable findings, and publish the stacked #602 PR without merging or modifying #589.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- No Runtime init mutation or restart for admission
- Shepherd cannot be overwritten or removed
- Authorization and provider verification precede mutation
- Durable and live admission truth never split
- Same declaration is idempotent and conflicting identity is rejected
- No credential appears in output logs errors or persisted state

## Risks

- Durable file and live roster could diverge on partial failure
- An endpoint could be syntactically valid but expose the wrong model
- Concurrent duplicate admissions could race
- Stacked #589 base could drift before merge

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/602/design.md

Digest: 8ec2477720ecd6e79add81268ac981bfbe427822a88219b463d19a4c13ee2235

## Diagram

.csdlc/prepared/issues/602/diagram.mmd

Digest: db4dc6f1b58fbe972497399037f5a3cb17293843c4c7d8e16c45c37b709b2463

## Stop Conditions

- #589 exact implementation is unavailable or incompatible
- Admission requires init-file mutation or Runtime restart
- The write token cannot be used without exposing credential material
- Durable atomicity or Shepherd preservation cannot be proven
- Live Wuji proof would require downloading or loading unrelated models

## Handoff

Proceed only after doctor readiness.
