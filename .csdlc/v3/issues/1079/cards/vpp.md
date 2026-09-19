---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "deepseek-openrouter-reasoning-validation-plan"
issue: 1079
task_id: "issue-1079"
run_id: "issue-1079"
version: "1.0.5"
title: "[v0.92.2][Runtime][OpenRouter] Honor reasoning effort for DeepSeek review turns"
branch: "codex/1079-deepseek-openrouter-reasoning"
generated_at: "<timestamp>"
card_status: "ready"
status: "in_progress"
initial_pvf_lane: "runtime_provider_integration"
planned_pvf_lane: "focused_provider_and_runtime_integration_plus_bounded_live_qualification"
lane_registry_path: "docs/validation/pvf-lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "bounded_local_plus_single_live_provider_calls"
validation_resource_profile: "local_cpu_network_and_installed_runtime"
validation_family: "provider_request_contract_runtime_conversation_and_live_qualification"
validation_size_split: "focused_then_provider_suite_then_two_live_turns"
expected_proof_cost: "two bounded OpenRouter Runtime turns plus local tests"
planned_validation_seconds: "unknown"
planned_validation_tokens: "unknown"
issue_goal_ref: "Issue #1079 active Codex goal"
sprint_goal_ref: "not_assigned"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1079"
  - kind: "stp"
    ref: ".csdlc/issues/1079/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/1079/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/1079/cards/spp.md"
selected_lanes:
  - "provider-core request contract; Runtime authenticated conversation behavior; installed DeepSeek full-review and A2A qualification"
parallel_groups:
  - "Provider-core and Runtime local checks may run independently; installed live proof runs only after reviewed release installation."
validation_commands:
  - "cargo test --manifest-path adl-provider-core/Cargo.toml; cargo clippy --manifest-path adl-provider-core/Cargo.toml --all-targets -- -D warnings; cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib resident_agent_conversation_uses_canonical_agent_runtime_wss_ingress; native csdlc validate/review; installed Runtime review and A2A proof scripts"
failure_policy: "Fail closed on any request mismatch, card validation error, review finding, ambiguous provider result, profile drift, or missing peer continuation. Do not retry ambiguous live outcomes."
notes: "Known unrelated Runtime suite and Clippy baseline failures are recorded separately and must not be misattributed."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

<plan_summary>

## Lane Registry Inputs

- Registry path: `docs/validation/pvf-lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `runtime_provider_integration`
- Planned PVF lane for execution: `focused_provider_and_runtime_integration_plus_bounded_live_qualification`

## Selected Validation Lanes

- provider-core request contract; Runtime authenticated conversation behavior; installed DeepSeek full-review and A2A qualification

## Parallelization Plan

- Parallel groups: Provider-core and Runtime local checks may run independently; installed live proof runs only after reviewed release installation.
- Validation runtime class: `bounded_local_plus_single_live_provider_calls`
- Validation resource profile: `local_cpu_network_and_installed_runtime`
- Validation family: `provider_request_contract_runtime_conversation_and_live_qualification`
- Validation size split: `focused_then_provider_suite_then_two_live_turns`

## Goal Accounting Hooks

- Issue goal ref: `Issue #1079 active Codex goal`
- Sprint goal ref: `not_assigned`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `two bounded OpenRouter Runtime turns plus local tests`
- Planned validation seconds: `unknown`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path adl-provider-core/Cargo.toml; cargo clippy --manifest-path adl-provider-core/Cargo.toml --all-targets -- -D warnings; cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib resident_agent_conversation_uses_canonical_agent_runtime_wss_ingress; native csdlc validate/review; installed Runtime review and A2A proof scripts

## Failure Semantics

- Fail closed on any request mismatch, card validation error, review finding, ambiguous provider result, profile drift, or missing peer continuation. Do not retry ambiguous live outcomes.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Known unrelated Runtime suite and Clippy baseline failures are recorded separately and must not be misattributed.
