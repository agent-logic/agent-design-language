---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "<slug>-validation-plan"
issue: 811
task_id: "issue-0811"
run_id: "issue-0811"
version: "1.0.5"
title: "Reconcile every closed issue including no-PR dispositions"
branch: "codex/811-all-issue-closeout"
generated_at: "<timestamp>"
card_status: "ready"
status: "ready"
initial_pvf_lane: "focused_terminal"
planned_pvf_lane: "focused_terminal"
lane_registry_path: "<lane_registry_path>"
lane_registry_template_set: "<lane_registry_template_set>"
validation_runtime_class: "<validation_runtime_class>"
validation_resource_profile: "small"
validation_family: "tooling"
validation_size_split: "<validation_size_split>"
expected_proof_cost: "<expected_proof_cost>"
planned_validation_seconds: "<planned_validation_seconds>"
planned_validation_tokens: "<planned_validation_tokens>"
issue_goal_ref: "<issue_goal_ref>"
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "<goal_metrics_rollup_ref>"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/811"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
selected_lanes:
  - "focused_terminal"
parallel_groups:
  - "<parallel_groups_inline>"
validation_commands:
  - "cargo test --manifest-path csdlc-v3/Cargo.toml --test terminal_cleanup_cutover_commands"
failure_policy: "Fail closed on evidence mismatch; no broad runtime suite locally."
notes: "<notes_risks_inline>"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Focused terminal cleanup tests: no-PR closed/retired/superseded/umbrella success, wrong identity/open issue/stale evidence/conflicting receipt failures; existing merged closeout regression and supported closing keywords. Then rerun the 199-issue audited closeout sweep.

## Lane Registry Inputs

- Registry path: `<lane_registry_path>`
- Registry template set: `<lane_registry_template_set>`
- Initial PVF lane from issue creation: `focused_terminal`
- Planned PVF lane for execution: `focused_terminal`

## Selected Validation Lanes

- focused_terminal

## Parallelization Plan

- Parallel groups: <parallel_groups_inline>
- Validation runtime class: `<validation_runtime_class>`
- Validation resource profile: `small`
- Validation family: `tooling`
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

- cargo test --manifest-path csdlc-v3/Cargo.toml --test terminal_cleanup_cutover_commands

## Failure Semantics

- Fail closed on evidence mismatch; no broad runtime suite locally.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

<notes_risks_inline>
