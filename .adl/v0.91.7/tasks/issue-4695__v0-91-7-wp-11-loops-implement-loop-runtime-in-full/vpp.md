---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0-91-7-wp-11-loops-implement-loop-runtime-in-full-validation-plan"
issue: 4695
task_id: "issue-4695"
run_id: "issue-4695"
version: "v0.91.7"
title: "[v0.91.7][WP-11][loops] Implement loop runtime in full"
branch: "codex/4695-v0-91-7-wp-11-loops-implement-loop-runtime-in-full"
generated_at: "2026-07-10T02:57:02Z"
card_status: "ready"
status: "ready"
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "vpp.lane.v1"
validation_runtime_class: "small"
validation_resource_profile: "local"
validation_family: "runtime_loop_runtime_profile"
validation_size_split: "small_only"
expected_proof_cost: "medium"
planned_validation_seconds: "1800"
planned_validation_tokens: "24000"
issue_goal_ref: "issue-4695"
sprint_goal_ref: "unknown"
goal_metrics_rollup_ref: "unknown"
source_refs:
  - kind: "issue"
    ref: "https://github.com/danielbaustin/agent-design-language/issues/4695"
  - kind: "stp"
    ref: ".adl/v0.91.7/tasks/issue-4695__v0-91-7-wp-11-loops-implement-loop-runtime-in-full/stp.md"
  - kind: "sip"
    ref: ".adl/v0.91.7/tasks/issue-4695__v0-91-7-wp-11-loops-implement-loop-runtime-in-full/sip.md"
  - kind: "spp"
    ref: ".adl/v0.91.7/tasks/issue-4695__v0-91-7-wp-11-loops-implement-loop-runtime-in-full/spp.md"
selected_lanes:
  - "runtime_focused_unit"
  - "cli_subcommand_smoke"
  - "diff_hygiene"
parallel_groups:
  - "runtime_v2_loop_runtime"
validation_commands:
  - "cargo test --manifest-path adl/Cargo.toml runtime_v2_loop_runtime -- --nocapture"
  - "adl/target/debug/adl runtime-v2 loop-runtime --out artifacts/v0917/issue-4695-loop-runtime/loop-runtime.json"
  - "git diff --check"
failure_policy: "fail_closed"
notes: "Updated during issue execution for the runtime loop-runtime implementation. Broad workspace tests, coverage release gates, and slow/remote proof lanes are deferred because the touched surface is a bounded Runtime v2 contract, CLI subcommand, focused unit tests, and ignored generated artifact proof."
---

Canonical Template Source: `docs/templates/prompts/1.0.3/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Validation planning prompt for [v0.91.7][WP-11][loops] Implement loop runtime in full; source issue prompt: .adl/v0.91.7/bodies/issue-4695-v0-91-7-wp-11-loops-implement-loop-runtime-in-full.md.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `vpp.lane.v1`
- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`

## Selected Validation Lanes

- docs_diff_check

## Parallelization Plan

- Parallel groups: runtime_v2_loop_runtime
- Validation runtime class: `small`
- Validation resource profile: `local`
- Validation family: `runtime_loop_runtime_profile`
- Validation size split: `small_only`

## Goal Accounting Hooks

- Issue goal ref: `issue-4695`
- Sprint goal ref: `unknown`
- Goal metrics rollup ref: `unknown`

## Proof Cost / Runtime Expectations

- Expected proof cost: `medium`
- Planned validation seconds: `1800`
- Planned validation token budget: `24000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path adl/Cargo.toml runtime_v2_loop_runtime -- --nocapture
- adl/target/debug/adl runtime-v2 loop-runtime --out artifacts/v0917/issue-4695-loop-runtime/loop-runtime.json
- git diff --check

## Failure Semantics

- fail_closed

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Updated during execution for the bounded Runtime v2 loop-runtime implementation. Broad workspace tests, coverage release gates, and slow/remote proof lanes are deferred because the proof surface is limited to a focused runtime contract, CLI subcommand integration, deterministic replay checks, and diff hygiene.
