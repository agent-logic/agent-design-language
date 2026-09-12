---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "909-gcp-move-in-validation-plan"
issue: 909
task_id: "issue-0909"
run_id: "issue-0909"
version: "v0.92.2"
title: "[v0.92.2][OPS-GCP] Produce one apply-ready company GCP move-in execution packet"
branch: "codex/909-gcp-move-in"
generated_at: "<timestamp>"
card_status: "ready"
status: "prepared"
initial_pvf_lane: "cloud-operations"
planned_pvf_lane: "cloud-operations"
lane_registry_path: "<lane_registry_path>"
lane_registry_template_set: "<lane_registry_template_set>"
validation_runtime_class: "bounded"
validation_resource_profile: "Read-only GCP and Terraform plan; local consistency/redaction checks; no apply or state changes"
validation_family: "Read-only GCP and Terraform plan; local consistency/redaction checks; no apply or state changes"
validation_size_split: "<validation_size_split>"
expected_proof_cost: "<expected_proof_cost>"
planned_validation_seconds: "1800"
planned_validation_tokens: "12000"
issue_goal_ref: "Create Sprint 8 issue #909 session goal after binding and before implementation"
sprint_goal_ref: "Sprint 8 umbrella #934"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/909"
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
failure_policy: "Missing or failed proof blocks acceptance; read failure is not resource absence; local checks do not prove live deployment."
notes: "<notes_risks_inline>"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

<plan_summary>

## Lane Registry Inputs

- Registry path: `<lane_registry_path>`
- Registry template set: `<lane_registry_template_set>`
- Initial PVF lane from issue creation: `cloud-operations`
- Planned PVF lane for execution: `cloud-operations`

## Selected Validation Lanes

- <selected_lanes_inline>

## Parallelization Plan

- Parallel groups: <parallel_groups_inline>
- Validation runtime class: `bounded`
- Validation resource profile: `Read-only GCP and Terraform plan; local consistency/redaction checks; no apply or state changes`
- Validation family: `Read-only GCP and Terraform plan; local consistency/redaction checks; no apply or state changes`
- Validation size split: `<validation_size_split>`

## Goal Accounting Hooks

- Issue goal ref: `Create Sprint 8 issue #909 session goal after binding and before implementation`
- Sprint goal ref: `Sprint 8 umbrella #934`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `<expected_proof_cost>`
- Planned validation seconds: `1800`
- Planned validation token budget: `12000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- <validation_commands_inline>

## Failure Semantics

- Missing or failed proof blocks acceptance; read failure is not resource absence; local checks do not prove live deployment.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

<notes_risks_inline>
