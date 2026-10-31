# v0.95 Feature: CSM Shepherd Model and Gemma Training Path

> **Current planning boundary (#1407, 2026-10-31):** Follow the [feature acceptance crosswalk](../../../planning/v095-reconciliation/FEATURE_CROSSWALK_1407.md) and [decision register](../../../planning/v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md). Runtime v4 is v0.93.3; tooling/foundations v0.93.4; GHB v0.93.5; remaining governance v0.93.6; affect/humour v0.93.7. Private implementations remain in their owner repositories. Earlier milestone names, source paths and proposed operations in retained source inputs are historical, not current execution authority. No feature is accepted or scope changed by this candidate document.

## Status

Forward-planning feature contract for `v0.95`.

## Purpose

Finish the MVP-facing evaluator and training path for the Shepherd-model /
Gemma line so the ANRM placement work from `v0.91.1` matures into a coherent
runtime-scaffold and training/evaluation story.

## Source Inputs

- `docs/milestones/v0.90.1/ideas/ANRM_SCAFFOLDED_SMALL_MODEL_EXPERIMENT.md`
- `docs/milestones/v0.87.1/features/SHEPHERD_RUNTIME_MODEL.md`
- `docs/planning/ADL_FEATURE_LIST.md`

## Scope

This feature should establish:

- the bounded training/evaluator path for the Shepherd/Gemma line
- explicit relationship to ANRM placement, trace datasets, and runtime care
  semantics
- a truthful MVP home for the later small-model runtime story

## Non-goals

- claiming successful model training before evidence exists
- replacing the broader runtime substrate with one model family
- treating scaffold experiments as product completion by themselves

## Completion Target

`v0.95`
