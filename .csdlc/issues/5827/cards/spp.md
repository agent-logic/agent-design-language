# Structured Planning Prompt

Template: 1.0.0

Issue: 5827

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement and prove WP-10 canonical multi-cycle continuity with predecessor binding, deterministic head derivation, and complete discontinuity negatives.

## Plan

Revision 24

## Steps

[
  {
    "id": "S1",
    "action": "Verify #5826 terminal proof and inspect adl-runtime-kernel continuity.rs and live_continuity.rs before claiming the exact birthday_continuity.rs, lib.rs, tests, fixture, feature, and evidence paths.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement the continuity record, canonical head derivation, two-cycle chain fixtures, and stable rejection reasons.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run deterministic replay, substitution/discontinuity negatives, privacy, and repo-relative portability lanes.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Resolve one bounded exact-head review and publish only with correct base and Closes #5827 linkage.",
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
  }
]

## Invariants

- Each continuity head binds its predecessor, current cycle evidence, and one identity root.
- Restart, wake, restore, snapshot, or copied state is never sufficient alone.
- No raw private state or host-specific path enters review evidence.

## Risks

- Cycle ordering or duplicate acceptance could fork continuity.
- Copied state could be mistaken for lineage continuity.
- Shared lineage paths may collide with adjacent implementation.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5827/design.md

Digest: 26807702a1552dbb27faea8ab64b97dc0e457cd09cc551f80296bf3f6aed637e

## Diagram

.csdlc/prepared/issues/5827/diagram.mmd

Digest: b8e8902ce03c1fd254d2be626f03fb412db939612b74f42de3942fcfd6cdbbb4

## Stop Conditions

- Stop before bind or product edits unless repaired #5826/PR #118 is freshly independently reviewed, fully green, merged, terminally reconciled, and its merge commit is ancestral to the chosen #5827 execution base.
- Stop if the execution base lacks the authoritative Birthday Identity output or if its accepted identity-memory/private-state projection authority cannot be verified from current source and receipt-backed evidence.
- Stop if future csdlc-bind cannot preserve legacy issue identity danielbaustin/agent-design-language#5827 while declaring code_repository agent-logic/agent-design-language.
- Stop if any owned-path collision exists or implementation requires a path outside the exact canonical #5827 owned-path set.
- Stop before validation if the issue-owned birthday_continuity source and exact integration target do not yet exist; once implementation creates them, replace preparation deferrals and run every mandatory lane rather than treating deferral as proof.
- Stop if deterministic replay requires rewriting predecessor Birthday Identity, identity-memory, private-state projection, continuity, or wake evidence.

## Handoff

Proceed only after doctor readiness.
