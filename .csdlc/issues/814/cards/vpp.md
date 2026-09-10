---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "greeting-recovery-local-model-boundary-validation-plan"
issue: 814
task_id: "issue-0814"
run_id: "issue-0814"
version: "1.0.5"
title: "[v0.92.1][TAIL-06.09][runtime] Repair greeting recovery identity and local-model boundary"
branch: "codex/814-greeting-recovery-local-model-boundary"
generated_at: "<timestamp>"
card_status: "ready"
status: "ready"
initial_pvf_lane: "runtime_focused"
planned_pvf_lane: "runtime_focused"
lane_registry_path: "adl/tools/owner-validation-lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "medium"
validation_resource_profile: "local_cpu"
validation_family: "runtime"
validation_size_split: "focused_then_owner_lane"
expected_proof_cost: "local_only"
planned_validation_seconds: "900"
planned_validation_tokens: "4000"
issue_goal_ref: "codex-goal:issue-814"
sprint_goal_ref: "issue-520-review-remediation"
goal_metrics_rollup_ref: "issue-522"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/814"
  - kind: "stp"
    ref: ".csdlc/issues/814/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/814/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/814/cards/spp.md"
selected_lanes:
  - "focused admission recovery; focused Shepherd URL boundary; Runtime owner validation; formatting and diff hygiene"
parallel_groups:
  - "focused tests may run independently before serialized owner validation"
validation_commands:
  - "cargo test --manifest-path adl-runtime-kernel/Cargo.toml; cargo test --manifest-path adl-runtime/Cargo.toml --test shepherd_local_model; bash adl/tools/run_owner_validation_lane.sh runtime; cargo fmt --check; git diff --check"
failure_policy: "Fail closed on any regression, lint, formatting, review, or boundary failure."
notes: "Do not substitute a mock provider or weaken local-only trust boundaries."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Use deterministic focused Runtime regressions as primary proof, then run the proportional Runtime owner lane and exact-head review.

## Lane Registry Inputs

- Registry path: `adl/tools/owner-validation-lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `runtime_focused`
- Planned PVF lane for execution: `runtime_focused`

## Selected Validation Lanes

- focused admission recovery; focused Shepherd URL boundary; Runtime owner validation; formatting and diff hygiene

## Parallelization Plan

- Parallel groups: focused tests may run independently before serialized owner validation
- Validation runtime class: `medium`
- Validation resource profile: `local_cpu`
- Validation family: `runtime`
- Validation size split: `focused_then_owner_lane`

## Goal Accounting Hooks

- Issue goal ref: `codex-goal:issue-814`
- Sprint goal ref: `issue-520-review-remediation`
- Goal metrics rollup ref: `issue-522`

## Proof Cost / Runtime Expectations

- Expected proof cost: `local_only`
- Planned validation seconds: `900`
- Planned validation token budget: `4000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path adl-runtime-kernel/Cargo.toml; cargo test --manifest-path adl-runtime/Cargo.toml --test shepherd_local_model; bash adl/tools/run_owner_validation_lane.sh runtime; cargo fmt --check; git diff --check

## Failure Semantics

- Fail closed on any regression, lint, formatting, review, or boundary failure.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Do not substitute a mock provider or weaken local-only trust boundaries.
