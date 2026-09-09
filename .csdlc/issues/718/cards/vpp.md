---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "a2a-canonical-name-routing-validation-plan"
issue: 718
task_id: "issue-0718"
run_id: "issue-0718"
version: "1.0.4"
title: "[v0.92.1][Runtime] Route agent-to-agent communication by canonical agent name"
branch: "codex/718-a2a-canonical-name-routing"
generated_at: "2026-09-09T18:20:00Z"
card_status: "approved"
status: "ready"
initial_pvf_lane: "runtime-focused"
planned_pvf_lane: "runtime-focused-plus-governed-demo"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.4"
validation_runtime_class: "bounded-runtime"
validation_resource_profile: "local-cpu-plus-existing-live-runtime-demo"
validation_family: "runtime-a2a-canonical-identity"
validation_size_split: "focused-unit-integration-contract-lifecycle-demo"
expected_proof_cost: "bounded local Runtime tests and one governed all-pairs demonstration"
planned_validation_seconds: "1200"
planned_validation_tokens: "18000"
issue_goal_ref: "issue-718-session-goal"
sprint_goal_ref: "v0.92.1-bugfix"
goal_metrics_rollup_ref: "v0.92.1-bugfix"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/718"
  - kind: "stp"
    ref: ".csdlc/issues/718/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/718/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/718/cards/spp.md"
selected_lanes:
  - "canonical resolver and denial unit tests; production conversation-to-A2A integration; legacy compatibility; history and lifecycle preservation; OpenAPI/prompt/Welcome Package contracts; all-pairs resident routing; formatting and review"
parallel_groups:
  - "Run deterministic local test filters together after implementation; serialize production-path demo and exact-head review on the stable candidate."
validation_commands:
  - "cargo test --manifest-path adl-runtime-kernel/Cargo.toml agent_to_agent; cargo test --manifest-path adl-runtime-kernel/Cargo.toml canonical_name; cargo test --manifest-path adl-runtime-kernel/Cargo.toml openapi; cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --all -- --check; git diff --check"
failure_policy: "Fail closed on ambiguity, unknown or malformed names, self-targeting, internal-ID leakage, altered signed ACIP semantics, lifecycle address drift, helper-only proof, regression, or actionable review finding."
notes: "A resolver-only test is insufficient; at least one production conversation action and the ordinary resident-pair matrix must cross the signed A2A path."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Prove canonical civic identity is the uniform public A2A address while internal identity remains stable and private behind governed dispatch.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.4`
- Initial PVF lane from issue creation: `runtime-focused`
- Planned PVF lane for execution: `runtime-focused-plus-governed-demo`

## Selected Validation Lanes

- canonical resolver and denial unit tests; production conversation-to-A2A integration; legacy compatibility; history and lifecycle preservation; OpenAPI/prompt/Welcome Package contracts; all-pairs resident routing; formatting and review

## Parallelization Plan

- Parallel groups: Run deterministic local test filters together after implementation; serialize production-path demo and exact-head review on the stable candidate.
- Validation runtime class: `bounded-runtime`
- Validation resource profile: `local-cpu-plus-existing-live-runtime-demo`
- Validation family: `runtime-a2a-canonical-identity`
- Validation size split: `focused-unit-integration-contract-lifecycle-demo`

## Goal Accounting Hooks

- Issue goal ref: `issue-718-session-goal`
- Sprint goal ref: `v0.92.1-bugfix`
- Goal metrics rollup ref: `v0.92.1-bugfix`

## Proof Cost / Runtime Expectations

- Expected proof cost: `bounded local Runtime tests and one governed all-pairs demonstration`
- Planned validation seconds: `1200`
- Planned validation token budget: `18000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path adl-runtime-kernel/Cargo.toml agent_to_agent; cargo test --manifest-path adl-runtime-kernel/Cargo.toml canonical_name; cargo test --manifest-path adl-runtime-kernel/Cargo.toml openapi; cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --all -- --check; git diff --check

## Failure Semantics

- Fail closed on ambiguity, unknown or malformed names, self-targeting, internal-ID leakage, altered signed ACIP semantics, lifecycle address drift, helper-only proof, regression, or actionable review finding.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

A resolver-only test is insufficient; at least one production conversation action and the ordinary resident-pair matrix must cross the signed A2A path.
