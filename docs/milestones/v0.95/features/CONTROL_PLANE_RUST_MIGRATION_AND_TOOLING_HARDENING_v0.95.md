# v0.95 Control-plane and tooling residual hardening

## Metadata

Milestone v0.95, candidate WP-08. Documentation reconciliation #1407, 2026-10-31. No implementation or release acceptance.

## Template Rules

Use current owner source and accepted evidence; old public-ADL PR command modules are historical source inputs.

## Purpose

Close only evidenced residual control-plane/tooling integration after existing ADL #1352 and C-SDLC v4, preserving the required MVP hardening boundary.

## Context

Cognitive-SDLC owns native lifecycle, editor, GitHub transport and terminal behavior. Public ADL owns language/contracts and CLI dispatch. Runtime and CodeFriend own their products.

## Coverage / Ownership

Cognitive-SDLC owns lifecycle implementation and native macOS/Linux proof. ADL maintains this cross-milestone acceptance map; affected package owners validate their own residuals.

## Overview

Reconcile completed work before selecting implementation. No duplicate migration, PR-wrapper restoration, shared binary replacement or broad rewrite.

## Design

Inventory the current exact owner revision and residual interpreter/validation coupling. Each admitted slice preserves behavior, names an actual responsibility boundary and smaller focused proof surface, with measured validation burden. File-size reduction alone is not acceptance.

## Execution Flow

Consume #1352 and accepted v0.93.4 CS4/compatibility outcomes → inspect residuals in their current owner → accept a bounded delta or reviewed no-change disposition → reuse unchanged proof → integrate into WP-11.

## Determinism and Constraints

No new lifecycle state engine, repeated binary hashing, duplicated tests/CI, speculative source extraction or automatic authority fallback. Metadata/docs do not prove installed correctness.

## Integration Points

Native C-SDLC six-card SIP/STP/SPP/VPP/SRP/SOR registry, admitted executable/manuals, current owner contracts and exact multi-repo lockset.

## Validation

Docs-only updates use focused contract/link checks. Later code slices prove named behavior/refusal/recovery in the owner lane and compare actual validation cost; broad validation only when required.

## Acceptance Criteria

Evidence accounts for #1352 and C-SDLC v4 first; current source-based residual or explicit no-change disposition; original hardening outcome is preserved without replaying delivered work; exact owner review and meaningful proof for any admitted change.

## Risks

Old source paths and mixed-language inventories are stale. Reinspect before choosing edits; do not claim acceleration or completion from a historical report.

## Future Work

New residuals require their own issue admission; no child implementation, provider/cloud work or cutover begins here.

## Notes

[Original target inventory at the audit baseline](https://github.com/agent-logic/agent-design-language/blob/6b4126980209c7de68b91fd227612e1b161e87b1/docs/milestones/v0.95/features/CONTROL_PLANE_RUST_MIGRATION_AND_TOOLING_HARDENING_v0.95.md) is immutable provenance. [Feature crosswalk](../../../planning/v095-reconciliation/FEATURE_CROSSWALK_1407.md) retains every original feature requirement.
