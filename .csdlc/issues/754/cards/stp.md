# Structured Task Prompt

Template: 1.0.0

Issue: 754

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Registry compatibility and focused regressions only.

## Deliverables

- Registry fix
- Regression tests
- Standalone validation and independent review

## Acceptance

1. AC-1: Accept coherent1.0.3/1.0.4 registries.
2. AC-2: Reject identity/path/shape drift before writes.
3. AC-3: Standalone tests/fmt/clippy and independent review pass.

## Dependencies

- None; separate blocker fix for PR753.

## Inputs

- csdlc-v2/src/registry.rs
- docs/templates/prompts/current.json

## Non Goals

- Native card version changes
- Release approval
- PR753 merge
