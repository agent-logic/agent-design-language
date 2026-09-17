---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "<slug>-validation-plan"
issue: 1044
task_id: "issue-1044"
run_id: "issue-1044"
version: "v0.92.2"
title: "[C-SDLC v3][defect] Prepare never-bound issues with settled issue-edit history"
branch: "codex/1044-prepare-settled-issue-edits"
generated_at: "<timestamp>"
card_status: "ready"
status: "<status>"
initial_pvf_lane: "<initial_pvf_lane>"
planned_pvf_lane: "owner_binary"
lane_registry_path: "<lane_registry_path>"
lane_registry_template_set: "<lane_registry_template_set>"
validation_runtime_class: "<validation_runtime_class>"
validation_resource_profile: "<validation_resource_profile>"
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
    ref: "https://github.com/agent-logic/agent-design-language/issues/1044"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
selected_lanes:
  - "owner_binary deterministic local tests"
parallel_groups:
  - "<parallel_groups_inline>"
validation_commands:
  - "Run focused semantic transaction and installed preparation tests including negative remote-residue cases; inspect immutable receipt inventories before/after. Native validate/doctor and independent bounded source review. Local deterministic small CPU/disk proof; no AWS, provider or unrelated remote mutations."
failure_policy: "Fail closed for pending or corrupt receipts; do not broaden trusted mutation variants or discard history. No #1042 changes, shared binary replacement, or AWS calls."
notes: "<notes_risks_inline>"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

<plan_summary>

## Lane Registry Inputs

- Registry path: `<lane_registry_path>`
- Registry template set: `<lane_registry_template_set>`
- Initial PVF lane from issue creation: `<initial_pvf_lane>`
- Planned PVF lane for execution: `owner_binary`

## Selected Validation Lanes

- owner_binary deterministic local tests

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
- Planned validation seconds: `<planned_validation_seconds>`
- Planned validation token budget: `<planned_validation_tokens>`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Run focused semantic transaction and installed preparation tests including negative remote-residue cases; inspect immutable receipt inventories before/after. Native validate/doctor and independent bounded source review. Local deterministic small CPU/disk proof; no AWS, provider or unrelated remote mutations.

## Failure Semantics

- Fail closed for pending or corrupt receipts; do not broaden trusted mutation variants or discard history. No #1042 changes, shared binary replacement, or AWS calls.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

<notes_risks_inline>
