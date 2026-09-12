# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0904
Run ID: issue-0904
Version: 0.92.2
Title: [v0.92.2][PLAT-PAIR] NVIDIA PAIR multi-node local-inference experiment
Branch: codex/904-v0922-pair-multinode-experiment
Card Status: ready
Generated: 2026-09-12T00:18:16.017452+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/904
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/904
- Docs: AGENTS.md; docs/csdlc-v3/CURRENT_AUTHORITY.md; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; docs/milestones/v0.92.2/SPRINT_v0.92.2.md; docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json; docs/milestones/v0.92.2/features/SUPPORTING_PLATFORM_TRACKS_v0.92.2.md
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
  output_card: .csdlc/issues/904/cards/sor.md
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
- Source issue-prompt slug: v0922-pair-multinode-experiment
- Required outcome type: executed_hardware_experiment_and_reviewed_decision
- Demo required: true

## Goal

A bounded experiment determines whether NVIDIA PAIR improves multi-node local inference for ADL without becoming a provider type.

Dependencies: PLAT-PROVIDER. Numeric identities are attached during native creation.

## Required Outcome

A bounded experiment determines whether NVIDIA PAIR improves multi-node local inference for ADL without becoming a provider type.

Dependencies: PLAT-PROVIDER. Numeric identities are attached during native creation.

## Acceptance Criteria

1. Before experiment execution, pin supported PAIR implementation/version, licensed model revision, two available approved nodes, transport, concurrency levels, baseline route, request corpus, time/memory/resource bounds and cost ceiling in the issue plan. These execution facts are not claimed selected merely by creating this issue. Missing suitable resources is a visible blocker, never synthetic success.
2. Run a real bounded raw PAIR route and the same requests through current Runtime v3 using canonical provider definitions. Retain exact code/model/hardware, per-request result, correctness/response checks, latency/throughput and resource measurements, startup and failure data. Repeat the declared same-corpus baseline for a fair comparison.
3. Exercise concurrent requests and actual controlled node-loss/unavailability with bounded recovery or declared fallback. Record failures, exclusions, cache/warmth and null/no-benefit outcomes; no caller-set success flags or reference trace substitutes for execution.
4. Produce one reviewed keep/repair/retire decision justified by complete observed data, including negative results. A negative benefit result can complete the experiment; missing raw or Runtime execution cannot. Separate this experimental decision from production readiness and preserve all evidence.

PVF: deterministic harness/accounting/negative contracts plus hardware-dependent integration/measurement; resources restricted to explicitly approved nodes/network and budget, required milestone experiment gate. No automatic paid provisioning, model sharding, pooled-VRAM claim, production rollout or public benchmark marketing. Returned deficiencies become scoped follow-up work rather than unbounded repairs in this experiment.

acceptance: `canonical_provider_definition_consumed`, `raw_pair_route_proven`, `runtime_path_compared`, `concurrency_and_failure_behavior_measured`, `no_provider_type_added`.

pvf: `raw_pair_smoke`, `runtime_v3_pair_smoke`, `concurrent_request_comparison`, `node_loss_fallback`, `null_benefit_detection`.

stop_conditions: `provider_contract_bypass`, `paid_or_network_mutation_without_authority`, `unsupported_performance_claim`.

non_goals: `model_sharding`, `pooled_vram`, `production_rollout`.

Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`.

## Inputs

Full live source contract, retained without dropping requirements:

# [v0.92.2][PLAT-PAIR] NVIDIA PAIR multi-node local-inference experiment

## One complete result

A bounded experiment determines whether NVIDIA PAIR improves multi-node local inference for ADL without becoming a provider type.

Dependencies: PLAT-PROVIDER. Numeric identities are attached during native creation.

## Bounded source and experiment

Use canonical definitions from PLAT-PROVIDER and existing provider routing in `adl/src/provider/mod.rs`, `provider/profiles.rs`, `provider/http_family.rs` and `provider/local.rs`. Own a proposed reproducible harness `adl/tools/pair_experiment.py` and issue-local raw results/decision. Read current Runtime routes before wiring the experiment; add no new PAIR provider type and do not rewrite production provider behavior. The cited historical TBD plan may be absent; do not invent its contents.

## Executed acceptance

1. Before experiment execution, pin supported PAIR implementation/version, licensed model revision, two available approved nodes, transport, concurrency levels, baseline route, request corpus, time/memory/resource bounds and cost ceiling in the issue plan. These execution facts are not claimed selected merely by creating this issue. Missing suitable resources is a visible blocker, never synthetic success.
2. Run a real bounded raw PAIR route and the same requests through current Runtime v3 using canonical provider definitions. Retain exact code/model/hardware, per-request result, correctness/response checks, latency/throughput and resource measurements, startup and failure data. Repeat the declared same-corpus baseline for a fair comparison.
3. Exercise concurrent requests and actual controlled node-loss/unavailability with bounded recovery or declared fallback. Record failures, exclusions, cache/warmth and null/no-benefit outcomes; no caller-set success flags or reference trace substitutes for execution.
4. Produce one reviewed keep/repair/retire decision justified by complete observed data, including negative results. A negative benefit result can complete the experiment; missing raw or Runtime execution cannot. Separate this experimental decision from production readiness and preserve all evidence.

PVF: deterministic harness/accounting/negative contracts plus hardware-dependent integration/measurement; resources restricted to explicitly approved nodes/network and budget, required milestone experiment gate. No automatic paid provisioning, model sharding, pooled-VRAM claim, production rollout or public benchmark marketing. Returned deficiencies become scoped follow-up work rather than unbounded repairs in this experiment.

## Global startup and proof boundary

All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant.

Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch.

## Inherited obligation ledger

acceptance: `canonical_provider_definition_consumed`, `raw_pair_route_proven`, `runtime_path_compared`, `concurrency_and_failure_behavior_measured`, `no_provider_type_added`.

pvf: `raw_pair_smoke`, `runtime_v3_pair_smoke`, `concurrent_request_comparison`, `node_loss_fallback`, `null_benefit_detection`.

stop_conditions: `provider_contract_bypass`, `paid_or_network_mutation_without_authority`, `unsupported_performance_claim`.

non_goals: `model_sharding`, `pooled_vram`, `production_rollout`.

Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`.


## Canonical execution links

Planning owner: #864. Creation/review batch: 6; this grouping adds no execution gate.
Execution prerequisite: #876 (PLAT-PROVIDER); accepted output is required before dependent execution.

Reviewed creation source: `f5a4cd52eba0932852e8ae9c55585cee4e65f1e6`. This issue records a complete task; creation does not claim execution or acceptance.


<!-- csdlc-v3-operation:93e1f6fccb7993ac6d7c9495aad0d0ec2ca235716deab9e014cb8a2e82b633c0 -->

## Target Files / Surfaces

Use canonical definitions from PLAT-PROVIDER and existing provider routing in `adl/src/provider/mod.rs`, `provider/profiles.rs`, `provider/http_family.rs` and `provider/local.rs`. Own a proposed reproducible harness `adl/tools/pair_experiment.py` and issue-local raw results/decision. Read current Runtime routes before wiring the experiment; add no new PAIR provider type and do not rewrite production provider behavior. The cited historical TBD plan may be absent; do not invent its contents.

## Validation Plan

1. Before experiment execution, pin supported PAIR implementation/version, licensed model revision, two available approved nodes, transport, concurrency levels, baseline route, request corpus, time/memory/resource bounds and cost ceiling in the issue plan. These execution facts are not claimed selected merely by creating this issue. Missing suitable resources is a visible blocker, never synthetic success.
2. Run a real bounded raw PAIR route and the same requests through current Runtime v3 using canonical provider definitions. Retain exact code/model/hardware, per-request result, correctness/response checks, latency/throughput and resource measurements, startup and failure data. Repeat the declared same-corpus baseline for a fair comparison.
3. Exercise concurrent requests and actual controlled node-loss/unavailability with bounded recovery or declared fallback. Record failures, exclusions, cache/warmth and null/no-benefit outcomes; no caller-set success flags or reference trace substitutes for execution.
4. Produce one reviewed keep/repair/retire decision justified by complete observed data, including negative results. A negative benefit result can complete the experiment; missing raw or Runtime execution cannot. Separate this experimental decision from production readiness and preserve all evidence.

PVF: deterministic harness/accounting/negative contracts plus hardware-dependent integration/measurement; resources restricted to explicitly approved nodes/network and budget, required milestone experiment gate. No automatic paid provisioning, model sharding, pooled-VRAM claim, production rollout or public benchmark marketing. Returned deficiencies become scoped follow-up work rather than unbounded repairs in this experiment.

All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant.

Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch.

Proposed harness `adl/tools/pair_experiment.py` does not exist at preparation. Add its bounded deterministic contract tests at proposed `adl/tools/test_pair_experiment.py`; run `python3 adl/tools/test_pair_experiment.py` after authoring. Fixtures reject omitted requests, mismatched corpus/model/warmth/concurrency, fabricated success, missing Runtime or raw route, absent node-loss effects and invalid latency/resource accounting. These are local harness proofs, not hardware measurements. Resolve the implemented harness argv and exact approved environment only after #876 lands, then execute both real routes and same-corpus baseline with complete results and a current independent review. Do not guess a PAIR version, node identity, model license or performance result now.

Use focused existing provider/profile tests only if their production owners are touched; inspect registered names first. Source currently routes through build_provider_for_id and provider_profile_materialization_projection; no PAIR provider type exists or is authorized. Keep ordinary provider behavior unchanged. Required raw/runtime/node-loss and null-benefit evidence cannot be replaced by test-only transports or a zero-match Rust filter.

`git diff --check` accompanies focused proof. Required qualification harness repairs and scenario proofs are future work; existing source alone is not acceptance. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

## Demo / Proof Requirements

1. Before experiment execution, pin supported PAIR implementation/version, licensed model revision, two available approved nodes, transport, concurrency levels, baseline route, request corpus, time/memory/resource bounds and cost ceiling in the issue plan. These execution facts are not claimed selected merely by creating this issue. Missing suitable resources is a visible blocker, never synthetic success.
2. Run a real bounded raw PAIR route and the same requests through current Runtime v3 using canonical provider definitions. Retain exact code/model/hardware, per-request result, correctness/response checks, latency/throughput and resource measurements, startup and failure data. Repeat the declared same-corpus baseline for a fair comparison.
3. Exercise concurrent requests and actual controlled node-loss/unavailability with bounded recovery or declared fallback. Record failures, exclusions, cache/warmth and null/no-benefit outcomes; no caller-set success flags or reference trace substitutes for execution.
4. Produce one reviewed keep/repair/retire decision justified by complete observed data, including negative results. A negative benefit result can complete the experiment; missing raw or Runtime execution cannot. Separate this experimental decision from production readiness and preserve all evidence.

PVF: deterministic harness/accounting/negative contracts plus hardware-dependent integration/measurement; resources restricted to explicitly approved nodes/network and budget, required milestone experiment gate. No automatic paid provisioning, model sharding, pooled-VRAM claim, production rollout or public benchmark marketing. Returned deficiencies become scoped follow-up work rather than unbounded repairs in this experiment.

All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant.

Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch.

Proposed harness `adl/tools/pair_experiment.py` does not exist at preparation. Add its bounded deterministic contract tests at proposed `adl/tools/test_pair_experiment.py`; run `python3 adl/tools/test_pair_experiment.py` after authoring. Fixtures reject omitted requests, mismatched corpus/model/warmth/concurrency, fabricated success, missing Runtime or raw route, absent node-loss effects and invalid latency/resource accounting. These are local harness proofs, not hardware measurements. Resolve the implemented harness argv and exact approved environment only after #876 lands, then execute both real routes and same-corpus baseline with complete results and a current independent review. Do not guess a PAIR version, node identity, model license or performance result now.

Use focused existing provider/profile tests only if their production owners are touched; inspect registered names first. Source currently routes through build_provider_for_id and provider_profile_materialization_projection; no PAIR provider type exists or is authorized. Keep ordinary provider behavior unchanged. Required raw/runtime/node-loss and null-benefit evidence cannot be replaced by test-only transports or a zero-match Rust filter.

`git diff --check` accompanies focused proof. Required qualification harness repairs and scenario proofs are future work; existing source alone is not acceptance. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

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

acceptance: `canonical_provider_definition_consumed`, `raw_pair_route_proven`, `runtime_path_compared`, `concurrency_and_failure_behavior_measured`, `no_provider_type_added`.

pvf: `raw_pair_smoke`, `runtime_v3_pair_smoke`, `concurrent_request_comparison`, `node_loss_fallback`, `null_benefit_detection`.

stop_conditions: `provider_contract_bypass`, `paid_or_network_mutation_without_authority`, `unsupported_performance_claim`.

non_goals: `model_sharding`, `pooled_vram`, `production_rollout`.

Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`.

All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant.

Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch.

## Notes / Risks

Implementation of bounded harness/accounting and deterministic negatives authorized under #932. Native bound goal active for #904. Real experiment remains blocked on two approved compatible nodes, installed PAIR version/model and current Runtime route. Upstream PAIR supports macOS as well as Windows/Linux; NVIDIA-only node restriction is not part of issue acceptance. No paid provisioning, downloads or service mutation authorized.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
