---
issue_card_schema: adl.issue.v1
wp: "TAIL-06.22"
slug: "active-boot-paths-control-plane-guidance"
title: "[v0.92.1][TAIL-06.22][architecture] Publish active boot paths and retire stale control-plane guidance"
labels:
  - "track:roadmap"
issue_number: 837
generated_at: "2026-09-10T23:00:00Z"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.1"
required_outcome_type:
  - "architecture"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/837"
canonical_files: []
demo_required: false
demo_names: []
issue_graph_notes:
  - "Child of #522 resolving TPR-005."
pr_start:
  enabled: true
  slug: "active-boot-paths-control-plane-guidance"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-10T23:00:00Z

# Structured Task Prompt

## Summary

Distinguish active native C-SDLC v3 lifecycle authority from retained rollback code and ADL Runtime/product binaries, then repair stale current guidance.

## Goal

Publish and enforce one source-backed active boot path per subsystem and retire stale ordinary C-SDLC v2 guidance.

## Required Outcome

Active docs, CLI help, selector, installed layout, and tests agree that native v3 is the sole ordinary lifecycle path while product/runtime and rollback-only surfaces are correctly classified.

## Deliverables

Per-subsystem boot-path table, source evidence, retained-generation classification, current-guidance repairs, focused guard, and review evidence.

## Acceptance Criteria

Exactly one ordinary lifecycle path; ADL and Runtime correctly classified; retained v2 has no ordinary route; docs/help/selector/layout/tests agree; focused validator and exact-head review pass.

## Repo Inputs

AGENTS.md; docs/csdlc-v3/CURRENT_AUTHORITY.md; csdlc-v3/operator/authority-selector.json; installed owner-binary manifests; active tooling and Runtime documentation.

## Dependencies

#505 / PR #591 cutover, current AGENTS.md, and docs/csdlc-v3/CURRENT_AUTHORITY.md.

## Target Files / Surfaces

Current lifecycle/boot-path documentation, focused source-backed inventory, and issue-837 enforcement artifacts only.

## Validation Plan

Derive the boot-path table from source and selectors, repair only stale current guidance, and run a focused validator with negative ambiguity fixtures.

## Demo Expectations

Deterministic source-backed inventory and negative stale-guidance fixture; no live demo required.

## Non-goals

No deletion of retained rollback source, authority change, Runtime/lifecycle combination, or unrelated docs cleanup.

## Issue-Graph Notes

The sibling remediation denominator remains outside #837.

## Notes

Historical evidence and explicit rollback docs must remain allowed; broad scans must not confuse product binaries with lifecycle generations.

## Tooling Notes

Native C-SDLC v3 only; bound FastWork worktree; focused docs/contract validation.
