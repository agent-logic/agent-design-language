# Structured Planning Prompt

Template: 1.0.0

Issue: 309

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Pin the exact baseline and rollback source; generate the complete disposition inventory; execute dead-code, superseded-compatibility, and only safe Runtime-v2 deletion bands as separate reversible commits; prove behavior, continuity, platform, accounting, and rollback; then obtain exact-head review and publish the truthful achieved reduction.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Verify #308 terminal ancestry/cleanup, pin the e926e3bc baseline, generate the complete file/blob/line/reference inventory, and validate one disposition per row.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Execute Band A dead/unreachable deletion as one commit and prove focused behavior plus exact rollback/reapply.",
    "acceptance_ids": [
      "AC-3",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Execute Band B superseded-compatibility deletion as one commit with replacement/authority parity and rollback proof.",
    "acceptance_ids": [
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Classify Runtime v2 consumers, execute only already-proven safe contraction, and record retained authority or migrate-then-delete boundaries.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run combined macOS/Linux, clean-install, behavior, artifact, trace, continuity, accounting, scope, and rollback proof; obtain exact-head review and publish.",
    "acceptance_ids": [
      "AC-5",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Supported behavior and authority do not change
- Every deletion is inventory-backed and independently reversible
- No active Runtime v2 or #414 continuity authority is removed without exact replacement proof
- No reduction credit for movement exclusion gating or compatibility copying
- Every retained path has an active owner or timed exception
- Rollback never overwrites unrelated later work

## Risks

- Dynamic CLI and artifact references may not appear in Rust module reachability
- Large legacy tests can encode current compatibility contracts
- Runtime v2 remains partially consumed by current production paths
- Broad deletions can create platform-specific or clean-install regressions
- Later independently owned edits can make naive path restoration unsafe

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/309/design.md

Digest: 9b509902cb90ee78e72276403ef9e443432b3630c29ca460d50286a923c990e4

## Diagram

.csdlc/prepared/issues/309/diagram.mmd

Digest: 53609f25156db701cde3505b92446c763a5abff23a915730ac20a99e9061d9a4

## Stop Conditions

- Any baseline row or active reference lacks exactly one accountable disposition
- A candidate deletion retains a live build command runtime documentation test or artifact consumer
- Replacement parity or authority ownership is incomplete
- #414 continuity or current Runtime behavior regresses
- Rollback cannot restore exact blobs without overwriting later independent work
- The next reduction requires migration or refactoring owned outside #309

## Handoff

Proceed only after doctor readiness.
