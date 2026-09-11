---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "issue-844-native-pr-merge-validation-plan"
issue: 844
task_id: "issue-0844"
run_id: "issue-0844"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.24][csdlc] Add a first-class native v3 pull-request merge operation"
branch: "codex/844-native-pr-merge"
generated_at: "2026-09-11T15:54:41.182312+00:00"
card_status: "ready"
status: "in_progress"
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "small"
validation_resource_profile: "local CPU/Git and fake transport; no live mutation"
validation_family: "csdlc"
validation_size_split: "focused deterministic owner tests; full owner lane"
expected_proof_cost: "local CPU; no live merge needed for regression proof"
planned_validation_seconds: "unknown"
planned_validation_tokens: "unknown"
issue_goal_ref: "Planning #7 active issue844 session goal"
sprint_goal_ref: "not_applicable; issue-local goal"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/844"
  - kind: "stp"
    ref: ".csdlc/issues/844/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/844/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/844/cards/spp.md"
selected_lanes:
  - "native C-SDLC v3 contract"
parallel_groups:
  - "separate full native tests and clippy"
validation_commands:
  - "cargo test --manifest-path csdlc-v3/Cargo.toml --all-targets -- --test-threads=1; cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings; cargo fmt --manifest-path csdlc-v3/Cargo.toml -- --check"
failure_policy: "Fail closed for every failed check, incomplete policy, unknown remote state or missing review."
notes: "Eligibility must be authenticated and fail closed for incomplete pagination or protection data. GitHub exact-head SHA condition constrains race; never fabricate terminal result."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Local deterministic merge contract plus native all-target suite; hosted CI remains integration proof.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`

## Selected Validation Lanes

- native C-SDLC v3 contract

## Parallelization Plan

- Parallel groups: separate full native tests and clippy
- Validation runtime class: `small`
- Validation resource profile: `local CPU/Git and fake transport; no live mutation`
- Validation family: `csdlc`
- Validation size split: `focused deterministic owner tests; full owner lane`

## Goal Accounting Hooks

- Issue goal ref: `Planning #7 active issue844 session goal`
- Sprint goal ref: `not_applicable; issue-local goal`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `local CPU; no live merge needed for regression proof`
- Planned validation seconds: `unknown`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path csdlc-v3/Cargo.toml --all-targets -- --test-threads=1; cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings; cargo fmt --manifest-path csdlc-v3/Cargo.toml -- --check

## Failure Semantics

- Fail closed for every failed check, incomplete policy, unknown remote state or missing review.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Eligibility must be authenticated and fail closed for incomplete pagination or protection data. GitHub exact-head SHA condition constrains race; never fabricate terminal result.
