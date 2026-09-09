---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "dynamic-agent-health-task-failures-validation-plan"
issue: 759
task_id: "issue-0759"
run_id: "issue-0759"
version: "1.0.4"
title: "[v0.92.1][TAIL-06.03][runtime] Isolate dynamic-agent health task failures"
branch: "codex/759-dynamic-agent-health-task-failures"
generated_at: "<timestamp>"
card_status: "ready"
status: "ready"
initial_pvf_lane: "runtime-focused"
planned_pvf_lane: "runtime-focused-defect-regression"
lane_registry_path: "issue #759 local PVF classification"
lane_registry_template_set: "docs/templates/prompts/1.0.4"
validation_runtime_class: "Rust runtime crate"
validation_resource_profile: "local deterministic async tests; no live provider calls"
validation_family: "runtime dynamic-agent health sweep"
validation_size_split: "focused first, strict clippy/check before review, CI after publication"
expected_proof_cost: "medium"
planned_validation_seconds: "1800"
planned_validation_tokens: "unknown"
issue_goal_ref: "Codex goal: Issue #759 dynamic-agent health sweep remediation"
sprint_goal_ref: "v0.92.1 closeout tail runtime defect lane"
goal_metrics_rollup_ref: "v0.92.1"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/759"
  - kind: "stp"
    ref: ".csdlc/issues/759/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/759/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/759/cards/spp.md"
selected_lanes:
  - "focused Rust Runtime dynamic-agent health regression; Runtime fmt; strict clippy/check for touched crate; hosted CI after publish"
parallel_groups:
  - "Run focused tests serially for deterministic async behavior; fmt/clippy may run after source stabilizes."
validation_commands:
  - "`cargo test --manifest-path adl-runtime-kernel/Cargo.toml <focused dynamic-agent health filter> -- --nocapture`; `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml -- --check`; strict clippy/check command selected after final touched surface is known."
failure_policy: "Any focused Runtime regression, fmt, or strict clippy/check failure blocks review and publication until repaired or truthfully routed as an external pre-existing failure with isolated evidence."
notes: "The regression must not rely on live Ollama or network availability; inject or simulate task failure deterministically."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

<plan_summary>

## Lane Registry Inputs

- Registry path: `issue #759 local PVF classification`
- Registry template set: `docs/templates/prompts/1.0.4`
- Initial PVF lane from issue creation: `runtime-focused`
- Planned PVF lane for execution: `runtime-focused-defect-regression`

## Selected Validation Lanes

- focused Rust Runtime dynamic-agent health regression; Runtime fmt; strict clippy/check for touched crate; hosted CI after publish

## Parallelization Plan

- Parallel groups: Run focused tests serially for deterministic async behavior; fmt/clippy may run after source stabilizes.
- Validation runtime class: `Rust runtime crate`
- Validation resource profile: `local deterministic async tests; no live provider calls`
- Validation family: `runtime dynamic-agent health sweep`
- Validation size split: `focused first, strict clippy/check before review, CI after publication`

## Goal Accounting Hooks

- Issue goal ref: `Codex goal: Issue #759 dynamic-agent health sweep remediation`
- Sprint goal ref: `v0.92.1 closeout tail runtime defect lane`
- Goal metrics rollup ref: `v0.92.1`

## Proof Cost / Runtime Expectations

- Expected proof cost: `medium`
- Planned validation seconds: `1800`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml <focused dynamic-agent health filter> -- --nocapture`; `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml -- --check`; strict clippy/check command selected after final touched surface is known.

## Failure Semantics

- Any focused Runtime regression, fmt, or strict clippy/check failure blocks review and publication until repaired or truthfully routed as an external pre-existing failure with isolated evidence.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

The regression must not rely on live Ollama or network availability; inject or simulate task failure deterministically.
