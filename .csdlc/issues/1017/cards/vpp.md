---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "<slug>-validation-plan"
issue: 1017
task_id: "issue-1017"
run_id: "issue-1017"
version: "1.0.5"
title: "[v0.92.2][corporate][AWS] Migrate v-*.ai domains from personal AWS to the company account"
branch: "codex/1017-v0922-company-domain-migration"
generated_at: "<timestamp>"
card_status: "ready"
status: "ready"
initial_pvf_lane: "<initial_pvf_lane>"
planned_pvf_lane: "<planned_pvf_lane>"
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
    ref: "https://github.com/agent-logic/agent-design-language/issues/1017"
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
  - "Native validate plus declared semantic_card_projections Cargo tests for local lifecycle structure only; git diff --check and independent privacy review. Live AWS source/destination inventory, six operation statuses, destination details, before/after zone records, and public NS/SOA/A/AAAA/MX/TXT/DS comparison recorded separately. Company contact types/organization and contact-email equality with agent-logic.ai verified privately."
failure_policy: "<failure_policy>"
notes: "Require separate live AWS transfer/control/configuration and public DNS before/after evidence. Empty source zones retained with owner approval until v0.93 website work. No DNS cutover is planned, so TTL propagation observation is not applicable. Retain standard registrar contact notification path; do not claim custom alarms or delivered-email proof. Local card tests prove lifecycle structure only."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

<plan_summary>

## Lane Registry Inputs

- Registry path: `<lane_registry_path>`
- Registry template set: `<lane_registry_template_set>`
- Initial PVF lane from issue creation: `<initial_pvf_lane>`
- Planned PVF lane for execution: `<planned_pvf_lane>`

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
- Planned validation seconds: `<planned_validation_seconds>`
- Planned validation token budget: `<planned_validation_tokens>`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Native validate plus declared semantic_card_projections Cargo tests for local lifecycle structure only; git diff --check and independent privacy review. Live AWS source/destination inventory, six operation statuses, destination details, before/after zone records, and public NS/SOA/A/AAAA/MX/TXT/DS comparison recorded separately. Company contact types/organization and contact-email equality with agent-logic.ai verified privately.

## Failure Semantics

- <failure_policy>

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Require separate live AWS transfer/control/configuration and public DNS before/after evidence. Empty source zones retained with owner approval until v0.93 website work. No DNS cutover is planned, so TTL propagation observation is not applicable. Retain standard registrar contact notification path; do not claim custom alarms or delivered-email proof. Local card tests prove lifecycle structure only.
