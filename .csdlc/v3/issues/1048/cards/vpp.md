---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "publication-metadata-amendment-validation-plan"
issue: 1048
task_id: "issue-1048"
run_id: "issue-1048"
version: "v0.92.2"
title: "[C-SDLC] Allow typed correction of accepted publication metadata before dispatch"
branch: "codex/1048-publication-metadata-amendment"
generated_at: "2026-09-17T00:17:50.059444+00:00"
card_status: "ready"
status: "pre_execution"
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
lane_registry_path: "<lane_registry_path>"
lane_registry_template_set: "<lane_registry_template_set>"
validation_runtime_class: "<validation_runtime_class>"
validation_resource_profile: "small local CPU/Git fixtures"
validation_family: "<validation_family>"
validation_size_split: "<validation_size_split>"
expected_proof_cost: "<expected_proof_cost>"
planned_validation_seconds: "<planned_validation_seconds>"
planned_validation_tokens: "<planned_validation_tokens>"
issue_goal_ref: "<issue_goal_ref>"
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "<goal_metrics_rollup_ref>"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1048"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
selected_lanes:
  - "deterministic local Rust lifecycle/contract fixtures; no external effects"
parallel_groups:
  - "<parallel_groups_inline>"
validation_commands:
  - "Focused Rust semantic amendment/storage/intent command tests and cargo fmt; command-manifest/manual contract checks for touched documentation. Fixtures simulate remote effects and assert zero dispatch on rejected admission. PVF deterministic local tooling/contract proof, small CPU/local Git, no live cloud writes. Actual migration evidence is outside this issue."
failure_policy: "Fail closed on invalid metadata, stale evidence, identity mismatch, ambiguous remote effect or duplicate attempt."
notes: "<notes_risks_inline>"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

<plan_summary>

## Lane Registry Inputs

- Registry path: `<lane_registry_path>`
- Registry template set: `<lane_registry_template_set>`
- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`

## Selected Validation Lanes

- deterministic local Rust lifecycle/contract fixtures; no external effects

## Parallelization Plan

- Parallel groups: <parallel_groups_inline>
- Validation runtime class: `<validation_runtime_class>`
- Validation resource profile: `small local CPU/Git fixtures`
- Validation family: `<validation_family>`
- Validation size split: `<validation_size_split>`

## Goal Accounting Hooks

- Issue goal ref: `<issue_goal_ref>`
- Sprint goal ref: `<sprint_goal_ref>`
- Goal metrics rollup ref: `<goal_metrics_rollup_ref>`

## Proof Cost / Runtime Expectations

- Expected proof cost: `<expected_proof_cost>`
- Planned validation seconds: `<planned_validation_seconds>`
- Planned validation token budget: `<planned_validation_tokens>`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Focused Rust semantic amendment/storage/intent command tests and cargo fmt; command-manifest/manual contract checks for touched documentation. Fixtures simulate remote effects and assert zero dispatch on rejected admission. PVF deterministic local tooling/contract proof, small CPU/local Git, no live cloud writes. Actual migration evidence is outside this issue.

## Failure Semantics

- Fail closed on invalid metadata, stale evidence, identity mismatch, ambiguous remote effect or duplicate attempt.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

<notes_risks_inline>
