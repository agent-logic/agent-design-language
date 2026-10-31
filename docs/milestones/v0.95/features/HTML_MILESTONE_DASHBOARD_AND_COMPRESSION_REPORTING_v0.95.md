# v0.95 Feature: HTML Milestone Dashboard and Compression Reporting

> **Current planning boundary (#1407, 2026-10-31):** Follow the [feature acceptance crosswalk](../../../planning/v095-reconciliation/FEATURE_CROSSWALK_1407.md) and [decision register](../../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md). Runtime v4 is v0.93.3; tooling/foundations v0.93.4; GHB v0.93.5; remaining governance v0.93.6; affect/humour v0.93.7. Private implementations remain in their owner repositories. Earlier milestone names, source paths and proposed operations in retained source inputs are historical, not current execution authority. No feature is accepted or scope changed by this candidate document.

## Status

Forward-planning feature contract for `v0.95`.

## Purpose

Complete the bounded operator-facing dashboard/compression surface so milestone
status, proof posture, and next-action guidance have a maintained HTML home
instead of remaining a historical `v0.90.4` artifact plus backlog intent.

## Source Inputs

- `docs/tooling/milestone-dashboard/README.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `docs/milestones/v0.85/features/ROAD_TO_v0.95.md`

## Scope

This feature should establish:

- a maintained HTML milestone dashboard path for current milestone packages
- compression reporting tied to milestone truth rather than stale snapshots
- clear operator/read-only posture with no release-authority overclaim
- convergence between dashboard data, milestone package, and review entrypoints

## Non-goals

- a generic product analytics dashboard
- replacing canonical milestone docs as release authority
- real-time operational claims without bounded refresh truth

## Completion Target

`v0.95`
