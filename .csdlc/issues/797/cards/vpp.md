---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "issue-metadata-validation-plan"
issue: 797
task_id: "issue-0797"
run_id: "issue-0797"
version: "v0.92.1"
title: "[C-SDLC v3] Support existing-issue label and milestone updates"
branch: "codex/797-issue-metadata"
generated_at: "2026-09-09T18:56:10.034970+00:00"
card_status: "ready"
status: "IN_PROGRESS"
initial_pvf_lane: "owner_binary"
planned_pvf_lane: "owner_binary"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.4"
validation_runtime_class: "small"
validation_resource_profile: "local CPU, filesystem and Git; no cloud"
validation_family: "native_remote_owner_contract"
validation_size_split: "single focused local proof"
expected_proof_cost: "Small local Rust build and fake-transport tests; no runtime or cloud suite."
planned_validation_seconds: "not_collected"
planned_validation_tokens: "not_collected"
issue_goal_ref: "Issue #797 session goal in current Codex task"
sprint_goal_ref: "not_applicable: standalone defect"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/797"
  - kind: "stp"
    ref: ".csdlc/issues/797/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/797/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/797/cards/spp.md"
selected_lanes:
  - "owner_binary: focused remote owner and operational CLI tests"
parallel_groups:
  - "Serialized native edits and validation; independent review after proof"
validation_commands:
  - "cargo test --locked --manifest-path csdlc-v3/Cargo.toml --lib commands::remote::tests; cargo test --locked --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands; cargo clippy --locked --manifest-path csdlc-v3/Cargo.toml --lib --bin csdlc -- -D warnings; native six-card validation"
failure_policy: "Resolve failures before publication; never retry metadata writes on readback drift."
notes: "API writes are not conditional transactions; stale readback fails closed. IssueEdit explicit recovery does not replay PATCH."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Required native-owner contract proof with deterministic fake transports, exact request/PATCH/readback assertions and restart simulations.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.4`
- Initial PVF lane from issue creation: `owner_binary`
- Planned PVF lane for execution: `owner_binary`

## Selected Validation Lanes

- owner_binary: focused remote owner and operational CLI tests

## Parallelization Plan

- Parallel groups: Serialized native edits and validation; independent review after proof
- Validation runtime class: `small`
- Validation resource profile: `local CPU, filesystem and Git; no cloud`
- Validation family: `native_remote_owner_contract`
- Validation size split: `single focused local proof`

## Goal Accounting Hooks

- Issue goal ref: `Issue #797 session goal in current Codex task`
- Sprint goal ref: `not_applicable: standalone defect`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Small local Rust build and fake-transport tests; no runtime or cloud suite.`
- Planned validation seconds: `not_collected`
- Planned validation token budget: `not_collected`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --locked --manifest-path csdlc-v3/Cargo.toml --lib commands::remote::tests; cargo test --locked --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands; cargo clippy --locked --manifest-path csdlc-v3/Cargo.toml --lib --bin csdlc -- -D warnings; native six-card validation

## Failure Semantics

- Resolve failures before publication; never retry metadata writes on readback drift.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

API writes are not conditional transactions; stale readback fails closed. IssueEdit explicit recovery does not replay PATCH.
