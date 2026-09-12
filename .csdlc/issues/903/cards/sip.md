# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0903
Run ID: issue-0903
Version: 0.92.2
Title: [v0.92.2][PLAT-MLX] Bounded MLX and Apple Metal provider adapter
Branch: codex/903-v0922-mlx-metal-provider
Card Status: ready
Generated: 2026-09-12T00:18:14.454312+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/903
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/903
- Docs: AGENTS.md; docs/csdlc-v3/CURRENT_AUTHORITY.md; docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml; ATOMIC_TASK_CONTRACTS_v0.92.2.json; accepted #876 contract when merged; adl/src/provider/mod.rs, profiles.rs, reload.rs, local.rs
- Other: none

## Agent Execution Rules
- This issue is not started yet; do not assume a branch or worktree already exists.
- Do not use v1 wrappers; bind execution with native v3 `csdlc bind` only if execution later becomes necessary.
- Do not delete or recreate cards.
- Do not switch branches unless explicitly instructed.
- Do not work on `main`.
- Only modify files required for the issue.
- Use repository-relative paths; avoid absolute host paths.
- Write the output record to the paired local task bundle `sor.md` path.
- If repository state is unexpected, stop and ask before attempting repository repair.

## Lifecycle Semantics
- Lifecycle stage: `SIP`
- Activation state: active after issue-intent review.
- Next stage: `STP`, where the selected task or solution is made explicit.
- Downstream planning path: `STP -> SPP -> VPP -> SRP -> SOR` once execution planning becomes concrete.
- Legacy compatibility: older references may call this an input card, but new issue work should treat it as the Structured Issue Prompt.

## Prompt Spec
```yaml
prompt_schema: adl.v1
actor:
  role: execution_agent
  name: codex
model:
  id: gpt-5-codex
  determinism_mode: stable
inputs:
  sections:
    - goal
    - required_outcome
    - acceptance_criteria
    - inputs
    - target_files_surfaces
    - validation_plan
    - demo_proof_requirements
    - constraints_policies
    - system_invariants
    - reviewer_checklist
    - non_goals_out_of_scope
    - notes_risks
    - instructions_to_agent
outputs:
  output_card: .csdlc/issues/903/cards/sor.md
  summary_style: concise_structured
constraints:
  include_system_invariants: true
  include_reviewer_checklist: true
  disallow_secrets: true
  disallow_absolute_host_paths: true
automation_hints:
  source_issue_prompt_required: true
  target_files_surfaces_recommended: true
  validation_plan_required: true
  required_outcome_type_supported: true
review_surfaces:
  - card_review_checklist.v1
  - card_review_output.v1
  - card_reviewer_gpt.v1.1
```

## Execution
- Agent:
- Provider:
- Tools allowed:
- Sandbox / approvals:
- Source issue-prompt slug: v0922-mlx-metal-provider
- Required outcome type: implementation_and_executed_proof
- Demo required: true

## Goal

One bounded MLX and Apple Metal adapter consumes the canonical provider-definition contract.

Dependencies: PLAT-PROVIDER. Numeric identities are attached during native creation.

## Required Outcome

One bounded MLX and Apple Metal adapter consumes the canonical provider-definition contract.

Dependencies: PLAT-PROVIDER. Numeric identities are attached during native creation.

## Acceptance Criteria

1. Implement real dispatch through the selected registered MLX/Apple Metal adapter, loading canonical endpoint/model/profile and approved resource limits. Pin an actually available compatible model, MLX/Metal versions, macOS/Apple-silicon hardware and exact candidate before execution. Hardware/model absence blocks proving execution, not honest issue creation.
2. Generate a nonempty response on actual supported hardware through Runtime's production adapter path; retain input/output identity and bounded timing/resources. A CLI wrapper, schema or mocked response alone cannot establish adapter completion.
3. Execute invalid definition, missing model/service, timeout/cancellation and malformed output behavior with actionable redacted errors. Prove unsupported platforms fail explicitly without attempting unsupported execution or silently switching providers. Deterministic mocks establish negatives, not Metal execution.
4. Preserve shared provider contract, reload/cost safeguards and credentials handling. Document installation, model/resource prerequisites, operator configuration and supported/unsupported environments. Actual smoke plus required focused CI and independent review must pass; no general local-model or benchmark claim follows.

PVF: deterministic local adapter/negative contract plus observational real Metal smoke; bounded approved local CPU/GPU/memory, required milestone support gate. Record actual resources and separate platform execution from fixture proof. No model download, paid allocation or live provider mutation without its required scope/authority.

## Inputs

Complete source contract (live issue snapshot; preserve all requirements):

# [v0.92.2][PLAT-MLX] Bounded MLX and Apple Metal provider adapter

## One complete result

One bounded MLX and Apple Metal adapter consumes the canonical provider-definition contract.

Dependencies: PLAT-PROVIDER. Numeric identities are attached during native creation.

## Bounded source and implementation

Consume PLAT-PROVIDER's merged editable definitions through `adl/src/provider/mod.rs`, `provider/profiles.rs` and `provider/reload.rs`. Existing `provider/local.rs` shows local-provider conventions; create one cohesive `adl/src/provider/mlx.rs` adapter and focused `adl/tests/mlx_provider.rs` instead of cloning registry/reload behavior. These are proposed new paths. The old TBD plan path may be absent; current adopted milestone specifications remain the requirement, not imagined historical content.

## Executed acceptance

1. Implement real dispatch through the selected registered MLX/Apple Metal adapter, loading canonical endpoint/model/profile and approved resource limits. Pin an actually available compatible model, MLX/Metal versions, macOS/Apple-silicon hardware and exact candidate before execution. Hardware/model absence blocks proving execution, not honest issue creation.
2. Generate a nonempty response on actual supported hardware through Runtime's production adapter path; retain input/output identity and bounded timing/resources. A CLI wrapper, schema or mocked response alone cannot establish adapter completion.
3. Execute invalid definition, missing model/service, timeout/cancellation and malformed output behavior with actionable redacted errors. Prove unsupported platforms fail explicitly without attempting unsupported execution or silently switching providers. Deterministic mocks establish negatives, not Metal execution.
4. Preserve shared provider contract, reload/cost safeguards and credentials handling. Document installation, model/resource prerequisites, operator configuration and supported/unsupported environments. Actual smoke plus required focused CI and independent review must pass; no general local-model or benchmark claim follows.

PVF: deterministic local adapter/negative contract plus observational real Metal smoke; bounded approved local CPU/GPU/memory, required milestone support gate. Record actual resources and separate platform execution from fixture proof. No model download, paid allocation or live provider mutation without its required scope/authority.

## Global startup and proof boundary

All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant.

Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch.

## Inherited obligation ledger

acceptance: `canonical_definition_consumed`, `platform_boundary_explicit`, `unsupported_platform_fails_cleanly`.

pvf: `mlx_metal_smoke`, `unsupported_platform_failure`.

stop_conditions: `provider_contract_bypass`, `unsupported_platform_claim`.

non_goals: `general_local_model_rewrite`, `public_benchmark_marketing`.

Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`.


## Canonical execution links

Planning owner: #864. Creation/review batch: 6; this grouping adds no execution gate.
Execution prerequisite: #876 (PLAT-PROVIDER); accepted output is required before dependent execution.

Reviewed creation source: `f5a4cd52eba0932852e8ae9c55585cee4e65f1e6`. This issue records a complete task; creation does not claim execution or acceptance.

## Target Files / Surfaces

Consume PLAT-PROVIDER's merged editable definitions through `adl/src/provider/mod.rs`, `provider/profiles.rs` and `provider/reload.rs`. Existing `provider/local.rs` shows local-provider conventions; create one cohesive `adl/src/provider/mlx.rs` adapter and focused `adl/tests/mlx_provider.rs` instead of cloning registry/reload behavior. These are proposed new paths. The old TBD plan path may be absent; current adopted milestone specifications remain the requirement, not imagined historical content.

## Validation Plan

Provider lane: deterministic local adapter contract and integration fixtures, plus separately classified observational actual MLX/Metal smoke. Selected new test path adl/tests/mlx_provider.rs is planned, not existing proof. After authoring, run cargo test --manifest-path adl/Cargo.toml --test mlx_provider and focused registered provider-profile/reload/cost/Runtime-dispatch regressions; enumerate exact test names first and record nonzero scenarios. Run cargo fmt --manifest-path adl/Cargo.toml --check and git diff --check. Couple fixture metadata for lane, role, determinism, CPU/GPU/memory/process resources and required milestone gate. Test invalid definitions, missing model/service, malformed output, timeout/cancel settling, explicit unsupported-platform errors without fallback, redaction and stdout/stderr separation; test compatibility logging if exposed. Real proof must invoke the registered production Runtime path on supported Apple hardware, pin model and MLX/Metal/macOS/candidate identities and retain nonempty output plus bounded timing/resources. Mocks prove negatives only. Exact operational smoke command must be documented/tested after actual #876 API and approved available MLX transport are resolved; no invented CLI syntax or generic local-model support claim. Required CI and actual-platform proof remain separate, mandatory, and not run.

## Demo / Proof Requirements

Provider lane: deterministic local adapter contract and integration fixtures, plus separately classified observational actual MLX/Metal smoke. Selected new test path adl/tests/mlx_provider.rs is planned, not existing proof. After authoring, run cargo test --manifest-path adl/Cargo.toml --test mlx_provider and focused registered provider-profile/reload/cost/Runtime-dispatch regressions; enumerate exact test names first and record nonzero scenarios. Run cargo fmt --manifest-path adl/Cargo.toml --check and git diff --check. Couple fixture metadata for lane, role, determinism, CPU/GPU/memory/process resources and required milestone gate. Test invalid definitions, missing model/service, malformed output, timeout/cancel settling, explicit unsupported-platform errors without fallback, redaction and stdout/stderr separation; test compatibility logging if exposed. Real proof must invoke the registered production Runtime path on supported Apple hardware, pin model and MLX/Metal/macOS/candidate identities and retain nonempty output plus bounded timing/resources. Mocks prove negatives only. Exact operational smoke command must be documented/tested after actual #876 API and approved available MLX transport are resolved; no invented CLI syntax or generic local-model support claim. Required CI and actual-platform proof remain separate, mandatory, and not run.

## Constraints / Policies

- Follow `AGENTS.md`.
- Use authenticated native C-SDLC v3 for lifecycle routing. Retained typed v2 requires explicit issue-scoped rollback or remediation approval.
- Edit cards only with editor skills.
- Work only in the bound issue worktree after native v3 `csdlc bind`.
- Keep validation focused on the touched surface.

## System Invariants (must remain true)

- Deterministic execution for identical inputs.
- No hidden state or undeclared side effects.
- Artifacts remain replay-compatible with the replay runner.
- Trace artifacts contain no secrets, prompts, tool arguments, or absolute host paths.
- Artifact schema changes are explicit and approved.

## Reviewer Checklist (machine-readable hints)
```yaml
determinism_required: true
network_allowed: false
artifact_schema_change: false
replay_required: true
security_sensitive: true
ci_validation_required: true
```

## Card Automation Hooks (prompt generation)
- Prompt source fields:
  - Goal
  - Required Outcome
  - Acceptance Criteria
  - Inputs
  - Target Files / Surfaces
  - Validation Plan
  - Demo / Proof Requirements
  - Constraints / Policies
  - System Invariants
  - Reviewer Checklist
- Generation requirements:
  - Deterministic output for identical SIP content
  - No secrets, tokens, or absolute host paths in generated prompt text
  - Preserve traceability back to the source issue prompt
  - Preserve explicit required-outcome and demo/proof requirements

## Non-goals / Out of scope

acceptance: `canonical_definition_consumed`, `platform_boundary_explicit`, `unsupported_platform_fails_cleanly`.

pvf: `mlx_metal_smoke`, `unsupported_platform_failure`.

stop_conditions: `provider_contract_bypass`, `unsupported_platform_claim`.

non_goals: `general_local_model_rewrite`, `public_benchmark_marketing`.

Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`.

## Notes / Risks

#876 CLOSED; PR #953 MERGED at b6d110c84e11253b392d0bb078f2fb33a36b9a0c, ancestor of selected main. Setup only under Sprint 6 umbrella #932; no implementation, model loading/download, provider/service mutation, paid allocation or hardware experiment is authorized by this preparation. All 69 startup gate and #864 are accepted. Keep implementation steps pending and require each future worker to create its child-bound execution goal. Local Apple M4 Pro 64 GiB and mlx 0.32.0 observed by root. mlx-lm absent from default Python; complete model snapshot and compatible serving transport not yet verified. These facts establish platform candidacy, not an executable MLX Runtime route. Read-only cache candidate: mlx-community/Llama-3.2-3B-Instruct-4bit snapshot 7f0dc925e0d0afb0322d96f9255cfddf2ba5636e; 6 files, one safetensors of 1,807,496,278 bytes, config/tokenizer present, no broken links observed. Presence only, not load, shard-completeness or compatibility proof; candidate remains unselected.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
