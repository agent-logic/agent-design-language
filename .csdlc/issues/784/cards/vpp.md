---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "runtime-a2a-closed-loop-validation-plan"
issue: 784
task_id: "issue-0784"
run_id: "issue-0784"
version: "1.0.4"
title: "[v0.92.1][Runtime] Return completed A2A replies to the initiating agent"
branch: "codex/784-runtime-a2a-closed-loop"
generated_at: "2026-09-09T16:40:00Z"
card_status: "approved"
status: "ready"
initial_pvf_lane: "runtime-focused"
planned_pvf_lane: "runtime-focused-plus-demo"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.4"
validation_runtime_class: "bounded-runtime"
validation_resource_profile: "local-cpu-plus-existing-live-runtime-demo"
validation_family: "runtime-conversation-and-a2a"
validation_size_split: "focused-unit-adjacent-regression-live-demo"
expected_proof_cost: "bounded local validation and one live two-agent exchange"
planned_validation_seconds: "900"
planned_validation_tokens: "12000"
issue_goal_ref: "issue-784-session-goal"
sprint_goal_ref: "v0.92.1-closeout"
goal_metrics_rollup_ref: "v0.92.1-closeout"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/784"
  - kind: "stp"
    ref: ".csdlc/issues/784/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/784/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/784/cards/spp.md"
selected_lanes:
  - "focused Runtime A2A unit; adjacent conversation continuity; formatting and diff hygiene; bounded subagent review; live delegate/reply/synthesize demo"
parallel_groups:
  - "Run deterministic local tests together; serialize live Runtime proof after publication candidate is built."
validation_commands:
  - "cargo test --manifest-path adl-runtime-kernel/Cargo.toml agent_to_agent_model_action_from_conversation_delivers_peer_response; cargo test --manifest-path adl-runtime-kernel/Cargo.toml agent_to_agent; cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml -- --check; git diff --check"
failure_policy: "Fail closed on missing peer context, duplicate peer result, lost typed disposition, regression, formatting failure, or actionable review finding."
notes: "A history-only assertion is insufficient; the provider request for the initiator's next turn must contain the peer result exactly once."
---

Canonical Template Source: `docs/templates/prompts/1.0.4/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Prove the full local close-loop sequence and retain live proof for the exact candidate.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.4`
- Initial PVF lane from issue creation: `runtime-focused`
- Planned PVF lane for execution: `runtime-focused-plus-demo`

## Selected Validation Lanes

- focused Runtime A2A unit; adjacent conversation continuity; formatting and diff hygiene; bounded subagent review; live delegate/reply/synthesize demo

## Parallelization Plan

- Parallel groups: Run deterministic local tests together; serialize live Runtime proof after publication candidate is built.
- Validation runtime class: `bounded-runtime`
- Validation resource profile: `local-cpu-plus-existing-live-runtime-demo`
- Validation family: `runtime-conversation-and-a2a`
- Validation size split: `focused-unit-adjacent-regression-live-demo`

## Goal Accounting Hooks

- Issue goal ref: `issue-784-session-goal`
- Sprint goal ref: `v0.92.1-closeout`
- Goal metrics rollup ref: `v0.92.1-closeout`

## Proof Cost / Runtime Expectations

- Expected proof cost: `bounded local validation and one live two-agent exchange`
- Planned validation seconds: `900`
- Planned validation token budget: `12000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path adl-runtime-kernel/Cargo.toml agent_to_agent_model_action_from_conversation_delivers_peer_response; cargo test --manifest-path adl-runtime-kernel/Cargo.toml agent_to_agent; cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml -- --check; git diff --check

## Failure Semantics

- Fail closed on missing peer context, duplicate peer result, lost typed disposition, regression, formatting failure, or actionable review finding.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

A history-only assertion is insufficient; the provider request for the initiator's next turn must contain the peer result exactly once.
