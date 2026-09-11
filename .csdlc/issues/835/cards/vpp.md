---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "release-gate-projection-validation-plan"
issue: 835
task_id: "issue-0835"
run_id: "issue-0835"
version: "v0.92.1"
title: "Recompute v0.92.1 release-gate projection"
branch: "codex/835-release-gate-projection"
generated_at: "2026-09-11"
card_status: "ready"
status: "<status>"
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
lane_registry_path: "<lane_registry_path>"
lane_registry_template_set: "<lane_registry_template_set>"
validation_runtime_class: "small"
validation_resource_profile: "Local Git and deterministic Python; authenticated read-only GitHub snapshots"
validation_family: "release_projection_contract"
validation_size_split: "<validation_size_split>"
expected_proof_cost: "<expected_proof_cost>"
planned_validation_seconds: "120"
planned_validation_tokens: "not_collected"
issue_goal_ref: "Issue835 active session goal"
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "<goal_metrics_rollup_ref>"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/835"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
selected_lanes:
  - "<selected_lanes_inline>"
parallel_groups:
  - "<parallel_groups_inline>"
validation_commands:
  - "<validation_commands_inline>"
failure_policy: "Fail closed on missing current proof; no ready decision with unresolved gates"
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

- <selected_lanes_inline>

## Parallelization Plan

- Parallel groups: <parallel_groups_inline>
- Validation runtime class: `small`
- Validation resource profile: `Local Git and deterministic Python; authenticated read-only GitHub snapshots`
- Validation family: `release_projection_contract`
- Validation size split: `<validation_size_split>`

## Goal Accounting Hooks

- Issue goal ref: `Issue835 active session goal`
- Sprint goal ref: `<sprint_goal_ref>`
- Goal metrics rollup ref: `<goal_metrics_rollup_ref>`

## Proof Cost / Runtime Expectations

- Expected proof cost: `<expected_proof_cost>`
- Planned validation seconds: `120`
- Planned validation token budget: `not_collected`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- <validation_commands_inline>

## Failure Semantics

- Fail closed on missing current proof; no ready decision with unresolved gates

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

<notes_risks_inline>
