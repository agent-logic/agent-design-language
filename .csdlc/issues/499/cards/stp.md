# Structured Task Prompt

Template: 1.0.0

Issue: 499

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue completion is exactly one behavior-preserving resilience owner-boundary refactor; module extraction and test relocation are internal steps and line movement is not a separate result.

## Deliverables

- One refactored resilience module family with explicit owner boundaries and a narrower change-validation surface.
- Bounded validation evidence
- Exact-head review receipt

## Acceptance

1. AC-1: Supported resilience behavior and public API remain compatible
2. AC-2: Each extracted module has one coherent owner
3. AC-3: Tests remain behavior-focused and PVF-classified
4. AC-4: Validation-impact change is measured exactly
5. AC-5: No line-count reduction quota is imposed

## Dependencies

- #480 WP-01 merged opening gate

## Inputs

- agent-logic/agent-design-language#499
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml#RUST-01
- docs/milestones/v0.92.1/SPRINT_v0.92.1.md

## Non Goals

- Repository-wide refactoring
- Mandatory LoC reduction
- Runtime v4
- Aesthetic file splitting
