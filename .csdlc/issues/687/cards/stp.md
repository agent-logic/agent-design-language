# Structured Task Prompt

Template: 1.0.0

Issue: 687

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement the residual provider/model inference-readiness truth model and focused deterministic projection proof only.

## Deliverables

- Typed inference-readiness taxonomy
- Consistent resident Shepherd and dynamic-agent roster/API projection
- Typed resident Shepherd attempt-failure classification
- Focused deterministic state-matrix and production-credit tests

## Acceptance

1. AC-1: A serialized closed taxonomy defines unimplemented, unavailable, model_loading, failed, and ready
2. AC-2: Only ready inference is communication-eligible and receives healthy/available projection
3. AC-3: Resident Shepherd recovery distinguishes unsupported adapter, unavailable provider/model, failed probe, loading, and ready
4. AC-4: Dynamic-agent refresh retains and projects concrete verification failure semantics instead of a boolean
5. AC-5: Agent sample, roster entry, and detail projection preserve one inference state without identity drift
6. AC-6: Missing or placeholder production adapters receive no ready credit
7. AC-7: Focused deterministic tests, formatting, diff hygiene, and independent exact-head review pass
8. AC-8: Validation performs no live Runtime, provider, credential, spend, or cloud mutation

## Dependencies

- Issue #640 / PR #653 resident Shepherd model-backed execution baseline
- Issues #622 and #648 provider profile reload baseline
- .adl/docs/TBD/resilience/RUNTIME_V3_LAUNCH_AND_OBSERVATORY_RECOVERY_PLAN.md

## Inputs

- agent-logic/agent-design-language#687
- adl-runtime-kernel/src/resident_shepherd.rs
- adl-runtime-kernel/src/control/feeds.rs
- adl-runtime-kernel/src/agent_roster.rs
- adl-runtime-kernel/src/assembly.rs

## Non Goals

- No provider implementation or credential changes
- No provider-profile hot reload changes
- No canonical identity or dynamic lifecycle redesign
- No Observatory UI or Runtime v2 changes
- No live deployment, restart, provider inference, or cloud work
