# v0.95 Feature: Zed Integration

> **Current planning boundary (#1407, 2026-10-31):** Follow the [feature acceptance crosswalk](../../../planning/v095-reconciliation/FEATURE_CROSSWALK_1407.md) and [decision register](../../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md). Runtime v4 is v0.93.3; tooling/foundations v0.93.4; GHB v0.93.5; remaining governance v0.93.6; affect/humour v0.93.7. Private implementations remain in their owner repositories. Earlier milestone names, source paths and proposed operations in retained source inputs are historical, not current execution authority. No feature is accepted or scope changed by this candidate document.

## Status

Forward-planning feature contract for `v0.95`.

## Purpose

Define the decision boundary for Zed as an ADL editor/operator host surface:
either ship a bounded credible integration or explicitly keep it out of the
must-have MVP set.

## Source Inputs

- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/milestones/v0.85/ideas/V095_MVP_BOUNDARY.md`
- `docs/milestones/v0.85/HTA_EDITOR_PLANNING.md`
- `docs/milestones/v0.85/features/ROAD_TO_v0.95.md`

## Scope

This feature should establish:

- the bounded Zed integration decision and success criteria
- compatibility with the validated control-plane lifecycle rather than a
  parallel workflow
- explicit relationship to the required HTML/editor-capability baseline
- a truth-preserving rule for shipping, deferring, or dropping Zed from MVP

## Non-goals

- silently promoting Zed into the must-have set
- bypassing control-plane validation
- treating host-editor preference as architectural proof

## Completion Target

`v0.95`
