# v0.94 Feature: Reasoning Graph Baseline

> **Current planning boundary (#1407, 2026-10-31):** Follow the [feature acceptance crosswalk](../../../planning/v095-reconciliation/FEATURE_CROSSWALK_1407.md) and [decision register](../../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md). Runtime v4 is v0.93.3; tooling/foundations v0.93.4; GHB v0.93.5; remaining governance v0.93.6; affect/humour v0.93.7. Private implementations remain in their owner repositories. Earlier milestone names, source paths and proposed operations in retained source inputs are historical, not current execution authority. No feature is accepted or scope changed by this candidate document.

## Status

Forward-planning feature contract for `v0.94`.

## Purpose

Define the bounded reasoning-graph baseline that gives ADL an inspectable,
queryable reasoning substrate aligned with signed trace, provenance, and later
temporal/self-projection work.

## Source Inputs

- `docs/milestones/v0.85/REASONING_GRAPH_SCHEMA_V0.85.md`
- `docs/milestones/v0.90/features/HYPOTHESIS_ENGINE_REASONING_GRAPH_V0.9.md`
- `docs/milestones/v0.85/features/ROAD_TO_v0.95.md`
- `docs/planning/ADL_FEATURE_LIST.md`

## Scope

This feature should establish:

- the bounded reasoning-graph contract ADL will actually converge on for MVP
- relationship between reasoning graphs, signed trace, and trace query
- compatibility with Gödel/GHB reasoning surfaces without collapsing them into
  one opaque graph mythology
- one clear milestone home for reasoning/provenance closure in `v0.94`

## Non-goals

- redoing earlier cognitive or trace baselines from scratch
- an unconstrained graph-analytics platform
- hiding provenance or authority decisions inside graph structure

## Completion Target

`v0.94`
