---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "<slug>-validation-plan"
issue: 526
task_id: "issue-0526"
run_id: "issue-0526"
version: "v0.92.1"
title: "[v0.92.1][TAIL-10] Release ceremony"
branch: "codex/526-release-ceremony"
generated_at: "<timestamp>"
card_status: "ready"
status: "<status>"
initial_pvf_lane: "release-evidence"
planned_pvf_lane: "release-evidence"
lane_registry_path: "<lane_registry_path>"
lane_registry_template_set: "<lane_registry_template_set>"
validation_runtime_class: "local_contract"
validation_resource_profile: "Small local JSON/Markdown plus read-only GitHub queries; no Rust builds or cloud runs."
validation_family: "release-evidence"
validation_size_split: "small preparation checks"
expected_proof_cost: "<expected_proof_cost>"
planned_validation_seconds: "<planned_validation_seconds>"
planned_validation_tokens: "<planned_validation_tokens>"
issue_goal_ref: "<issue_goal_ref>"
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "<goal_metrics_rollup_ref>"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/526"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
selected_lanes:
  - "tail-merge-census; notes-parity; tag-and-release-readback (read-only preparation)"
parallel_groups:
  - "Independent prerequisite and artifact inspection."
validation_commands:
  - "Native validate; assemble_release_evidence.py; read-only Git/GitHub census and hash verification."
failure_policy: "Missing proof, absent operator approval or candidate drift forbids ceremony; preparation remains non-authorizing."
notes: "<notes_risks_inline>"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Assemble live prerequisite and canonical evidence inputs; record exact missing gates, notes hash and proposed tag; prepare execution/readback checklist; review preparation and stop before operator release approval.

## Lane Registry Inputs

- Registry path: `<lane_registry_path>`
- Registry template set: `<lane_registry_template_set>`
- Initial PVF lane from issue creation: `release-evidence`
- Planned PVF lane for execution: `release-evidence`

## Selected Validation Lanes

- tail-merge-census; notes-parity; tag-and-release-readback (read-only preparation)

## Parallelization Plan

- Parallel groups: Independent prerequisite and artifact inspection.
- Validation runtime class: `local_contract`
- Validation resource profile: `Small local JSON/Markdown plus read-only GitHub queries; no Rust builds or cloud runs.`
- Validation family: `release-evidence`
- Validation size split: `small preparation checks`

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

- Native validate; assemble_release_evidence.py; read-only Git/GitHub census and hash verification.

## Failure Semantics

- Missing proof, absent operator approval or candidate drift forbids ceremony; preparation remains non-authorizing.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

<notes_risks_inline>
