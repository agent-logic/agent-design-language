---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "<slug>-validation-plan"
issue: 757
task_id: "issue-0757"
run_id: "issue-0757"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.01][runtime] Make dynamic-agent removal crash-consistent"
branch: "codex/757-dynamic-agent-removal-crash-consistency"
generated_at: "<timestamp>"
card_status: "ready"
status: "<status>"
initial_pvf_lane: "<initial_pvf_lane>"
planned_pvf_lane: "local_cpu"
lane_registry_path: "<lane_registry_path>"
lane_registry_template_set: "<lane_registry_template_set>"
validation_runtime_class: "deterministic_local_cpu"
validation_resource_profile: "medium"
validation_family: "runtime_crash_consistency"
validation_size_split: "focused"
expected_proof_cost: "<expected_proof_cost>"
planned_validation_seconds: "600"
planned_validation_tokens: "8000"
issue_goal_ref: "<issue_goal_ref>"
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "<goal_metrics_rollup_ref>"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/757"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
selected_lanes:
  - "dynamic-removal-failure-injection; restart-reconciliation; runtime-owner-focused; diff-hygiene"
parallel_groups:
  - "<parallel_groups_inline>"
validation_commands:
  - "<validation_commands_inline>"
failure_policy: "Fail on any partial state that resurrects a tombstoned agent, loses a recoverable resident checkpoint, reports failed removal with unrecoverable identity, or leaves projections inconsistent."
notes: "<notes_risks_inline>"
---

Canonical Template Source: `docs/templates/prompts/1.0.4/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

<plan_summary>

## Lane Registry Inputs

- Registry path: `<lane_registry_path>`
- Registry template set: `<lane_registry_template_set>`
- Initial PVF lane from issue creation: `<initial_pvf_lane>`
- Planned PVF lane for execution: `local_cpu`

## Selected Validation Lanes

- dynamic-removal-failure-injection; restart-reconciliation; runtime-owner-focused; diff-hygiene

## Parallelization Plan

- Parallel groups: <parallel_groups_inline>
- Validation runtime class: `deterministic_local_cpu`
- Validation resource profile: `medium`
- Validation family: `runtime_crash_consistency`
- Validation size split: `focused`

## Goal Accounting Hooks

- Issue goal ref: `<issue_goal_ref>`
- Sprint goal ref: `<sprint_goal_ref>`
- Goal metrics rollup ref: `<goal_metrics_rollup_ref>`

## Proof Cost / Runtime Expectations

- Expected proof cost: `<expected_proof_cost>`
- Planned validation seconds: `600`
- Planned validation token budget: `8000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- <validation_commands_inline>

## Failure Semantics

- Fail on any partial state that resurrects a tombstoned agent, loses a recoverable resident checkpoint, reports failed removal with unrecoverable identity, or leaves projections inconsistent.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

<notes_risks_inline>
