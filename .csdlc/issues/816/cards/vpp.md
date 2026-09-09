---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "validation-integrity-validation-plan"
issue: 816
task_id: "issue-0816"
run_id: "issue-0816"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.11][quality] Repair redaction and hot-reload validation integrity"
branch: "codex/816-validation-integrity"
generated_at: "2026-09-09T20:50:00Z"
card_status: "ready"
status: "READY"
initial_pvf_lane: "runtime_tests"
planned_pvf_lane: "runtime_tests"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "small"
validation_resource_profile: "local CPU and filesystem; no cloud"
validation_family: "security_and_runtime_test_integrity"
validation_size_split: "two focused proof groups"
expected_proof_cost: "Small local shell and targeted Rust tests."
planned_validation_seconds: "not_collected"
planned_validation_tokens: "not_collected"
issue_goal_ref: "Issue #816 session goal"
sprint_goal_ref: "Parent #522"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/816"
  - kind: "stp"
    ref: ".csdlc/issues/816/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/816/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/816/cards/spp.md"
selected_lanes:
  - "security redaction validator; runtime targeted tests; diff hygiene"
parallel_groups:
  - "Redaction and Runtime proofs may run independently after implementation."
validation_commands:
  - "Focused validator command; targeted cargo test commands repeated twenty times; cargo fmt --check; git diff --check."
failure_policy: "Any false-positive fixture or intermittent run blocks publication and must be fixed."
notes: "Do not use fixed scheduler sleeps as causal proof."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Run publication-shaped redaction positives/negatives and repeated synchronized cancellation tests.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `runtime_tests`
- Planned PVF lane for execution: `runtime_tests`

## Selected Validation Lanes

- security redaction validator; runtime targeted tests; diff hygiene

## Parallelization Plan

- Parallel groups: Redaction and Runtime proofs may run independently after implementation.
- Validation runtime class: `small`
- Validation resource profile: `local CPU and filesystem; no cloud`
- Validation family: `security_and_runtime_test_integrity`
- Validation size split: `two focused proof groups`

## Goal Accounting Hooks

- Issue goal ref: `Issue #816 session goal`
- Sprint goal ref: `Parent #522`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Small local shell and targeted Rust tests.`
- Planned validation seconds: `not_collected`
- Planned validation token budget: `not_collected`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Focused validator command; targeted cargo test commands repeated twenty times; cargo fmt --check; git diff --check.

## Failure Semantics

- Any false-positive fixture or intermittent run blocks publication and must be fixed.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Do not use fixed scheduler sleeps as causal proof.
