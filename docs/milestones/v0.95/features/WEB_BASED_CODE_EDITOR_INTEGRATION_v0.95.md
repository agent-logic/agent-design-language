# v0.95 Feature: Web-Based Code Editor Integration

> **Current planning boundary (#1407, 2026-10-31):** Follow the [feature acceptance crosswalk](../../../planning/v095-reconciliation/FEATURE_CROSSWALK_1407.md) and [decision register](../../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md). Runtime v4 is v0.93.3; tooling/foundations v0.93.4; GHB v0.93.5; remaining governance v0.93.6; affect/humour v0.93.7. Private implementations remain in their owner repositories. Earlier milestone names, source paths and proposed operations in retained source inputs are historical, not current execution authority. No feature is accepted or scope changed by this candidate document.

## Status

Forward-planning feature contract for `v0.95`.

## Purpose

Define the required editor/operator integration baseline for the MVP using
HTML-based in-repo editor surfaces, independently of whether Zed is ultimately
shipped.

## Source Inputs

- `docs/milestones/v0.85/HTA_EDITOR_PLANNING.md`
- `docs/milestones/v0.85/ideas/V095_MVP_BOUNDARY.md`
- `docs/planning/ADL_FEATURE_LIST.md`

## Scope

This feature should establish:

- a credible HTML-based editor capability for authoring, execution, and review
- explicit coupling to the validated control-plane lifecycle rather than a
  parallel state model
- the minimum required editor surface even if Zed remains optional

## Non-goals

- silent substitution of host-editor integration for the required web/editor
  baseline
- bypassing control-plane validation or workflow truth
- claiming polished host-specific integration as the only acceptable MVP path

## Completion Target

`v0.95`
