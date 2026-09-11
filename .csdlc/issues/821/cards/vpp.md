---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "<slug>-validation-plan"
issue: 821
task_id: "issue-0821"
run_id: "issue-0821"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.08d][quality] Re-prove current TAIL-01 quality-gate obligations"
branch: "codex/821-current-quality-gate"
generated_at: "<timestamp>"
card_status: "ready"
status: "<status>"
initial_pvf_lane: "release-evidence"
planned_pvf_lane: "release-evidence"
lane_registry_path: "<lane_registry_path>"
lane_registry_template_set: "<lane_registry_template_set>"
validation_runtime_class: "release"
validation_resource_profile: "Small local JSON and path checks now; existing required release lane resources at final candidate."
validation_family: "release-evidence"
validation_size_split: "preparation versus final candidate proof"
expected_proof_cost: "unknown"
planned_validation_seconds: "unknown"
planned_validation_tokens: "unknown"
issue_goal_ref: "<issue_goal_ref>"
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "<goal_metrics_rollup_ref>"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/821"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
selected_lanes:
  - "Four-row denominator integrity and existing TAIL-01 required proving lanes."
parallel_groups:
  - "Read-only dependency inspection independent of proof-plan preparation; final gate serialized after prerequisite integration."
validation_commands:
  - "ruby .csdlc/prepared/issues/517/validate-quality-gate.rb; python3 docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/validate.py; ruby .csdlc/prepared/issues/819/validate-retained-v3-proof.rb"
failure_policy: "Fail closed on missing, skipped, non-proving, stale or mismatched evidence; never promote preparation checks to final gate proof."
notes: "These commands validate retained inputs only. Required final-candidate lane execution and reconciliation remain pending #818/#820 integration; no release recommendation."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Prepare exact four-row consumption map and existing command sequence, record live prerequisite state, then regenerate final gate after #818/#820 integration, reconcile census, review exact candidate and publish.

## Lane Registry Inputs

- Registry path: `<lane_registry_path>`
- Registry template set: `<lane_registry_template_set>`
- Initial PVF lane from issue creation: `release-evidence`
- Planned PVF lane for execution: `release-evidence`

## Selected Validation Lanes

- Four-row denominator integrity and existing TAIL-01 required proving lanes.

## Parallelization Plan

- Parallel groups: Read-only dependency inspection independent of proof-plan preparation; final gate serialized after prerequisite integration.
- Validation runtime class: `release`
- Validation resource profile: `Small local JSON and path checks now; existing required release lane resources at final candidate.`
- Validation family: `release-evidence`
- Validation size split: `preparation versus final candidate proof`

## Goal Accounting Hooks

- Issue goal ref: `<issue_goal_ref>`
- Sprint goal ref: `<sprint_goal_ref>`
- Goal metrics rollup ref: `<goal_metrics_rollup_ref>`

## Proof Cost / Runtime Expectations

- Expected proof cost: `unknown`
- Planned validation seconds: `unknown`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- ruby .csdlc/prepared/issues/517/validate-quality-gate.rb; python3 docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/validate.py; ruby .csdlc/prepared/issues/819/validate-retained-v3-proof.rb

## Failure Semantics

- Fail closed on missing, skipped, non-proving, stale or mismatched evidence; never promote preparation checks to final gate proof.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

These commands validate retained inputs only. Required final-candidate lane execution and reconciliation remain pending #818/#820 integration; no release recommendation.
