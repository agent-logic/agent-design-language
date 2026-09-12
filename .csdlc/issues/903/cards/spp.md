---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0922-mlx-metal-provider-execution-plan"
issue: 903
task_id: "issue-0903"
run_id: "issue-0903"
version: "0.92.2"
title: "[v0.92.2][PLAT-MLX] Bounded MLX and Apple Metal provider adapter"
branch: "codex/903-v0922-mlx-metal-provider"
generated_at: "2026-09-12T00:18:14.454312+00:00"
card_status: "ready"
status: "planned"
activation_state: "prepared_not_bound"
plan_revision: 1
initial_pvf_lane: "provider"
planned_pvf_lane: "provider"
planned_pvf_lane_source: "https://github.com/agent-logic/agent-design-language/issues/903 validation contract; docs/validation/pvf_lanes.json"
estimate_elapsed_seconds: "14400"
estimate_total_tokens: "30000"
estimate_validation_seconds: "1800"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "low"
estimate_data_source: "Conservative planning estimate for bounded adapter and deterministic fixtures with warm cache; real hardware/model provisioning and observation excluded pending available environment; not an imposed token budget"
estimate_source_ref: "https://github.com/agent-logic/agent-design-language/issues/903"
issue_goal_ref: "Active Planning #7 goal: Sprint6 #932 child903 full implementation, hardware proof, reviewed green PR and post-merge native closeout"
sprint_goal_ref: "issue-932 Sprint6 implementation"
goal_metrics_rollup_ref: ".csdlc/evidence/903/goal-metrics.json (planned; absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/903"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/903"
  - kind: "stp"
    ref: ".csdlc/issues/903/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/903/cards/sip.md"
scope:
  files:
    - "Consume PLAT-PROVIDER's merged editable definitions through `adl/src/provider/mod.rs`, `provider/profiles.rs` and `provider/reload.rs`. Existing `provider/local.rs` shows local-provider conventions; create one cohesive `adl/src/provider/mlx.rs` adapter and focused `adl/tests/mlx_provider.rs` instead of cloning registry/reload behavior. These are proposed new paths. The old TBD plan path may be absent; current adopted milestone specifications remain the requirement, not imagined historical content."
  components:
    - "v0922-mlx-metal-provider"
  out_of_scope:
    - "acceptance: `canonical_definition_consumed`, `platform_boundary_explicit`, `unsupported_platform_fails_cleanly`. pvf: `mlx_metal_smoke`, `unsupported_platform_failure`. stop_conditions: `provider_contract_bypass`, `unsupported_platform_claim`. non_goals: `general_local_model_rewrite`, `public_benchmark_marketing`. Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Complete real Metal evidence, successful-review matched-provider validation, independent exact-head review and green PR; operator merge then native closeout."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: #876 CLOSED; PR #953 MERGED at b6d110c84e11253b392d0bb078f2fb33a36b9a0c, ancestor of selected main; no sibling experiment dependency."
    expected_output: ".csdlc/issues/903/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Complete source contract (live issue snapshot; preserve all requirements): # [v0.92.2][PLAT-MLX] Bounded MLX and Apple Metal provider adapter ## One complete result One bounded MLX and Apple Metal adapter consumes the canonical provider-definition contract. Dependencies: PLAT-PROVIDER. Numeric identities are attached during native creation. ## Bounded source and implementation Consume PLAT-PROVIDER's merged editable definitions through `adl/src/provider/mod.rs`, `provider/profiles.rs` and `provider/reload.rs`. Existing `provider/local.rs` shows local-provider conventions; create one cohesive `adl/src/provider/mlx.rs` adapter and focused `adl/tests/mlx_provider.rs` instead of cloning registry/reload behavior. These are proposed new paths. The old TBD plan path may be absent; current adopted milestone specifications remain the requirement, not imagined historical content. ## Executed acceptance 1. Implement real dispatch through the selected registered MLX/Apple Metal adapter, loading canonical endpoint/model/profile and approved resource limits. Pin an actually available compatible model, MLX/Metal versions, macOS/Apple-silicon hardware and exact candidate before execution. Hardware/model absence blocks proving execution, not honest issue creation. 2. Generate a nonempty response on actual supported hardware through Runtime's production adapter path; retain input/output identity and bounded timing/resources. A CLI wrapper, schema or mocked response alone cannot establish adapter completion. 3. Execute invalid definition, missing model/service, timeout/cancellation and malformed output behavior with actionable redacted errors. Prove unsupported platforms fail explicitly without attempting unsupported execution or silently switching providers. Deterministic mocks establish negatives, not Metal execution. 4. Preserve shared provider contract, reload/cost safeguards and credentials handling. Document installation, model/resource prerequisites, operator configuration and supported/unsupported environments. Actual smoke plus required focused CI and independent review must pass; no general local-model or benchmark claim follows. PVF: deterministic local adapter/negative contract plus observational real Metal smoke; bounded approved local CPU/GPU/memory, required milestone support gate. Record actual resources and separate platform execution from fixture proof. No model download, paid allocation or live provider mutation without its required scope/authority. ## Global startup and proof boundary All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant. Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch. ## Inherited obligation ledger acceptance: `canonical_definition_consumed`, `platform_boundary_explicit`, `unsupported_platform_fails_cleanly`. pvf: `mlx_metal_smoke`, `unsupported_platform_failure`. stop_conditions: `provider_contract_bypass`, `unsupported_platform_claim`. non_goals: `general_local_model_rewrite`, `public_benchmark_marketing`. Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`. ## Canonical execution links Planning owner: #864. Creation/review batch: 6; this grouping adds no execution gate. Execution prerequisite: #876 (PLAT-PROVIDER); accepted output is required before dependent execution. Reviewed creation source: `f5a4cd52eba0932852e8ae9c55585cee4e65f1e6`. This issue records a complete task; creation does not claim execution or acceptance."
    expected_output: ".csdlc/issues/903/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Consume PLAT-PROVIDER's merged editable definitions through `adl/src/provider/mod.rs`, `provider/profiles.rs` and `provider/reload.rs`. Existing `provider/local.rs` shows local-provider conventions; create one cohesive `adl/src/provider/mlx.rs` adapter and focused `adl/tests/mlx_provider.rs` instead of cloning registry/reload behavior. These are proposed new paths. The old TBD plan path may be absent; current adopted milestone specifications remain the requirement, not imagined historical content. One bounded MLX and Apple Metal adapter consumes the canonical provider-definition contract. Dependencies: PLAT-PROVIDER. Numeric identities are attached during native creation."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: 1. Implement real dispatch through the selected registered MLX/Apple Metal adapter, loading canonical endpoint/model/profile and approved resource limits. Pin an actually available compatible model, MLX/Metal versions, macOS/Apple-silicon hardware and exact candidate before execution. Hardware/model absence blocks proving execution, not honest issue creation. 2. Generate a nonempty response on actual supported hardware through Runtime's production adapter path; retain input/output identity and bounded timing/resources. A CLI wrapper, schema or mocked response alone cannot establish adapter completion. 3. Execute invalid definition, missing model/service, timeout/cancellation and malformed output behavior with actionable redacted errors. Prove unsupported platforms fail explicitly without attempting unsupported execution or silently switching providers. Deterministic mocks establish negatives, not Metal execution. 4. Preserve shared provider contract, reload/cost safeguards and credentials handling. Document installation, model/resource prerequisites, operator configuration and supported/unsupported environments. Actual smoke plus required focused CI and independent review must pass; no general local-model or benchmark claim follows. PVF: deterministic local adapter/negative contract plus observational real Metal smoke; bounded approved local CPU/GPU/memory, required milestone support gate. Record actual resources and separate platform execution from fixture proof. No model download, paid allocation or live provider mutation without its required scope/authority."
    expected_output: "validation evidence recorded in VPP/SOR"
    allowed_mode: "execution_after_approval"
  - id: "step-5"
    description: "Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges."
    expected_output: "reviewed SRP and truthful VPP/SOR"
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Confirm dependencies and starting state from the source issue prompt."
    status: "pending"
  - step: "Inspect repo inputs and target surfaces before editing."
    status: "pending"
  - step: "Implement the bounded deliverables only."
    status: "pending"
  - step: "Run focused validation and proof gates."
    status: "pending"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "pending"
affected_areas:
  - "v0922-mlx-metal-provider"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "#876 CLOSED; PR #953 MERGED at b6d110c84e11253b392d0bb078f2fb33a36b9a0c, ancestor of selected main. Setup only under Sprint 6 umbrella #932; no implementation, model loading/download, provider/service mutation, paid allocation or hardware experiment is authorized by this preparation. All 69 startup gate and #864 are accepted. Keep implementation steps pending and require each future worker to create its child-bound execution goal. Local Apple M4 Pro 64 GiB and mlx 0.32.0 observed by root. mlx-lm absent from default Python; complete model snapshot and compatible serving transport not yet verified. These facts establish platform candidacy, not an executable MLX Runtime route. Read-only cache candidate: mlx-community/Llama-3.2-3B-Instruct-4bit snapshot 7f0dc925e0d0afb0322d96f9255cfddf2ba5636e; 6 files, one safetensors of 1,807,496,278 bytes, config/tokenizer present, no broken links observed. Presence only, not load, shard-completeness or compatibility proof; candidate remains unselected."
test_strategy:
  - "Provider lane: deterministic local adapter contract and integration fixtures, plus separately classified observational actual MLX/Metal smoke. Selected new test path adl/tests/mlx_provider.rs is planned, not existing proof. After authoring, run cargo test --manifest-path adl/Cargo.toml --test mlx_provider and focused registered provider-profile/reload/cost/Runtime-dispatch regressions; enumerate exact test names first and record nonzero scenarios. Run cargo fmt --manifest-path adl/Cargo.toml --check and git diff --check. Couple fixture metadata for lane, role, determinism, CPU/GPU/memory/process resources and required milestone gate. Test invalid definitions, missing model/service, malformed output, timeout/cancel settling, explicit unsupported-platform errors without fallback, redaction and stdout/stderr separation; test compatibility logging if exposed. Real proof must invoke the registered production Runtime path on supported Apple hardware, pin model and MLX/Metal/macOS/candidate identities and retain nonempty output plus bounded timing/resources. Mocks prove negatives only. Exact operational smoke command must be documented/tested after actual #876 API and approved available MLX transport are resolved; no invented CLI syntax or generic local-model support claim. Required CI and actual-platform proof remain separate, mandatory, and not run."
execution_handoff: "Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges."
required_permissions:
  - "workspace-write after execution approval"
stop_conditions:
  - "Stop and re-plan if dependencies are unmet or materially different from this design-time plan."
  - "Stop and update SPP if touched files, proof gates, or validation commands change materially."
  - "Stop and route follow-on work if acceptance requires scope outside this issue."
alternatives_considered:
  - description: "Rely only on transient chat planning."
    reason_not_chosen: "Chat-only planning is not durable or reviewable enough for this workflow surface."
review_hooks:
  - "Check dependency truth, scope truthfulness, touched-file truthfulness, validation sufficiency, and re-plan triggers."
notes: "Root Planning #7 owns903; adapter and real canonical workflow Metal smoke passed. Operator expanded bounded validation to apples-to-apples MLX/non-MLX real code-review comparison with larger budgets; both reviews must succeed before speed comparison is accepted. Llama3.2-3B failed review quality at512 and2048 token ceilings, so those timings are non-proving for successful reviews. Qualify stronger matched weights/tokenizer/settings and independent correctness assessment. Preserve #855 shared provider extraction coordination and no resident-kernel claim."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][PLAT-MLX] Bounded MLX and Apple Metal provider adapter`.

Complete real Metal evidence, successful-review matched-provider validation, independent exact-head review and green PR; operator merge then native closeout.

## PVF Lane Plan

- Initial PVF lane from issue creation: `provider`
- Planned PVF lane for execution: `provider`
- Planning lane source: `https://github.com/agent-logic/agent-design-language/issues/903 validation contract; docs/validation/pvf_lanes.json`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `14400`
- Estimated total tokens: `30000`
- Estimated validation seconds: `1800`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `low`
- Estimate data source: `Conservative planning estimate for bounded adapter and deterministic fixtures with warm cache; real hardware/model provisioning and observation excluded pending available environment; not an imposed token budget`
- Estimate source ref: `https://github.com/agent-logic/agent-design-language/issues/903`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [pending] Confirm dependencies and starting state from the source issue prompt.
2. [pending] Inspect repo inputs and target surfaces before editing.
3. [pending] Implement the bounded deliverables only.
4. [pending] Run focused validation and proof gates.
5. [pending] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: #876 CLOSED; PR #953 MERGED at b6d110c84e11253b392d0bb078f2fb33a36b9a0c, ancestor of selected main; no sibling experiment dependency.
2. Review repo inputs and scoped surfaces before editing: Complete source contract (live issue snapshot; preserve all requirements): # [v0.92.2][PLAT-MLX] Bounded MLX and Apple Metal provider adapter ## One complete result One bounded MLX and Apple Metal adapter consumes the canonical provider-definition contract. Dependencies: PLAT-PROVIDER. Numeric identities are attached during native creation. ## Bounded source and implementation Consume PLAT-PROVIDER's merged editable definitions through `adl/src/provider/mod.rs`, `provider/profiles.rs` and `provider/reload.rs`. Existing `provider/local.rs` shows local-provider conventions; create one cohesive `adl/src/provider/mlx.rs` adapter and focused `adl/tests/mlx_provider.rs` instead of cloning registry/reload behavior. These are proposed new paths. The old TBD plan path may be absent; current adopted milestone specifications remain the requirement, not imagined historical content. ## Executed acceptance 1. Implement real dispatch through the selected registered MLX/Apple Metal adapter, loading canonical endpoint/model/profile and approved resource limits. Pin an actually available compatible model, MLX/Metal versions, macOS/Apple-silicon hardware and exact candidate before execution. Hardware/model absence blocks proving execution, not honest issue creation. 2. Generate a nonempty response on actual supported hardware through Runtime's production adapter path; retain input/output identity and bounded timing/resources. A CLI wrapper, schema or mocked response alone cannot establish adapter completion. 3. Execute invalid definition, missing model/service, timeout/cancellation and malformed output behavior with actionable redacted errors. Prove unsupported platforms fail explicitly without attempting unsupported execution or silently switching providers. Deterministic mocks establish negatives, not Metal execution. 4. Preserve shared provider contract, reload/cost safeguards and credentials handling. Document installation, model/resource prerequisites, operator configuration and supported/unsupported environments. Actual smoke plus required focused CI and independent review must pass; no general local-model or benchmark claim follows. PVF: deterministic local adapter/negative contract plus observational real Metal smoke; bounded approved local CPU/GPU/memory, required milestone support gate. Record actual resources and separate platform execution from fixture proof. No model download, paid allocation or live provider mutation without its required scope/authority. ## Global startup and proof boundary All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant. Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch. ## Inherited obligation ledger acceptance: `canonical_definition_consumed`, `platform_boundary_explicit`, `unsupported_platform_fails_cleanly`. pvf: `mlx_metal_smoke`, `unsupported_platform_failure`. stop_conditions: `provider_contract_bypass`, `unsupported_platform_claim`. non_goals: `general_local_model_rewrite`, `public_benchmark_marketing`. Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`. ## Canonical execution links Planning owner: #864. Creation/review batch: 6; this grouping adds no execution gate. Execution prerequisite: #876 (PLAT-PROVIDER); accepted output is required before dependent execution. Reviewed creation source: `f5a4cd52eba0932852e8ae9c55585cee4e65f1e6`. This issue records a complete task; creation does not claim execution or acceptance.
3. Implement only the bounded deliverables: Consume PLAT-PROVIDER's merged editable definitions through `adl/src/provider/mod.rs`, `provider/profiles.rs` and `provider/reload.rs`. Existing `provider/local.rs` shows local-provider conventions; create one cohesive `adl/src/provider/mlx.rs` adapter and focused `adl/tests/mlx_provider.rs` instead of cloning registry/reload behavior. These are proposed new paths. The old TBD plan path may be absent; current adopted milestone specifications remain the requirement, not imagined historical content. One bounded MLX and Apple Metal adapter consumes the canonical provider-definition contract. Dependencies: PLAT-PROVIDER. Numeric identities are attached during native creation.
4. Run focused proof gates for acceptance: 1. Implement real dispatch through the selected registered MLX/Apple Metal adapter, loading canonical endpoint/model/profile and approved resource limits. Pin an actually available compatible model, MLX/Metal versions, macOS/Apple-silicon hardware and exact candidate before execution. Hardware/model absence blocks proving execution, not honest issue creation. 2. Generate a nonempty response on actual supported hardware through Runtime's production adapter path; retain input/output identity and bounded timing/resources. A CLI wrapper, schema or mocked response alone cannot establish adapter completion. 3. Execute invalid definition, missing model/service, timeout/cancellation and malformed output behavior with actionable redacted errors. Prove unsupported platforms fail explicitly without attempting unsupported execution or silently switching providers. Deterministic mocks establish negatives, not Metal execution. 4. Preserve shared provider contract, reload/cost safeguards and credentials handling. Document installation, model/resource prerequisites, operator configuration and supported/unsupported environments. Actual smoke plus required focused CI and independent review must pass; no general local-model or benchmark claim follows. PVF: deterministic local adapter/negative contract plus observational real Metal smoke; bounded approved local CPU/GPU/memory, required milestone support gate. Record actual resources and separate platform execution from fixture proof. No model download, paid allocation or live provider mutation without its required scope/authority.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- v0922-mlx-metal-provider

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- #876 CLOSED; PR #953 MERGED at b6d110c84e11253b392d0bb078f2fb33a36b9a0c, ancestor of selected main. Setup only under Sprint 6 umbrella #932; no implementation, model loading/download, provider/service mutation, paid allocation or hardware experiment is authorized by this preparation. All 69 startup gate and #864 are accepted. Keep implementation steps pending and require each future worker to create its child-bound execution goal. Local Apple M4 Pro 64 GiB and mlx 0.32.0 observed by root. mlx-lm absent from default Python; complete model snapshot and compatible serving transport not yet verified. These facts establish platform candidacy, not an executable MLX Runtime route. Read-only cache candidate: mlx-community/Llama-3.2-3B-Instruct-4bit snapshot 7f0dc925e0d0afb0322d96f9255cfddf2ba5636e; 6 files, one safetensors of 1,807,496,278 bytes, config/tokenizer present, no broken links observed. Presence only, not load, shard-completeness or compatibility proof; candidate remains unselected.

## Test Strategy

- Provider lane: deterministic local adapter contract and integration fixtures, plus separately classified observational actual MLX/Metal smoke. Selected new test path adl/tests/mlx_provider.rs is planned, not existing proof. After authoring, run cargo test --manifest-path adl/Cargo.toml --test mlx_provider and focused registered provider-profile/reload/cost/Runtime-dispatch regressions; enumerate exact test names first and record nonzero scenarios. Run cargo fmt --manifest-path adl/Cargo.toml --check and git diff --check. Couple fixture metadata for lane, role, determinism, CPU/GPU/memory/process resources and required milestone gate. Test invalid definitions, missing model/service, malformed output, timeout/cancel settling, explicit unsupported-platform errors without fallback, redaction and stdout/stderr separation; test compatibility logging if exposed. Real proof must invoke the registered production Runtime path on supported Apple hardware, pin model and MLX/Metal/macOS/candidate identities and retain nonempty output plus bounded timing/resources. Mocks prove negatives only. Exact operational smoke command must be documented/tested after actual #876 API and approved available MLX transport are resolved; no invented CLI syntax or generic local-model support claim. Required CI and actual-platform proof remain separate, mandatory, and not run.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Root Planning #7 owns903; adapter and real canonical workflow Metal smoke passed. Operator expanded bounded validation to apples-to-apples MLX/non-MLX real code-review comparison with larger budgets; both reviews must succeed before speed comparison is accepted. Llama3.2-3B failed review quality at512 and2048 token ceilings, so those timings are non-proving for successful reviews. Qualify stronger matched weights/tokenizer/settings and independent correctness assessment. Preserve #855 shared provider extraction coordination and no resident-kernel claim.
