---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "codefriend-four-plus-one-architecture-validation-plan"
issue: 1109
task_id: "issue-1109"
run_id: "issue-1109"
version: "1.0.5"
title: "[v0.92.2][CF-ARCH] Generate the complete 4+1 architecture package in Beta 1"
branch: "codex/1109-codefriend-four-plus-one-architecture"
generated_at: "<timestamp>"
card_status: "ready"
status: "IN_PROGRESS"
initial_pvf_lane: "runtime_full_validation"
planned_pvf_lane: "runtime_full_validation"
lane_registry_path: "issue-1109-proof-inventory"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "deterministic-local-and-installed"
validation_resource_profile: "local-cpu"
validation_family: "codefriend"
validation_size_split: "focused-then-installed"
expected_proof_cost: "unknown"
planned_validation_seconds: "unknown"
planned_validation_tokens: "unknown"
issue_goal_ref: "issue-1109-current-session"
sprint_goal_ref: "not-assigned"
goal_metrics_rollup_ref: "issue-1109-current-session"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1109"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
selected_lanes:
  - "Focused deterministic Rust contract tests; installed journey; rendered report inspection; CI integration lanes"
parallel_groups:
  - "sequential implementation and proof; bounded pre-PR independent review"
validation_commands:
  - "cargo test --manifest-path adl/Cargo.toml four_plus_one; cargo fmt --manifest-path adl/Cargo.toml --check; focused installed journey commands selected after CLI integration"
failure_policy: "Fail closed on missing view evidence, mismatched revision, invalid references, or unrun required installed proof."
notes: "Worker 9 assigned and bound. No implementation or proof complete. No new approval pipeline; missing evidence remains explicit. Independent qualification #915 is not claimed."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Generate a source-bound 4+1 package in the existing CodeFriend journey and publish through existing report owners; qualify complete, incomplete and conflicting evidence.

## Lane Registry Inputs

- Registry path: `issue-1109-proof-inventory`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `runtime_full_validation`
- Planned PVF lane for execution: `runtime_full_validation`

## Selected Validation Lanes

- Focused deterministic Rust contract tests; installed journey; rendered report inspection; CI integration lanes

## Parallelization Plan

- Parallel groups: sequential implementation and proof; bounded pre-PR independent review
- Validation runtime class: `deterministic-local-and-installed`
- Validation resource profile: `local-cpu`
- Validation family: `codefriend`
- Validation size split: `focused-then-installed`

## Goal Accounting Hooks

- Issue goal ref: `issue-1109-current-session`
- Sprint goal ref: `not-assigned`
- Goal metrics rollup ref: `issue-1109-current-session`

## Proof Cost / Runtime Expectations

- Expected proof cost: `unknown`
- Planned validation seconds: `unknown`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path adl/Cargo.toml four_plus_one; cargo fmt --manifest-path adl/Cargo.toml --check; focused installed journey commands selected after CLI integration

## Failure Semantics

- Fail closed on missing view evidence, mismatched revision, invalid references, or unrun required installed proof.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Worker 9 assigned and bound. No implementation or proof complete. No new approval pipeline; missing evidence remains explicit. Independent qualification #915 is not claimed.
