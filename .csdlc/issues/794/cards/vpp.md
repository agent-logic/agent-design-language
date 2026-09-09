---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "native-typed-issue-close-validation-plan"
issue: 794
task_id: "issue-0794"
run_id: "issue-0794"
version: "1.0.4"
title: "[v0.92.2][tooling] Add native typed duplicate and no-op issue closure"
branch: "codex/794-native-typed-issue-close"
generated_at: "<timestamp>"
card_status: "ready"
status: "implemented_pending_review"
initial_pvf_lane: "<initial_pvf_lane>"
planned_pvf_lane: "csdlc-v3-github-issue-close-fake-adapter-and-cli"
lane_registry_path: "<lane_registry_path>"
lane_registry_template_set: "<lane_registry_template_set>"
validation_runtime_class: "local deterministic Rust tests and docs diff hygiene"
validation_resource_profile: "local CPU; fake GitHub adapters only; no live GitHub mutation required for focused proof"
validation_family: "native v3 GitHub issue mutation routing"
validation_size_split: "focused"
expected_proof_cost: "<expected_proof_cost>"
planned_validation_seconds: "<planned_validation_seconds>"
planned_validation_tokens: "<planned_validation_tokens>"
issue_goal_ref: "<issue_goal_ref>"
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "<goal_metrics_rollup_ref>"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/794"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
selected_lanes:
  - "issue_close filtered tests; operational_cli_commands integration; commands::remote::tests; command_manifest; git diff --check"
parallel_groups:
  - "<parallel_groups_inline>"
validation_commands:
  - "`cargo fmt --manifest-path csdlc-v3/Cargo.toml`; `cargo test --manifest-path csdlc-v3/Cargo.toml issue_close -- --nocapture`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands -- --nocapture`; `cargo test --manifest-path csdlc-v3/Cargo.toml commands::remote::tests -- --nocapture`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest -- --nocapture`; `cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings`; native validate/doctor; `git diff --check`"
failure_policy: "Fail on any route-family mismatch, unsafe close classification, missing authenticated readback, replay mutation, CLI/request divergence, or undocumented operator surface."
notes: "<notes_risks_inline>"
---

Canonical Template Source: `docs/templates/prompts/1.0.4/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

<plan_summary>

## Lane Registry Inputs

- Registry path: `<lane_registry_path>`
- Registry template set: `<lane_registry_template_set>`
- Initial PVF lane from issue creation: `<initial_pvf_lane>`
- Planned PVF lane for execution: `csdlc-v3-github-issue-close-fake-adapter-and-cli`

## Selected Validation Lanes

- issue_close filtered tests; operational_cli_commands integration; commands::remote::tests; command_manifest; git diff --check

## Parallelization Plan

- Parallel groups: <parallel_groups_inline>
- Validation runtime class: `local deterministic Rust tests and docs diff hygiene`
- Validation resource profile: `local CPU; fake GitHub adapters only; no live GitHub mutation required for focused proof`
- Validation family: `native v3 GitHub issue mutation routing`
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

- `cargo fmt --manifest-path csdlc-v3/Cargo.toml`; `cargo test --manifest-path csdlc-v3/Cargo.toml issue_close -- --nocapture`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands -- --nocapture`; `cargo test --manifest-path csdlc-v3/Cargo.toml commands::remote::tests -- --nocapture`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest -- --nocapture`; `cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings`; native validate/doctor; `git diff --check`

## Failure Semantics

- Fail on any route-family mismatch, unsafe close classification, missing authenticated readback, replay mutation, CLI/request divergence, or undocumented operator surface.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

<notes_risks_inline>
