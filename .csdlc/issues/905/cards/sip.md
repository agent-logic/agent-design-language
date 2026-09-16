# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0905
Run ID: issue-0905
Version: 0.92.2
Title: [v0.92.2][SPEC-RETEST] Speculative-decoding requalification
Branch: codex/905-v0922-speculative-decoding-retest
Card Status: ready
Generated: 2026-09-12T00:18:14.558347+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/905
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/905
- Docs: AGENTS.md; docs/csdlc-v3/CURRENT_AUTHORITY.md; docs/milestones/v0.92.2/ADOPTED_DESIGN_CONTRACTS_v0.92.2.md; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; docs/milestones/v0.92.2/SPRINT_v0.92.2.md
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
  output_card: .csdlc/issues/905/cards/sor.md
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
- Source issue-prompt slug: v0922-speculative-decoding-retest
- Required outcome type: executed_requalification_and_disposition
- Demo required: true

## Goal

Current evidence determines whether speculative decoding should be kept, repaired, or retired.

Dependencies: WP-01. Numeric identities are attached during native creation.

## Required Outcome

Current evidence determines whether speculative decoding should be kept, repaired, or retired.

Dependencies: WP-01. Numeric identities are attached during native creation.

## Acceptance Criteria

1. Pin current Runtime/provider candidate, actual target/draft model and tokenizer revisions, engine/container/toolchain, available approved hardware, fixed corpus, sampling policy, repetitions and resource ceiling before the run. Source historical results as baseline context only.
2. Execute baseline and speculative routes through the current Runtime on the declared comparable corpus. Verify output equivalence under deterministic compatible settings or the declared correctness criterion where deterministic equality is inapplicable; reject any unsupported correctness claim. Capture accepted/proposed tokens where supported, total wall time, throughput, startup/warmth, resource cost and failed attempts.
3. Exercise draft-model failure/incompatibility and fallback to healthy ordinary generation; confirm correct outputs and bounded actionable failure behavior. Null or negative speed benefit remains valid measurement. Missing target path, unexecuted comparison or stale-only fixtures leaves the task incomplete.
4. Deliver a reviewed keep/repair/retire recommendation tied to actual current evidence and limitations. This task completes retest and disposition; it does not silently undertake model/runtime repair or decommission running services. Implementing a subsequent change requires separately bounded work.

PVF: deterministic benchmark accounting/output/fallback negatives plus actual hardware-dependent current-Runtime comparison; bounded approved CPU/GPU/disk/time, required milestone support gate. No paid/cloud/model acquisition or service disruption implied by creation. Retain nonzero executed denominators; neither a historical performance report nor a benchmark script alone closes the task.

## Inputs

Complete source contract (live issue snapshot; preserve all requirements):

# [v0.92.2][SPEC-RETEST] Speculative-decoding requalification

## One complete result

Current evidence determines whether speculative decoding should be kept, repaired, or retired.

Dependencies: WP-01. Numeric identities are attached during native creation.

## Current source and bounded retest

Start from existing `adl/tools/vllm_qwen_speculative_decoding_benchmark.py`, which records vLLM draft/accepted-token metrics, and historical provider review artifacts under `docs/milestones/v0.91.7/review/provider/`. The former `adl/src/speculative_decoding_prototype.rs` is absent at inspection; do not call a removed prototype the current Runtime path. Resolve current production routing through `adl/src/provider/` and `adl-runtime-kernel/`, record the actual integration point, and limit code changes to the retest harness. Own a current issue-local measurement packet and decision.

## Executed acceptance

1. Pin current Runtime/provider candidate, actual target/draft model and tokenizer revisions, engine/container/toolchain, available approved hardware, fixed corpus, sampling policy, repetitions and resource ceiling before the run. Source historical results as baseline context only.
2. Execute baseline and speculative routes through the current Runtime on the declared comparable corpus. Verify output equivalence under deterministic compatible settings or the declared correctness criterion where deterministic equality is inapplicable; reject any unsupported correctness claim. Capture accepted/proposed tokens where supported, total wall time, throughput, startup/warmth, resource cost and failed attempts.
3. Exercise draft-model failure/incompatibility and fallback to healthy ordinary generation; confirm correct outputs and bounded actionable failure behavior. Null or negative speed benefit remains valid measurement. Missing target path, unexecuted comparison or stale-only fixtures leaves the task incomplete.
4. Deliver a reviewed keep/repair/retire recommendation tied to actual current evidence and limitations. This task completes retest and disposition; it does not silently undertake model/runtime repair or decommission running services. Implementing a subsequent change requires separately bounded work.

PVF: deterministic benchmark accounting/output/fallback negatives plus actual hardware-dependent current-Runtime comparison; bounded approved CPU/GPU/disk/time, required milestone support gate. No paid/cloud/model acquisition or service disruption implied by creation. Retain nonzero executed denominators; neither a historical performance report nor a benchmark script alone closes the task.

## Global startup and proof boundary

All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant.

Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch.

## Inherited obligation ledger

acceptance: `current_runtime_tested`, `correctness_preserved`, `performance_measured`, `disposition_evidence_bound`.

pvf: `reproducible_benchmark`, `output_equivalence`, `fallback_test`.

stop_conditions: `stale_fixture_only`, `correctness_regression`, `unsupported_speed_claim`.

non_goals: `productization_without_requalification`.

Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`.


## Canonical execution links

Planning owner: #864. Creation/review batch: 6; this grouping adds no execution gate.
Execution prerequisite: #864 (WP-01); accepted output is required before dependent execution.

Reviewed creation source: `f5a4cd52eba0932852e8ae9c55585cee4e65f1e6`. This issue records a complete task; creation does not claim execution or acceptance.

## Target Files / Surfaces

Start from existing `adl/tools/vllm_qwen_speculative_decoding_benchmark.py`, which records vLLM draft/accepted-token metrics, and historical provider review artifacts under `docs/milestones/v0.91.7/review/provider/`. The former `adl/src/speculative_decoding_prototype.rs` is absent at inspection; do not call a removed prototype the current Runtime path. Resolve current production routing through `adl/src/provider/` and `adl-runtime-kernel/`, record the actual integration point, and limit code changes to the retest harness. Own a current issue-local measurement packet and decision.

Own bounded revisions to adl/tools/vllm_qwen_speculative_decoding_benchmark.py and proposed new adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; issue-local current measurements/decision and PVF inventory under .csdlc/evidence/905/. Provider/kernel paths are inspected only to resolve actual integration; no broad runtime repair. Historical provider review bytes remain immutable.

## Validation Plan



Planned local command after authoring: python3 adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; git diff --check. Deterministic accounting tests cover empty/zero or censored denominators, malformed/nonfinite metrics, missing output, corpus/model/revision mismatch, contradictory correctness and unsupported speed claims. Existing raw harness flags are --mode target_only|speculative, --target-model, --draft-model, --out, --repeats, --prompt-limit, --warmup-runs and --gpu-memory-utilization; inspect current help before authorized execution. Raw LLM invocation alone is insufficient: record the current Runtime command/path in SPP/VPP before running both actual routes. Retain every attempt; measure output correctness under declared compatible sampling rather than assuming exact equality. Real controlled draft failure/incompatibility must demonstrate healthy normal fallback. Actual hardware/provider execution and required CI remain separate from local fixture proof, with nonzero completed denominator.

## Demo / Proof Requirements



Planned local command after authoring: python3 adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; git diff --check. Deterministic accounting tests cover empty/zero or censored denominators, malformed/nonfinite metrics, missing output, corpus/model/revision mismatch, contradictory correctness and unsupported speed claims. Existing raw harness flags are --mode target_only|speculative, --target-model, --draft-model, --out, --repeats, --prompt-limit, --warmup-runs and --gpu-memory-utilization; inspect current help before authorized execution. Raw LLM invocation alone is insufficient: record the current Runtime command/path in SPP/VPP before running both actual routes. Retain every attempt; measure output correctness under declared compatible sampling rather than assuming exact equality. Real controlled draft failure/incompatibility must demonstrate healthy normal fallback. Actual hardware/provider execution and required CI remain separate from local fixture proof, with nonzero completed denominator.

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



## Notes / Risks

#864 CLOSED; PR #865 MERGED at f1c4e2a915c215797f0d2708cb8b0568f2b80b32, ancestor of selected main. Setup only under Sprint 6 umbrella #932; no implementation, model loading/download, provider/service mutation, paid allocation or hardware experiment is authorized by this preparation. All 69 startup gate and #864 are accepted. Keep implementation steps pending and require each future worker to create its child-bound execution goal. Current Runtime speculative route, compatible target/draft model and tokenizer revisions, engine and approved comparison resource ceiling remain unselected. Existing vLLM harness is historical starting material, not demonstrated current Runtime integration.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
