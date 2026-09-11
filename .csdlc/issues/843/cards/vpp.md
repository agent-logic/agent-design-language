---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "legacy-ready-intent-recovery-validation-plan"
issue: 843
task_id: "issue-0843"
run_id: "issue-0843"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06.23][tooling] Recover retained ready intents without target identity"
branch: "codex/843-legacy-ready-intent-recovery"
generated_at: "2026-09-11T15:33:13Z"
card_status: "ready"
status: "READY"
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "small"
validation_resource_profile: "local CPU/filesystem; fake transport"
validation_family: "github_mutation_reconciliation"
validation_size_split: "focused tests plus current-source validator"
expected_proof_cost: "Small focused tooling proof."
planned_validation_seconds: "unknown"
planned_validation_tokens: "unknown"
issue_goal_ref: "Active #835 expanded goal, including required #843 repair; no independent completion claim."
sprint_goal_ref: "Parent #522"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/843"
  - kind: "stp"
    ref: ".csdlc/issues/843/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/843/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/843/cards/spp.md"
selected_lanes:
  - "tooling"
parallel_groups:
  - "Focused tests followed by current-source proof."
validation_commands:
  - "cargo test --locked --manifest-path csdlc-v3/Cargo.toml (clean detached source); cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings; current V3-F validator and --negative; native card validation; git diff --check"
failure_policy: "Any missing, mismatched, stale, or uncertain target identity blocks dispatch and publication."
notes: "A legacy record cannot self-authorize a new target."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Deterministic local proof of matching legacy recovery and fail-closed mismatches before current-source validation.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`

## Selected Validation Lanes

- tooling

## Parallelization Plan

- Parallel groups: Focused tests followed by current-source proof.
- Validation runtime class: `small`
- Validation resource profile: `local CPU/filesystem; fake transport`
- Validation family: `github_mutation_reconciliation`
- Validation size split: `focused tests plus current-source validator`

## Goal Accounting Hooks

- Issue goal ref: `Active #835 expanded goal, including required #843 repair; no independent completion claim.`
- Sprint goal ref: `Parent #522`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Small focused tooling proof.`
- Planned validation seconds: `unknown`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --locked --manifest-path csdlc-v3/Cargo.toml (clean detached source); cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings; current V3-F validator and --negative; native card validation; git diff --check

## Failure Semantics

- Any missing, mismatched, stale, or uncertain target identity blocks dispatch and publication.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

A legacy record cannot self-authorize a new target.
