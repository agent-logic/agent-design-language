---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "<slug>-validation-plan"
issue: 1249
task_id: "issue-1249"
run_id: "issue-1249"
version: "1.0.5"
title: "[v0.93.1][C-SDLC] Admit Node test proof and reconcile operative plan amendments"
branch: "codex/1249-node-proof-plan-reconciliation"
generated_at: "<timestamp>"
card_status: "ready"
status: "ready"
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
lane_registry_path: "<lane_registry_path>"
lane_registry_template_set: "<lane_registry_template_set>"
validation_runtime_class: "<validation_runtime_class>"
validation_resource_profile: "<validation_resource_profile>"
validation_family: "<validation_family>"
validation_size_split: "<validation_size_split>"
expected_proof_cost: "<expected_proof_cost>"
planned_validation_seconds: "300"
planned_validation_tokens: "unknown"
issue_goal_ref: "<issue_goal_ref>"
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "<goal_metrics_rollup_ref>"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1249"
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
failure_policy: "<failure_policy>"
notes: "No live providers or deployment. Maintain authority, CAS, binding, timeout, output bounds, redaction and independent review. Install isolated candidate only after review."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Add guarded Node --test execution and reconcile plan-class card steps, validate focused positive/negative tests, independently review, then publish and unblock1145.

## Lane Registry Inputs

- Registry path: `<lane_registry_path>`
- Registry template set: `<lane_registry_template_set>`
- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`

## Selected Validation Lanes

- <selected_lanes_inline>

## Parallelization Plan

- Parallel groups: <parallel_groups_inline>
- Validation runtime class: `<validation_runtime_class>`
- Validation resource profile: `<validation_resource_profile>`
- Validation family: `<validation_family>`
- Validation size split: `<validation_size_split>`

## Goal Accounting Hooks

- Issue goal ref: `<issue_goal_ref>`
- Sprint goal ref: `<sprint_goal_ref>`
- Goal metrics rollup ref: `<goal_metrics_rollup_ref>`

## Proof Cost / Runtime Expectations

- Expected proof cost: `<expected_proof_cost>`
- Planned validation seconds: `300`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- <validation_commands_inline>

## Failure Semantics

- <failure_policy>

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

No live providers or deployment. Maintain authority, CAS, binding, timeout, output bounds, redaction and independent review. Install isolated candidate only after review.
