---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-server-review-model-access-validation-plan"
issue: 1056
task_id: "issue-1056"
run_id: "issue-1056"
version: "v0.92.2"
title: "[v0.92.2][CF-SERVER] Execute hosted reviews and provide governed model access"
branch: "codex/1056-v0922-server-review-model-access"
generated_at: "2026-09-16T20:16:52.878587+00:00"
card_status: "ready"
status: "pre_execution"
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "local_and_authorized_remote"
validation_resource_profile: "Bounded deterministic CPU fixtures; separately authorized installed/browser/provider proof with pinned resource limits."
validation_family: "runtime"
validation_size_split: "small"
expected_proof_cost: "Initial estimate 3600 seconds; revise after interface and environment inspection before execution."
planned_validation_seconds: "3600"
planned_validation_tokens: "3000"
issue_goal_ref: "Create issue #1056 goal after native bind, before implementation; Sprint #936 prerequisite"
sprint_goal_ref: "https://github.com/agent-logic/agent-design-language/issues/936"
goal_metrics_rollup_ref: "Future #1056 SOR and child evidence register"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1056"
  - kind: "stp"
    ref: ".csdlc/issues/1056/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/1056/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/1056/cards/spp.md"
selected_lanes:
  - "runtime component and built-binary loopback; provider live acceptance pending deployment and spend approval"
parallel_groups:
  - "Run disjoint deterministic tests in parallel; serialize shared service, installation and model-provider resources."
validation_commands:
  - "cargo test --manifest-path adl/Cargo.toml --test codefriend_server; cargo clippy --manifest-path adl/Cargo.toml --bin codefriend-server --test codefriend_server -- -D warnings; rustfmt --check --edition 2021 on changed Rust files; git diff --check. Test proof includes actual built server subprocess and provider adapter with loopback fixture responses. Real deployed/provider acceptance remains required and not run."
failure_policy: "Required positive and negative feature acceptance must execute with nonzero evidence; missing real product proof blocks completion."
notes: "Coordinate accepted shared Runtime/provider/evidence and review consumer interfaces. Pin repository ownership and design before implementation; no live or paid operations during preparation."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Inspect and accept shared interfaces and repository ownership; implement the complete bounded feature; run component positives/negatives and separately authorized real product proof; record exact evidence and obtain independent review; publish without automatic merge. Coordinate accepted shared Runtime/provider/evidence and review consumer interfaces. Pin repository ownership and design before implementation; no live or paid operations during preparation.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`

## Selected Validation Lanes

- runtime component and built-binary loopback; provider live acceptance pending deployment and spend approval

## Parallelization Plan

- Parallel groups: Run disjoint deterministic tests in parallel; serialize shared service, installation and model-provider resources.
- Validation runtime class: `local_and_authorized_remote`
- Validation resource profile: `Bounded deterministic CPU fixtures; separately authorized installed/browser/provider proof with pinned resource limits.`
- Validation family: `runtime`
- Validation size split: `small`

## Goal Accounting Hooks

- Issue goal ref: `Create issue #1056 goal after native bind, before implementation; Sprint #936 prerequisite`
- Sprint goal ref: `https://github.com/agent-logic/agent-design-language/issues/936`
- Goal metrics rollup ref: `Future #1056 SOR and child evidence register`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Initial estimate 3600 seconds; revise after interface and environment inspection before execution.`
- Planned validation seconds: `3600`
- Planned validation token budget: `3000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path adl/Cargo.toml --test codefriend_server; cargo clippy --manifest-path adl/Cargo.toml --bin codefriend-server --test codefriend_server -- -D warnings; rustfmt --check --edition 2021 on changed Rust files; git diff --check. Test proof includes actual built server subprocess and provider adapter with loopback fixture responses. Real deployed/provider acceptance remains required and not run.

## Failure Semantics

- Required positive and negative feature acceptance must execute with nonzero evidence; missing real product proof blocks completion.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Coordinate accepted shared Runtime/provider/evidence and review consumer interfaces. Pin repository ownership and design before implementation; no live or paid operations during preparation.
