---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-mlx-metal-provider-validation-plan"
issue: 903
task_id: "issue-0903"
run_id: "issue-0903"
version: "0.92.2"
title: "[v0.92.2][PLAT-MLX] Bounded MLX and Apple Metal provider adapter"
branch: "codex/903-v0922-mlx-metal-provider"
generated_at: "2026-09-12T00:18:14.454312+00:00"
card_status: "ready"
status: "planned"
initial_pvf_lane: "provider"
planned_pvf_lane: "provider"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "bounded_local"
validation_resource_profile: "local CPU/Rust fixture proof; separate approved bounded Apple-silicon GPU/memory model smoke; availability and resource authority not yet established"
validation_family: "mlx_provider_contract_and_real_metal_smoke"
validation_size_split: "focused per source acceptance; no reflexive full workspace suite"
expected_proof_cost: "1800 seconds and 5000 tokens estimated local deterministic proof; separately estimate and approve real hardware smoke resources before execution"
planned_validation_seconds: "1800"
planned_validation_tokens: "5000"
issue_goal_ref: "not_created; create issue-bound goal before implementation"
sprint_goal_ref: "issue-932; Sprint 6 setup and coordination"
goal_metrics_rollup_ref: ".csdlc/evidence/903/goal-metrics.json (planned; absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/903"
  - kind: "stp"
    ref: ".csdlc/issues/903/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/903/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/903/cards/spp.md"
selected_lanes:
  - "provider; Provider lane: deterministic local adapter contract and integration fixtures, plus separately classified observational actual MLX/Metal smoke. Selected new test path adl/tests/mlx_provider.rs is planned, not existing proof. After authoring, run cargo test --manifest-path adl/Cargo.toml --test mlx_provider and focused registered provider-profile/reload/cost/Runtime-dispatch regressions; enumerate exact test names first and record nonzero scenarios. Run cargo fmt --manifest-path adl/Cargo.toml --check and git diff --check. Couple fixture metadata for lane, role, determinism, CPU/GPU/memory/process resources and required milestone gate. Test invalid definitions, missing model/service, malformed output, timeout/cancel settling, explicit unsupported-platform errors without fallback, redaction and stdout/stderr separation; test compatibility logging if exposed. Real proof must invoke the registered production Runtime path on supported Apple hardware, pin model and MLX/Metal/macOS/candidate identities and retain nonempty output plus bounded timing/resources. Mocks prove negatives only. Exact operational smoke command must be documented/tested after actual #876 API and approved available MLX transport are resolved; no invented CLI syntax or generic local-model support claim. Required CI and actual-platform proof remain separate, mandatory, and not run."
parallel_groups:
  - "Future disjoint adapter/harness authoring may run in parallel after execution authorization. Serialize shared provider registration edits and any use of the same accelerator/models; no hardware runs during setup."
validation_commands:
  - "Provider lane: deterministic local adapter contract and integration fixtures, plus separately classified observational actual MLX/Metal smoke. Selected new test path adl/tests/mlx_provider.rs is planned, not existing proof. After authoring, run cargo test --manifest-path adl/Cargo.toml --test mlx_provider and focused registered provider-profile/reload/cost/Runtime-dispatch regressions; enumerate exact test names first and record nonzero scenarios. Run cargo fmt --manifest-path adl/Cargo.toml --check and git diff --check. Couple fixture metadata for lane, role, determinism, CPU/GPU/memory/process resources and required milestone gate. Test invalid definitions, missing model/service, malformed output, timeout/cancel settling, explicit unsupported-platform errors without fallback, redaction and stdout/stderr separation; test compatibility logging if exposed. Real proof must invoke the registered production Runtime path on supported Apple hardware, pin model and MLX/Metal/macOS/candidate identities and retain nonempty output plus bounded timing/resources. Mocks prove negatives only. Exact operational smoke command must be documented/tested after actual #876 API and approved available MLX transport are resolved; no invented CLI syntax or generic local-model support claim. Required CI and actual-platform proof remain separate, mandatory, and not run."
failure_policy: "Required failures, skipped or zero-test proof block acceptance. Preserve guards; record durable anomalies; repair and rerun affected proof and independent exact-head review. CI evidence is separate from local proof."
notes: "#876 CLOSED; PR #953 MERGED at b6d110c84e11253b392d0bb078f2fb33a36b9a0c, ancestor of selected main. Setup only under Sprint 6 umbrella #932; no implementation, model loading/download, provider/service mutation, paid allocation or hardware experiment is authorized by this preparation. All 69 startup gate and #864 are accepted. Keep implementation steps pending and require each future worker to create its child-bound execution goal. Local Apple M4 Pro 64 GiB and mlx 0.32.0 observed by root. mlx-lm absent from default Python; complete model snapshot and compatible serving transport not yet verified. These facts establish platform candidacy, not an executable MLX Runtime route. Read-only cache candidate: mlx-community/Llama-3.2-3B-Instruct-4bit snapshot 7f0dc925e0d0afb0322d96f9255cfddf2ba5636e; 6 files, one safetensors of 1,807,496,278 bytes, config/tokenizer present, no broken links observed. Presence only, not load, shard-completeness or compatibility proof; candidate remains unselected."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Verify accepted #876 editable-definition and reload/cost contracts, inspect current local provider conventions and resolve shared provider owners; pin an available compatible model, MLX/Metal versions, Apple-silicon/macOS environment and approved CPU/GPU/memory bounds before real execution; implement one canonical-definition-consuming MLX adapter and register it through Runtime production dispatch without duplicating registry/reload policy; execute invalid definition, missing service/model, malformed output, timeout/cancellation and unsupported-platform negatives with redacted diagnostics; run separately identified real supported-hardware nonempty-generation smoke through Runtime, retain input/output/candidate/version/resource identity and focused CI, document exact prerequisites and limitations, then independently review exact implementation head.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `provider`
- Planned PVF lane for execution: `provider`

## Selected Validation Lanes

- provider; Provider lane: deterministic local adapter contract and integration fixtures, plus separately classified observational actual MLX/Metal smoke. Selected new test path adl/tests/mlx_provider.rs is planned, not existing proof. After authoring, run cargo test --manifest-path adl/Cargo.toml --test mlx_provider and focused registered provider-profile/reload/cost/Runtime-dispatch regressions; enumerate exact test names first and record nonzero scenarios. Run cargo fmt --manifest-path adl/Cargo.toml --check and git diff --check. Couple fixture metadata for lane, role, determinism, CPU/GPU/memory/process resources and required milestone gate. Test invalid definitions, missing model/service, malformed output, timeout/cancel settling, explicit unsupported-platform errors without fallback, redaction and stdout/stderr separation; test compatibility logging if exposed. Real proof must invoke the registered production Runtime path on supported Apple hardware, pin model and MLX/Metal/macOS/candidate identities and retain nonempty output plus bounded timing/resources. Mocks prove negatives only. Exact operational smoke command must be documented/tested after actual #876 API and approved available MLX transport are resolved; no invented CLI syntax or generic local-model support claim. Required CI and actual-platform proof remain separate, mandatory, and not run.

## Parallelization Plan

- Parallel groups: Future disjoint adapter/harness authoring may run in parallel after execution authorization. Serialize shared provider registration edits and any use of the same accelerator/models; no hardware runs during setup.
- Validation runtime class: `bounded_local`
- Validation resource profile: `local CPU/Rust fixture proof; separate approved bounded Apple-silicon GPU/memory model smoke; availability and resource authority not yet established`
- Validation family: `mlx_provider_contract_and_real_metal_smoke`
- Validation size split: `focused per source acceptance; no reflexive full workspace suite`

## Goal Accounting Hooks

- Issue goal ref: `not_created; create issue-bound goal before implementation`
- Sprint goal ref: `issue-932; Sprint 6 setup and coordination`
- Goal metrics rollup ref: `.csdlc/evidence/903/goal-metrics.json (planned; absent until execution)`

## Proof Cost / Runtime Expectations

- Expected proof cost: `1800 seconds and 5000 tokens estimated local deterministic proof; separately estimate and approve real hardware smoke resources before execution`
- Planned validation seconds: `1800`
- Planned validation token budget: `5000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Provider lane: deterministic local adapter contract and integration fixtures, plus separately classified observational actual MLX/Metal smoke. Selected new test path adl/tests/mlx_provider.rs is planned, not existing proof. After authoring, run cargo test --manifest-path adl/Cargo.toml --test mlx_provider and focused registered provider-profile/reload/cost/Runtime-dispatch regressions; enumerate exact test names first and record nonzero scenarios. Run cargo fmt --manifest-path adl/Cargo.toml --check and git diff --check. Couple fixture metadata for lane, role, determinism, CPU/GPU/memory/process resources and required milestone gate. Test invalid definitions, missing model/service, malformed output, timeout/cancel settling, explicit unsupported-platform errors without fallback, redaction and stdout/stderr separation; test compatibility logging if exposed. Real proof must invoke the registered production Runtime path on supported Apple hardware, pin model and MLX/Metal/macOS/candidate identities and retain nonempty output plus bounded timing/resources. Mocks prove negatives only. Exact operational smoke command must be documented/tested after actual #876 API and approved available MLX transport are resolved; no invented CLI syntax or generic local-model support claim. Required CI and actual-platform proof remain separate, mandatory, and not run.

## Failure Semantics

- Required failures, skipped or zero-test proof block acceptance. Preserve guards; record durable anomalies; repair and rerun affected proof and independent exact-head review. CI evidence is separate from local proof.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

#876 CLOSED; PR #953 MERGED at b6d110c84e11253b392d0bb078f2fb33a36b9a0c, ancestor of selected main. Setup only under Sprint 6 umbrella #932; no implementation, model loading/download, provider/service mutation, paid allocation or hardware experiment is authorized by this preparation. All 69 startup gate and #864 are accepted. Keep implementation steps pending and require each future worker to create its child-bound execution goal. Local Apple M4 Pro 64 GiB and mlx 0.32.0 observed by root. mlx-lm absent from default Python; complete model snapshot and compatible serving transport not yet verified. These facts establish platform candidacy, not an executable MLX Runtime route. Read-only cache candidate: mlx-community/Llama-3.2-3B-Instruct-4bit snapshot 7f0dc925e0d0afb0322d96f9255cfddf2ba5636e; 6 files, one safetensors of 1,807,496,278 bytes, config/tokenizer present, no broken links observed. Presence only, not load, shard-completeness or compatibility proof; candidate remains unselected.
