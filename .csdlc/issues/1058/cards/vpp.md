---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-installed-local-review-agent-validation-plan"
issue: 1058
task_id: "issue-1058"
run_id: "issue-1058"
version: "v0.92.2"
title: "[v0.92.2][CF-AGENT] Run website-controlled reviews through an installed local agent"
branch: "codex/1058-v0922-installed-local-review-agent"
generated_at: "2026-09-16T20:16:52.878587+00:00"
card_status: "ready"
status: "execution"
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "local_and_authorized_remote"
validation_resource_profile: "Bounded deterministic CPU fixtures; separately authorized installed/browser/provider proof with pinned resource limits."
validation_family: "runtime"
validation_size_split: "small"
expected_proof_cost: "Focused local suite plus separately authorized live qualification; initial local validation estimate 3600 seconds."
planned_validation_seconds: "3600"
planned_validation_tokens: "3000"
issue_goal_ref: "Active Sprint 10 #936 goal explicitly requested by operator"
sprint_goal_ref: "https://github.com/agent-logic/agent-design-language/issues/936"
goal_metrics_rollup_ref: "Future #1058 SOR and child evidence register"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1058"
  - kind: "stp"
    ref: ".csdlc/issues/1058/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/1058/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/1058/cards/spp.md"
selected_lanes:
  - "runtime, provider"
parallel_groups:
  - "Run disjoint deterministic tests in parallel; serialize shared service, installation and model-provider resources."
validation_commands:
  - "cargo test --manifest-path adl/Cargo.toml --test codefriend_agent --test codefriend_review; cargo clippy --manifest-path adl/Cargo.toml --bin codefriend-agent --test codefriend_agent -- -D warnings. Separate real installed website/model journeys remain mandatory and not yet executed."
failure_policy: "Required positive and negative feature acceptance must execute with nonzero evidence; missing real product proof blocks completion."
notes: "Sprint #936 goal remains active by explicit operator instruction; no replacement issue goal. Implement locally now. No paid calls, deployment or merge authority inferred. Website/agent credentials are private secrets even though they are not provider keys; never print them."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Validate the installed agent local component, existing shared runner and restricted model gateway protocol. Preserve separate live website/provider/platform acceptance.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`

## Selected Validation Lanes

- runtime, provider

## Parallelization Plan

- Parallel groups: Run disjoint deterministic tests in parallel; serialize shared service, installation and model-provider resources.
- Validation runtime class: `local_and_authorized_remote`
- Validation resource profile: `Bounded deterministic CPU fixtures; separately authorized installed/browser/provider proof with pinned resource limits.`
- Validation family: `runtime`
- Validation size split: `small`

## Goal Accounting Hooks

- Issue goal ref: `Active Sprint 10 #936 goal explicitly requested by operator`
- Sprint goal ref: `https://github.com/agent-logic/agent-design-language/issues/936`
- Goal metrics rollup ref: `Future #1058 SOR and child evidence register`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Focused local suite plus separately authorized live qualification; initial local validation estimate 3600 seconds.`
- Planned validation seconds: `3600`
- Planned validation token budget: `3000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path adl/Cargo.toml --test codefriend_agent --test codefriend_review; cargo clippy --manifest-path adl/Cargo.toml --bin codefriend-agent --test codefriend_agent -- -D warnings. Separate real installed website/model journeys remain mandatory and not yet executed.

## Failure Semantics

- Required positive and negative feature acceptance must execute with nonzero evidence; missing real product proof blocks completion.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Sprint #936 goal remains active by explicit operator instruction; no replacement issue goal. Implement locally now. No paid calls, deployment or merge authority inferred. Website/agent credentials are private secrets even though they are not provider keys; never print them.
