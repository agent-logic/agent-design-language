---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "bedrock-converse-resident-bindings-validation-plan"
issue: 1082
task_id: "issue-1082"
run_id: "issue-1082"
version: "1.0.5"
title: "[v0.92.2][Runtime][Bedrock] Adopt Converse and migrate unreliable OpenRouter residents"
branch: "codex/1082-bedrock-converse-resident-bindings"
generated_at: "2026-09-19"
card_status: "ready"
status: "ready"
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "local_and_bounded_live"
validation_resource_profile: "Local Rust CPU plus two one-attempt Bedrock calls and bounded Runtime conversations; no provisioning or recurring probe."
validation_family: "runtime_provider_identity"
validation_size_split: "Focused tests first, provider-core full library test and strict Clippy, Runtime component filters, then native proof and CI."
expected_proof_cost: "Two provider calls capped at 64 output tokens plus two resident conversations and one governed A2A exchange; stop on first failure for each provider."
planned_validation_seconds: "2400"
planned_validation_tokens: "unknown"
issue_goal_ref: "Issue #1082 implementation, bounded live qualification, exact-head review and green draft PR"
sprint_goal_ref: "v0.92.2 runtime provider follow-up"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1082"
  - kind: "stp"
    ref: ".csdlc/issues/1082/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/1082/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/1082/cards/spp.md"
selected_lanes:
  - "provider spec/profile materialization; Converse typed request/response; failure and response bounds; resident identity migration and continuity; bounded live exact-model inference; Runtime welcome/conversation/A2A/health/Observatory; docs and diff hygiene; independent review"
parallel_groups:
  - "Run independent deterministic crate tests in parallel when they do not share a Cargo lock; run cloud calls sequentially and never retry an ambiguous result."
validation_commands:
  - "cargo test --manifest-path adl-provider-core/Cargo.toml --lib; cargo clippy --manifest-path adl-provider-core/Cargo.toml --all-targets -- -D warnings; cargo test --manifest-path adl/Cargo.toml bedrock_converse_shapes_are_model_neutral --lib; cargo test --manifest-path adl-runtime-kernel/Cargo.toml agent_lifecycle --lib; cargo fmt checks; git diff --check; native csdlc proof validators; required CI"
failure_policy: "Fail closed on zero-test proof, unsupported or ignored provider control, wrong profile/account, oversized or malformed response, indistinguishable typed failure, unsafe retry, non-explicit identity rename, continuity loss, duplicate live identity, missing welcome digest, failed conversation/A2A/readiness projection, unresolved review finding or red CI."
notes: "Direct model success is separate from Runtime admission and resident proof; live provider state and repository implementation evidence must remain separate and truthful."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Prove deterministic provider and identity contracts locally, then use the smallest authorized live calls and Runtime interactions needed to establish exact-model and resident readiness.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`

## Selected Validation Lanes

- provider spec/profile materialization; Converse typed request/response; failure and response bounds; resident identity migration and continuity; bounded live exact-model inference; Runtime welcome/conversation/A2A/health/Observatory; docs and diff hygiene; independent review

## Parallelization Plan

- Parallel groups: Run independent deterministic crate tests in parallel when they do not share a Cargo lock; run cloud calls sequentially and never retry an ambiguous result.
- Validation runtime class: `local_and_bounded_live`
- Validation resource profile: `Local Rust CPU plus two one-attempt Bedrock calls and bounded Runtime conversations; no provisioning or recurring probe.`
- Validation family: `runtime_provider_identity`
- Validation size split: `Focused tests first, provider-core full library test and strict Clippy, Runtime component filters, then native proof and CI.`

## Goal Accounting Hooks

- Issue goal ref: `Issue #1082 implementation, bounded live qualification, exact-head review and green draft PR`
- Sprint goal ref: `v0.92.2 runtime provider follow-up`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Two provider calls capped at 64 output tokens plus two resident conversations and one governed A2A exchange; stop on first failure for each provider.`
- Planned validation seconds: `2400`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path adl-provider-core/Cargo.toml --lib; cargo clippy --manifest-path adl-provider-core/Cargo.toml --all-targets -- -D warnings; cargo test --manifest-path adl/Cargo.toml bedrock_converse_shapes_are_model_neutral --lib; cargo test --manifest-path adl-runtime-kernel/Cargo.toml agent_lifecycle --lib; cargo fmt checks; git diff --check; native csdlc proof validators; required CI

## Failure Semantics

- Fail closed on zero-test proof, unsupported or ignored provider control, wrong profile/account, oversized or malformed response, indistinguishable typed failure, unsafe retry, non-explicit identity rename, continuity loss, duplicate live identity, missing welcome digest, failed conversation/A2A/readiness projection, unresolved review finding or red CI.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Direct model success is separate from Runtime admission and resident proof; live provider state and repository implementation evidence must remain separate and truthful.
