# Structured Planning Prompt

Template: 1.0.0

Issue: 5343

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Render and validate all six cards; freeze exact #5344/#5345 dependency gates, selector transaction, fresh-install identity, explicit-v1 rollback window, evidence schema, ownership, protected paths, COTS, budgets, PVF, and no-deferral invariants; obtain bounded preparation review and fix findings; commit and push preparation only; remain blocked until #5344 is merged and typed closed_out; then execute the full reviewed lifecycle without widening scope.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Complete all six typed cards, design, diagram, exact dependency and protected-path gates, COTS, budgets, PVF, no-deferral, transaction and rollback-window invariants, preparation validation, bounded review/fixes, durable commit, and push without execution",
    "acceptance_ids": [
      "AC-1",
      "AC-8",
      "AC-9"
    ],
    "status": "in_progress"
  },
  {
    "id": "S2",
    "action": "Maintain a read-only dependency watch and begin execution only after #5344 and #5345 satisfy live merge, typed closed_out, retained receipt, claim release, ancestry, and exact handoff gates",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Execute one reviewed fresh-install-bound locked compare-and-swap default switch through the #5345 interface and prove failure preservation without implementing selector behavior",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Prove explicit v1 override, rollback-window checkpoints, exact restoration, evidence integrity, budgets, CI, and every negative case; fix all findings",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Obtain exact-revision review, publish through typed v2, shepherd required checks, merge only under authorization, run post-merge proof, close out, retain the terminal receipt, release the claim, and hand accepted evidence to WP-13",
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
    "status": "pending"
  }
]

## Invariants

- No selector mutation begins before #5344 and #5345 are terminal, receipt-backed, claim-free, ancestral, and exact-evidence accepted
- #5343 uses the authoritative #5345 transaction and never edits selector storage or implements a second selector
- The selected executable and installation receipt are exact and fresh-install verified before mutation
- Prior selector bytes, digest, v1 executable, and v1 receipt remain intact throughout the rollback window
- Every failure preserves prior bytes or uses explicit verified rollback through the same compare-and-swap interface
- Rollback or explicit-v1 proof failure blocks publication, closeout, WP-13, and WP-14A
- Evidence is deterministic, redacted, repo-relative, exact-revision bound, and non-secret
- Runtime v2 and incumbent ADL remain untouched rollback targets and no legacy deletion occurs
- All applicable acceptance and PVF lanes complete without deferral before publication

## Risks

- A cutover wrapper can become a second selector or bypass the #5345 transaction
- Fresh-install identity can drift from the executable actually selected
- Stale writers, lock contention, interruption, or partial persistence can damage prior selector state
- A green smoke can hide broken explicit-v1 override or rollback-window checkpoints
- Metadata can be mistaken for accepted #5344 soak/rollback proof
- A compatibility clock can start before all exact verification passes or remain ambiguous
- Evidence can leak host paths, credentials, or unreviewed production claims
- Cutover can be misread as WP-13 deletion authority

## Estimates

{
  "elapsed_seconds": 86400,
  "total_tokens": 240000,
  "validation_seconds": 21600
}

## Design

.csdlc/prepared/issues/5343/design.md

Digest: 255caed6a569e0f57e67a5d01757bd2fb3cfa284543209fed4229244e608435d

## Diagram

.csdlc/prepared/issues/5343/diagram.mmd

Digest: 9c3173bf34cdfcdc240b257dab20ffe3b2646471586e28eaa5bfdb8b9b2e239e

## Stop Conditions

- Any #5344 or #5345 live merge, typed closed_out, retained receipt, claim release, ancestry, or exact evidence predicate is absent or contradictory
- Any intended protected path collides with an active typed claim
- The selector requires direct storage editing, implicit fallback, hidden network, credentials, AWS, Runtime v2 edits, or production-state mutation
- Exact prior selector bytes, digest, v1 executable, or receipt cannot be retained and verified
- The rollback window is missing, expired, ambiguous, or not operator approved
- Evidence is non-deterministic, secret-bearing, host-bound, or incomplete
- Any acceptance or validation item would be deferred, skipped, or replaced with metadata-only proof
- A LoC, module, test, dependency, or duration budget exceeds its limit without exact reviewed variance

## Handoff

Proceed only after doctor readiness.
