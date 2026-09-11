---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "<slug>-validation-plan"
issue: 834
task_id: "issue-0834"
run_id: "issue-0834"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.19][review] Reconcile finalized internal-review predecessor"
branch: "codex/834-internal-review-reconciliation"
generated_at: "<timestamp>"
card_status: "ready"
status: "<status>"
initial_pvf_lane: "release-evidence"
planned_pvf_lane: "release-evidence"
lane_registry_path: "<lane_registry_path>"
lane_registry_template_set: "<lane_registry_template_set>"
validation_runtime_class: "small"
validation_resource_profile: "Small local CPU/disk; deterministic captured readbacks and local Git; no credentials in artifacts."
validation_family: "<validation_family>"
validation_size_split: "<validation_size_split>"
expected_proof_cost: "<expected_proof_cost>"
planned_validation_seconds: "300"
planned_validation_tokens: "4000"
issue_goal_ref: "<issue_goal_ref>"
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "<goal_metrics_rollup_ref>"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/834"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
selected_lanes:
  - "Required predecessor reconciliation and negative validator contracts."
parallel_groups:
  - "<parallel_groups_inline>"
validation_commands:
  - "python3 docs/milestones/v0.92.1/evidence/release/tail-06/issue-834/validate.py; python3 docs/milestones/v0.92.1/evidence/release/tail-06/issue-834/test_validate.py"
failure_policy: "Fail closed on stale state, wrong closing PR, nonancestor merge, missing/duplicate finding or unproven owner."
notes: "<notes_risks_inline>"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Inspect final520 disposition and historical external finding; map14 findings to verified merged owners; capture live closure/ancestry; add current reconciliation and fail-closed validator; run negatives; review exacthead and publish with required CI green.

## Lane Registry Inputs

- Registry path: `<lane_registry_path>`
- Registry template set: `<lane_registry_template_set>`
- Initial PVF lane from issue creation: `release-evidence`
- Planned PVF lane for execution: `release-evidence`

## Selected Validation Lanes

- Required predecessor reconciliation and negative validator contracts.

## Parallelization Plan

- Parallel groups: <parallel_groups_inline>
- Validation runtime class: `small`
- Validation resource profile: `Small local CPU/disk; deterministic captured readbacks and local Git; no credentials in artifacts.`
- Validation family: `<validation_family>`
- Validation size split: `<validation_size_split>`

## Goal Accounting Hooks

- Issue goal ref: `<issue_goal_ref>`
- Sprint goal ref: `<sprint_goal_ref>`
- Goal metrics rollup ref: `<goal_metrics_rollup_ref>`

## Proof Cost / Runtime Expectations

- Expected proof cost: `<expected_proof_cost>`
- Planned validation seconds: `300`
- Planned validation token budget: `4000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- python3 docs/milestones/v0.92.1/evidence/release/tail-06/issue-834/validate.py; python3 docs/milestones/v0.92.1/evidence/release/tail-06/issue-834/test_validate.py

## Failure Semantics

- Fail closed on stale state, wrong closing PR, nonancestor merge, missing/duplicate finding or unproven owner.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

<notes_risks_inline>
