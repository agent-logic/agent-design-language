---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "<slug>-validation-plan"
issue: 836
task_id: "issue-0836"
run_id: "issue-0836"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.21][quality] Publish recursive code-size and relocation evidence"
branch: "codex/836-recursive-rust-size"
generated_at: "<timestamp>"
card_status: "ready"
status: "<status>"
initial_pvf_lane: "local_contract"
planned_pvf_lane: "local_contract"
lane_registry_path: "<lane_registry_path>"
lane_registry_template_set: "<lane_registry_template_set>"
validation_runtime_class: "local_contract"
validation_resource_profile: "Small local CPU and temporary Git repositories; no cloud, credentials or Rust compilation."
validation_family: "source-accounting"
validation_size_split: "focused"
expected_proof_cost: "<expected_proof_cost>"
planned_validation_seconds: "<planned_validation_seconds>"
planned_validation_tokens: "<planned_validation_tokens>"
issue_goal_ref: "<issue_goal_ref>"
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "<goal_metrics_rollup_ref>"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/836"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
selected_lanes:
  - "Recursive Git inventory; revision-bound diff and relocation; byte stability; document claims guardrail."
parallel_groups:
  - "Independent documentation audit and algorithm review; serialized evidence generation."
validation_commands:
  - "python3 docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/test_measure.py; python3 docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/measure.py --check docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/measurement.json"
failure_policy: "Reject top-level-only inventory, unresolved revisions, missing scope and unsupported reduction claims. Do not equate size deltas or moved lines with behavioral proof."
notes: "Three focused tests, seven corrupted-report cases and two independent byte-identical recomputations. Exact-head review and hosted CI pending; no Rust compilation."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Freeze PR547 baseline/candidate and recursive resilience source plus declared test scope; build deterministic Git-object inventory and diff/relocation report; retain exact evidence; correct unsupported release claims; validate and review before publication.

## Lane Registry Inputs

- Registry path: `<lane_registry_path>`
- Registry template set: `<lane_registry_template_set>`
- Initial PVF lane from issue creation: `local_contract`
- Planned PVF lane for execution: `local_contract`

## Selected Validation Lanes

- Recursive Git inventory; revision-bound diff and relocation; byte stability; document claims guardrail.

## Parallelization Plan

- Parallel groups: Independent documentation audit and algorithm review; serialized evidence generation.
- Validation runtime class: `local_contract`
- Validation resource profile: `Small local CPU and temporary Git repositories; no cloud, credentials or Rust compilation.`
- Validation family: `source-accounting`
- Validation size split: `focused`

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

- python3 docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/test_measure.py; python3 docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/measure.py --check docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/measurement.json

## Failure Semantics

- Reject top-level-only inventory, unresolved revisions, missing scope and unsupported reduction claims. Do not equate size deltas or moved lines with behavioral proof.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Three focused tests, seven corrupted-report cases and two independent byte-identical recomputations. Exact-head review and hosted CI pending; no Rust compilation.
