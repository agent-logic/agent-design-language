---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "issue-967-deterministic-hosted-a2a-validation-plan"
issue: 967
task_id: "issue-0967"
run_id: "issue-0967"
version: "v0.92.2"
title: "[v0.92.2][RT-PROVIDER][corrective] Make hosted A2A initiation deterministic across provider output formats"
branch: "codex/967-deterministic-hosted-a2a"
generated_at: "2026-09-12"
card_status: "ready"
status: "in_progress"
initial_pvf_lane: "provider"
planned_pvf_lane: "provider"
lane_registry_path: "docs/templates/prompts/1.0.5/pvf_lane_policy.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "bounded_local_and_authorized_hosted"
validation_resource_profile: "CPU and loopback fixtures; hosted inference only with explicit authorization and existing bounds"
validation_family: "provider"
validation_size_split: "focused Runtime/OpenAPI; installed local matrices; separate hosted acceptance"
expected_proof_cost: "Local fixtures have zero paid calls; hosted cost bounded by separately approved run"
planned_validation_seconds: "unknown"
planned_validation_tokens: "unknown"
issue_goal_ref: "issue-967"
sprint_goal_ref: "issue-928"
goal_metrics_rollup_ref: "unknown"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/967"
  - kind: "stp"
    ref: ".csdlc/issues/967/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/967/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/967/cards/spp.md"
selected_lanes:
  - "provider: focused production dispatch, OpenAPI, zero-paid installed matrices and separately authorized hosted acceptance"
parallel_groups:
  - "Independent read-only review and contract inspection may run in parallel; shared Cargo target and hosted inference runs remain coordinated and bounded"
validation_commands:
  - "CARGO_TARGET_DIR=adl/target cargo test --locked --offline --manifest-path adl-runtime-kernel/Cargo.toml --lib sixth_registered_provider_uses_real_canonical_a2a_dispatch; CARGO_TARGET_DIR=adl/target cargo test --locked --offline --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract; bounded zero-paid lifecycle harness using typed requested_agent_action. Exact-head installed fixture command and paid command must be recorded with their source/binary identities before execution."
failure_policy: "Fail closed on invalid actions, conflicting actions, stale exact-head evidence, red CI or missing hosted acceptance. Preserve failed attempts; no silent or paid retry."
notes: "Preserve failed hosted-live-03 report SHA256 651f9a00fb5b96e2a6b0539d1f9321c4546631a16b78e9c723fe823c0d95d439: OpenAI five successful calls, Anthropic two successful calls, Vertex zero. Raw provider output was not retained; exact response shape is unknown. Commit corrective source under #967, rebuild exact-head binaries, retain and review exact proof, pass current CI, and obtain fresh authorization before one bounded hosted acceptance run. No hosted retry or merge is claimed."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Carry the bounded typed-action repair onto the merged baseline, preserve action and replay invariants, align schema, then qualify exact-head local and separately authorized hosted proof before publication acceptance.

## Lane Registry Inputs

- Registry path: `docs/templates/prompts/1.0.5/pvf_lane_policy.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `provider`
- Planned PVF lane for execution: `provider`

## Selected Validation Lanes

- provider: focused production dispatch, OpenAPI, zero-paid installed matrices and separately authorized hosted acceptance

## Parallelization Plan

- Parallel groups: Independent read-only review and contract inspection may run in parallel; shared Cargo target and hosted inference runs remain coordinated and bounded
- Validation runtime class: `bounded_local_and_authorized_hosted`
- Validation resource profile: `CPU and loopback fixtures; hosted inference only with explicit authorization and existing bounds`
- Validation family: `provider`
- Validation size split: `focused Runtime/OpenAPI; installed local matrices; separate hosted acceptance`

## Goal Accounting Hooks

- Issue goal ref: `issue-967`
- Sprint goal ref: `issue-928`
- Goal metrics rollup ref: `unknown`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Local fixtures have zero paid calls; hosted cost bounded by separately approved run`
- Planned validation seconds: `unknown`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- CARGO_TARGET_DIR=adl/target cargo test --locked --offline --manifest-path adl-runtime-kernel/Cargo.toml --lib sixth_registered_provider_uses_real_canonical_a2a_dispatch; CARGO_TARGET_DIR=adl/target cargo test --locked --offline --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract; bounded zero-paid lifecycle harness using typed requested_agent_action. Exact-head installed fixture command and paid command must be recorded with their source/binary identities before execution.

## Failure Semantics

- Fail closed on invalid actions, conflicting actions, stale exact-head evidence, red CI or missing hosted acceptance. Preserve failed attempts; no silent or paid retry.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Preserve failed hosted-live-03 report SHA256 651f9a00fb5b96e2a6b0539d1f9321c4546631a16b78e9c723fe823c0d95d439: OpenAI five successful calls, Anthropic two successful calls, Vertex zero. Raw provider output was not retained; exact response shape is unknown. Commit corrective source under #967, rebuild exact-head binaries, retain and review exact proof, pass current CI, and obtain fresh authorization before one bounded hosted acceptance run. No hosted retry or merge is claimed.
