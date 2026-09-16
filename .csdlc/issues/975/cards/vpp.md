---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "merge-graphql-observation-validation-plan"
issue: 975
task_id: "issue-0975"
run_id: "issue-0975"
version: "v0.92.2"
title: "[v0.92.2][C-SDLC] Repair native merge GraphQL observation transport"
branch: "codex/975-v0922-merge-graphql-observation"
generated_at: "2026-09-14T17:17:30.658534+00:00"
card_status: "ready"
status: "IN_PROGRESS"
initial_pvf_lane: "deterministic_local"
planned_pvf_lane: "deterministic_local"
lane_registry_path: ".csdlc/evidence/975/VALIDATION.md"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "short"
validation_resource_profile: "local CPU/filesystem; synthetic subprocess; bounded authenticated read-only HTTP"
validation_family: "native_csdlc_adapter"
validation_size_split: "focused_library_cli_publication"
expected_proof_cost: "small local checks; four bounded live read observations; no paid cloud workload"
planned_validation_seconds: "unknown"
planned_validation_tokens: "unknown"
issue_goal_ref: "codex-goal:01a0875a-799c-7422-a9ae-e1eefbd1c932:975"
sprint_goal_ref: "not_applicable: delegated tooling repair"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/975"
  - kind: "stp"
    ref: ".csdlc/issues/975/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/975/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/975/cards/spp.md"
selected_lanes:
  - "Deterministic adapter/merge contracts; native CLI/publication contracts; bounded read-only GitHub transport observation."
parallel_groups:
  - "Local suites independent of read-only remote observation; remote mutations excluded."
validation_commands:
  - "cargo test --manifest-path csdlc-v3/Cargo.toml --lib; cargo test --manifest-path csdlc-v3/Cargo.toml --lib merge_adapter_tests; cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands --test remote_publication_commands; native validate; git diff --check."
failure_policy: "Stop publication on any adapter/merge/card failure; truncated, partial or GraphQL-error observations do not authorize mutation."
notes: "Live observation is transport proof only; hosted CI and independent review remain required."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Reproduce production transport failure, use minimal JSON POST fix, validate focused regressions, hand off for independent review before publication.

## Lane Registry Inputs

- Registry path: `.csdlc/evidence/975/VALIDATION.md`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `deterministic_local`
- Planned PVF lane for execution: `deterministic_local`

## Selected Validation Lanes

- Deterministic adapter/merge contracts; native CLI/publication contracts; bounded read-only GitHub transport observation.

## Parallelization Plan

- Parallel groups: Local suites independent of read-only remote observation; remote mutations excluded.
- Validation runtime class: `short`
- Validation resource profile: `local CPU/filesystem; synthetic subprocess; bounded authenticated read-only HTTP`
- Validation family: `native_csdlc_adapter`
- Validation size split: `focused_library_cli_publication`

## Goal Accounting Hooks

- Issue goal ref: `codex-goal:01a0875a-799c-7422-a9ae-e1eefbd1c932:975`
- Sprint goal ref: `not_applicable: delegated tooling repair`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `small local checks; four bounded live read observations; no paid cloud workload`
- Planned validation seconds: `unknown`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path csdlc-v3/Cargo.toml --lib; cargo test --manifest-path csdlc-v3/Cargo.toml --lib merge_adapter_tests; cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands --test remote_publication_commands; native validate; git diff --check.

## Failure Semantics

- Stop publication on any adapter/merge/card failure; truncated, partial or GraphQL-error observations do not authorize mutation.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Live observation is transport proof only; hosted CI and independent review remain required.
