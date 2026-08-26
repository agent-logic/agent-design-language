# #260 design: distributed Runtime caller migration to governed authority adapters

Status: pre-bind preparation only. This design is intentionally execution-ready but bind/implementation gated until #259 is terminal, reconciled, and ancestral.

## Context

#260 is the third #203 child. #258 sealed the authority-store boundary. #259 owns governed transport certificate and authority flow binding. #260 owns the remaining distributed Runtime caller migration to the governed adapter facade after those gates.

## Goal

Migrate migration, recovery, placement, projection, resource-weather, snapshot-catalog, capability-advertisement, and related distributed Runtime callers so they use the governed authority adapter facade instead of raw certificate, lease, or fencing store paths.

## Boundaries

- Do not implement or revise the #258 authority-store boundary.
- Do not implement governed transport certificate/authority binding owned by #259.
- Do not publish or close parent #203.
- Do not begin bind or production source changes until #259 is terminal, reconciled, and ancestral.

## Intended implementation shape after #259 terminal

1. Resync a new #260 FastWork worktree to current main after #259 merge.
2. Inventory distributed Runtime caller sites outside #259 transport scope.
3. Replace direct raw-store access at those caller sites with the governed adapter facade established by #258/#259.
4. Preserve deterministic retry/reconcile behavior for migration and recovery paths.
5. Keep validation focused on the caller surfaces changed by #260, plus compile proof that no migrated caller retains raw-store bypass.

## Acceptance mapping

This design uses the same acceptance IDs as the generated STP so reviewers and validators read one canonical ID map:

- AC-1: #260 cards state that bind/source implementation is blocked until #259 terminal, reconciled, and ancestral.
- AC-2: #260 scope is limited to non-transport distributed Runtime caller migration to the governed adapter facade.
- AC-3: #260 non-goals preserve #258 authority-store boundary, #259 transport binding, and #203 parent integration ownership.
- AC-4: Validation lanes are focused on the touched distributed Runtime caller surfaces and do not claim parent #203 integration.
- AC-5: Independent pre-bind design/card review finds no actionable readiness issue before typed design approval.

Post-bind implementation will still prove that migrated caller surfaces no longer use raw certificate, lease, or fencing store access except through explicit governed adapter or test-fixture authority; that proof is scoped under AC-2 through AC-4.

## Validation plan

Use focused Runtime lanes over the touched caller surfaces. Expected starting commands after bind:

- `cargo check --manifest-path adl-runtime/Cargo.toml`
- focused `cargo test --manifest-path adl-runtime/Cargo.toml --test distributed_projection`
- focused `cargo test --manifest-path adl-runtime/Cargo.toml --test distributed_resource_weather`
- focused `cargo test --manifest-path adl-runtime/Cargo.toml --test distributed_discovery`
- focused `cargo test --manifest-path adl-runtime/Cargo.toml --test distributed_placement`

Selectors may be refined after #259 lands and the exact changed caller inventory is known. A no-run selector is non-proving unless paired with compile evidence and explicit owner rationale.

## Review focus

The reviewer should check that this packet is execution-ready but still fail-closed before #259 terminal; that child ownership boundaries are preserved; and that the validation plan is focused without claiming parent #203 integration.
