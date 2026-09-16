---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0922-speculative-decoding-retest-execution-plan"
issue: 905
task_id: "issue-0905"
run_id: "issue-0905"
version: "0.92.2"
title: "[v0.92.2][SPEC-RETEST] Speculative-decoding requalification"
branch: "codex/905-v0922-speculative-decoding-retest"
generated_at: "2026-09-12T00:18:14.558347+00:00"
card_status: "ready"
status: "completed"
activation_state: "executed_re_review_pending"
plan_revision: 1
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
planned_pvf_lane_source: "https://github.com/agent-logic/agent-design-language/issues/905 validation contract; docs/validation/pvf_lanes.json"
estimate_elapsed_seconds: "10800"
estimate_total_tokens: "65000"
estimate_validation_seconds: "3600"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "low"
estimate_data_source: "Conservative retest harness/accounting and measurement estimate; hardware elapsed time reestimated after route/resource selection, not operator budget"
estimate_source_ref: "https://github.com/agent-logic/agent-design-language/issues/905"
issue_goal_ref: "Active goal: complete #905 current Runtime requalification, publish and close, then close Sprint 6 #932."
sprint_goal_ref: "issue-932; Sprint 6 setup and coordination"
goal_metrics_rollup_ref: ".csdlc/evidence/905/goal-metrics.json (planned; absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/905"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/905"
  - kind: "stp"
    ref: ".csdlc/issues/905/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/905/cards/sip.md"
scope:
  files:
    - "Start from existing `adl/tools/vllm_qwen_speculative_decoding_benchmark.py`, which records vLLM draft/accepted-token metrics, and historical provider review artifacts under `docs/milestones/v0.91.7/review/provider/`. The former `adl/src/speculative_decoding_prototype.rs` is absent at inspection; do not call a removed prototype the current Runtime path. Resolve current production routing through `adl/src/provider/` and `adl-runtime-kernel/`, record the actual integration point, and limit code changes to the retest harness. Own a current issue-local measurement packet and decision. Own bounded revisions to adl/tools/vllm_qwen_speculative_decoding_benchmark.py and proposed new adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; issue-local current measurements/decision and PVF inventory under .csdlc/evidence/905/. Provider/kernel paths are inspected only to resolve actual integration; no broad runtime repair. Historical provider review bytes remain immutable."
  components:
    - "v0922-speculative-decoding-retest"
  out_of_scope:
    - ""
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Verify accepted WP-01 and reconcile current harness/Runtime owner; locate a real current Runtime route for both target-only and speculative execution before making production claims; pin target/draft/tokenizer revisions, engine/container/toolchain, approved hardware, fixed corpus/repeats, correctness criterion, resource/cost ceilings and warmup policy; update only bounded retest harness and accounting tests; execute both routes on the same corpus retaining all outputs/timings/token counts and failed attempts, then controlled draft failure/incompatibility with healthy ordinary-generation fallback; compare correctness and benefit including null/negative results; deliver reviewed keep/repair/retire disposition with actual evidence, no runtime repair or decommissioning."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: #864 CLOSED; PR #865 MERGED at f1c4e2a915c215797f0d2708cb8b0568f2b80b32, ancestor of selected main; no sibling experiment dependency."
    expected_output: ".csdlc/issues/905/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Complete source contract (live issue snapshot; preserve all requirements): # [v0.92.2][SPEC-RETEST] Speculative-decoding requalification ## One complete result Current evidence determines whether speculative decoding should be kept, repaired, or retired. Dependencies: WP-01. Numeric identities are attached during native creation. ## Current source and bounded retest Start from existing `adl/tools/vllm_qwen_speculative_decoding_benchmark.py`, which records vLLM draft/accepted-token metrics, and historical provider review artifacts under `docs/milestones/v0.91.7/review/provider/`. The former `adl/src/speculative_decoding_prototype.rs` is absent at inspection; do not call a removed prototype the current Runtime path. Resolve current production routing through `adl/src/provider/` and `adl-runtime-kernel/`, record the actual integration point, and limit code changes to the retest harness. Own a current issue-local measurement packet and decision. ## Executed acceptance 1. Pin current Runtime/provider candidate, actual target/draft model and tokenizer revisions, engine/container/toolchain, available approved hardware, fixed corpus, sampling policy, repetitions and resource ceiling before the run. Source historical results as baseline context only. 2. Execute baseline and speculative routes through the current Runtime on the declared comparable corpus. Verify output equivalence under deterministic compatible settings or the declared correctness criterion where deterministic equality is inapplicable; reject any unsupported correctness claim. Capture accepted/proposed tokens where supported, total wall time, throughput, startup/warmth, resource cost and failed attempts. 3. Exercise draft-model failure/incompatibility and fallback to healthy ordinary generation; confirm correct outputs and bounded actionable failure behavior. Null or negative speed benefit remains valid measurement. Missing target path, unexecuted comparison or stale-only fixtures leaves the task incomplete. 4. Deliver a reviewed keep/repair/retire recommendation tied to actual current evidence and limitations. This task completes retest and disposition; it does not silently undertake model/runtime repair or decommission running services. Implementing a subsequent change requires separately bounded work. PVF: deterministic benchmark accounting/output/fallback negatives plus actual hardware-dependent current-Runtime comparison; bounded approved CPU/GPU/disk/time, required milestone support gate. No paid/cloud/model acquisition or service disruption implied by creation. Retain nonzero executed denominators; neither a historical performance report nor a benchmark script alone closes the task. ## Global startup and proof boundary All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant. Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch. ## Inherited obligation ledger acceptance: `current_runtime_tested`, `correctness_preserved`, `performance_measured`, `disposition_evidence_bound`. pvf: `reproducible_benchmark`, `output_equivalence`, `fallback_test`. stop_conditions: `stale_fixture_only`, `correctness_regression`, `unsupported_speed_claim`. non_goals: `productization_without_requalification`. Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`. ## Canonical execution links Planning owner: #864. Creation/review batch: 6; this grouping adds no execution gate. Execution prerequisite: #864 (WP-01); accepted output is required before dependent execution. Reviewed creation source: `f5a4cd52eba0932852e8ae9c55585cee4e65f1e6`. This issue records a complete task; creation does not claim execution or acceptance."
    expected_output: ".csdlc/issues/905/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Start from existing `adl/tools/vllm_qwen_speculative_decoding_benchmark.py`, which records vLLM draft/accepted-token metrics, and historical provider review artifacts under `docs/milestones/v0.91.7/review/provider/`. The former `adl/src/speculative_decoding_prototype.rs` is absent at inspection; do not call a removed prototype the current Runtime path. Resolve current production routing through `adl/src/provider/` and `adl-runtime-kernel/`, record the actual integration point, and limit code changes to the retest harness. Own a current issue-local measurement packet and decision. Own bounded revisions to adl/tools/vllm_qwen_speculative_decoding_benchmark.py and proposed new adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; issue-local current measurements/decision and PVF inventory under .csdlc/evidence/905/. Provider/kernel paths are inspected only to resolve actual integration; no broad runtime repair. Historical provider review bytes remain immutable. Current evidence determines whether speculative decoding should be kept, repaired, or retired. Dependencies: WP-01. Numeric identities are attached during native creation."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: 1. Pin current Runtime/provider candidate, actual target/draft model and tokenizer revisions, engine/container/toolchain, available approved hardware, fixed corpus, sampling policy, repetitions and resource ceiling before the run. Source historical results as baseline context only. 2. Execute baseline and speculative routes through the current Runtime on the declared comparable corpus. Verify output equivalence under deterministic compatible settings or the declared correctness criterion where deterministic equality is inapplicable; reject any unsupported correctness claim. Capture accepted/proposed tokens where supported, total wall time, throughput, startup/warmth, resource cost and failed attempts. 3. Exercise draft-model failure/incompatibility and fallback to healthy ordinary generation; confirm correct outputs and bounded actionable failure behavior. Null or negative speed benefit remains valid measurement. Missing target path, unexecuted comparison or stale-only fixtures leaves the task incomplete. 4. Deliver a reviewed keep/repair/retire recommendation tied to actual current evidence and limitations. This task completes retest and disposition; it does not silently undertake model/runtime repair or decommission running services. Implementing a subsequent change requires separately bounded work. PVF: deterministic benchmark accounting/output/fallback negatives plus actual hardware-dependent current-Runtime comparison; bounded approved CPU/GPU/disk/time, required milestone support gate. No paid/cloud/model acquisition or service disruption implied by creation. Retain nonzero executed denominators; neither a historical performance report nor a benchmark script alone closes the task."
    expected_output: "validation evidence recorded in VPP/SOR"
    allowed_mode: "execution_after_approval"
  - id: "step-5"
    description: "Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges."
    expected_output: "reviewed SRP and truthful VPP/SOR"
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Confirm dependencies and starting state from the source issue prompt."
    status: "completed"
  - step: "Inspect repo inputs and target surfaces before editing."
    status: "completed"
  - step: "Implement the bounded deliverables only."
    status: "completed"
  - step: "Run focused validation and proof gates."
    status: "completed"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "completed"
affected_areas:
  - "v0922-speculative-decoding-retest"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "#864 CLOSED; PR #865 MERGED at f1c4e2a915c215797f0d2708cb8b0568f2b80b32, ancestor of selected main. Setup only under Sprint 6 umbrella #932; no implementation, model loading/download, provider/service mutation, paid allocation or hardware experiment is authorized by this preparation. All 69 startup gate and #864 are accepted. Keep implementation steps pending and require each future worker to create its child-bound execution goal. Current Runtime speculative route, compatible target/draft model and tokenizer revisions, engine and approved comparison resource ceiling remain unselected. Existing vLLM harness is historical starting material, not demonstrated current Runtime integration."
test_strategy:
  - "Planned local command after authoring: python3 adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; git diff --check. Deterministic accounting tests cover empty/zero or censored denominators, malformed/nonfinite metrics, missing output, corpus/model/revision mismatch, contradictory correctness and unsupported speed claims. Existing raw harness flags are --mode target_only|speculative, --target-model, --draft-model, --out, --repeats, --prompt-limit, --warmup-runs and --gpu-memory-utilization; inspect current help before authorized execution. Raw LLM invocation alone is insufficient: record the current Runtime command/path in SPP/VPP before running both actual routes. Retain every attempt; measure output correctness under declared compatible sampling rather than assuming exact equality. Real controlled draft failure/incompatibility must demonstrate healthy normal fallback. Actual hardware/provider execution and required CI remain separate from local fixture proof, with nonzero completed denominator."
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
notes: "Final comparable run reuses the exact Runtime agent identity across both arms, verifies the same immutable GGUF blob plus full model/tokenizer metadata, alternates A/B then B/A, and symmetrically preloads/prewarms every arm switch. Eight exact output pairs passed. Speculative execution was 12.98% faster end-to-end and 17.29% faster in decode throughput. Invalid draft configuration failed closed; operator-selected ordinary generation delivered. Accepted/proposed counters remain unavailable. Independent re-review, publication, CI, merge and terminal closeout remain pending."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][SPEC-RETEST] Speculative-decoding requalification`.

Verify accepted WP-01 and reconcile current harness/Runtime owner; locate a real current Runtime route for both target-only and speculative execution before making production claims; pin target/draft/tokenizer revisions, engine/container/toolchain, approved hardware, fixed corpus/repeats, correctness criterion, resource/cost ceilings and warmup policy; update only bounded retest harness and accounting tests; execute both routes on the same corpus retaining all outputs/timings/token counts and failed attempts, then controlled draft failure/incompatibility with healthy ordinary-generation fallback; compare correctness and benefit including null/negative results; deliver reviewed keep/repair/retire disposition with actual evidence, no runtime repair or decommissioning.

## PVF Lane Plan

- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`
- Planning lane source: `https://github.com/agent-logic/agent-design-language/issues/905 validation contract; docs/validation/pvf_lanes.json`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `10800`
- Estimated total tokens: `65000`
- Estimated validation seconds: `3600`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `low`
- Estimate data source: `Conservative retest harness/accounting and measurement estimate; hardware elapsed time reestimated after route/resource selection, not operator budget`
- Estimate source ref: `https://github.com/agent-logic/agent-design-language/issues/905`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [completed] Inspect repo inputs and target surfaces before editing.
3. [completed] Implement the bounded deliverables only.
4. [completed] Run focused validation and proof gates.
5. [completed] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: #864 CLOSED; PR #865 MERGED at f1c4e2a915c215797f0d2708cb8b0568f2b80b32, ancestor of selected main; no sibling experiment dependency.
2. Review repo inputs and scoped surfaces before editing: Complete source contract (live issue snapshot; preserve all requirements): # [v0.92.2][SPEC-RETEST] Speculative-decoding requalification ## One complete result Current evidence determines whether speculative decoding should be kept, repaired, or retired. Dependencies: WP-01. Numeric identities are attached during native creation. ## Current source and bounded retest Start from existing `adl/tools/vllm_qwen_speculative_decoding_benchmark.py`, which records vLLM draft/accepted-token metrics, and historical provider review artifacts under `docs/milestones/v0.91.7/review/provider/`. The former `adl/src/speculative_decoding_prototype.rs` is absent at inspection; do not call a removed prototype the current Runtime path. Resolve current production routing through `adl/src/provider/` and `adl-runtime-kernel/`, record the actual integration point, and limit code changes to the retest harness. Own a current issue-local measurement packet and decision. ## Executed acceptance 1. Pin current Runtime/provider candidate, actual target/draft model and tokenizer revisions, engine/container/toolchain, available approved hardware, fixed corpus, sampling policy, repetitions and resource ceiling before the run. Source historical results as baseline context only. 2. Execute baseline and speculative routes through the current Runtime on the declared comparable corpus. Verify output equivalence under deterministic compatible settings or the declared correctness criterion where deterministic equality is inapplicable; reject any unsupported correctness claim. Capture accepted/proposed tokens where supported, total wall time, throughput, startup/warmth, resource cost and failed attempts. 3. Exercise draft-model failure/incompatibility and fallback to healthy ordinary generation; confirm correct outputs and bounded actionable failure behavior. Null or negative speed benefit remains valid measurement. Missing target path, unexecuted comparison or stale-only fixtures leaves the task incomplete. 4. Deliver a reviewed keep/repair/retire recommendation tied to actual current evidence and limitations. This task completes retest and disposition; it does not silently undertake model/runtime repair or decommission running services. Implementing a subsequent change requires separately bounded work. PVF: deterministic benchmark accounting/output/fallback negatives plus actual hardware-dependent current-Runtime comparison; bounded approved CPU/GPU/disk/time, required milestone support gate. No paid/cloud/model acquisition or service disruption implied by creation. Retain nonzero executed denominators; neither a historical performance report nor a benchmark script alone closes the task. ## Global startup and proof boundary All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant. Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch. ## Inherited obligation ledger acceptance: `current_runtime_tested`, `correctness_preserved`, `performance_measured`, `disposition_evidence_bound`. pvf: `reproducible_benchmark`, `output_equivalence`, `fallback_test`. stop_conditions: `stale_fixture_only`, `correctness_regression`, `unsupported_speed_claim`. non_goals: `productization_without_requalification`. Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`. ## Canonical execution links Planning owner: #864. Creation/review batch: 6; this grouping adds no execution gate. Execution prerequisite: #864 (WP-01); accepted output is required before dependent execution. Reviewed creation source: `f5a4cd52eba0932852e8ae9c55585cee4e65f1e6`. This issue records a complete task; creation does not claim execution or acceptance.
3. Implement only the bounded deliverables: Start from existing `adl/tools/vllm_qwen_speculative_decoding_benchmark.py`, which records vLLM draft/accepted-token metrics, and historical provider review artifacts under `docs/milestones/v0.91.7/review/provider/`. The former `adl/src/speculative_decoding_prototype.rs` is absent at inspection; do not call a removed prototype the current Runtime path. Resolve current production routing through `adl/src/provider/` and `adl-runtime-kernel/`, record the actual integration point, and limit code changes to the retest harness. Own a current issue-local measurement packet and decision. Own bounded revisions to adl/tools/vllm_qwen_speculative_decoding_benchmark.py and proposed new adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; issue-local current measurements/decision and PVF inventory under .csdlc/evidence/905/. Provider/kernel paths are inspected only to resolve actual integration; no broad runtime repair. Historical provider review bytes remain immutable. Current evidence determines whether speculative decoding should be kept, repaired, or retired. Dependencies: WP-01. Numeric identities are attached during native creation.
4. Run focused proof gates for acceptance: 1. Pin current Runtime/provider candidate, actual target/draft model and tokenizer revisions, engine/container/toolchain, available approved hardware, fixed corpus, sampling policy, repetitions and resource ceiling before the run. Source historical results as baseline context only. 2. Execute baseline and speculative routes through the current Runtime on the declared comparable corpus. Verify output equivalence under deterministic compatible settings or the declared correctness criterion where deterministic equality is inapplicable; reject any unsupported correctness claim. Capture accepted/proposed tokens where supported, total wall time, throughput, startup/warmth, resource cost and failed attempts. 3. Exercise draft-model failure/incompatibility and fallback to healthy ordinary generation; confirm correct outputs and bounded actionable failure behavior. Null or negative speed benefit remains valid measurement. Missing target path, unexecuted comparison or stale-only fixtures leaves the task incomplete. 4. Deliver a reviewed keep/repair/retire recommendation tied to actual current evidence and limitations. This task completes retest and disposition; it does not silently undertake model/runtime repair or decommission running services. Implementing a subsequent change requires separately bounded work. PVF: deterministic benchmark accounting/output/fallback negatives plus actual hardware-dependent current-Runtime comparison; bounded approved CPU/GPU/disk/time, required milestone support gate. No paid/cloud/model acquisition or service disruption implied by creation. Retain nonzero executed denominators; neither a historical performance report nor a benchmark script alone closes the task.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- v0922-speculative-decoding-retest

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- #864 CLOSED; PR #865 MERGED at f1c4e2a915c215797f0d2708cb8b0568f2b80b32, ancestor of selected main. Setup only under Sprint 6 umbrella #932; no implementation, model loading/download, provider/service mutation, paid allocation or hardware experiment is authorized by this preparation. All 69 startup gate and #864 are accepted. Keep implementation steps pending and require each future worker to create its child-bound execution goal. Current Runtime speculative route, compatible target/draft model and tokenizer revisions, engine and approved comparison resource ceiling remain unselected. Existing vLLM harness is historical starting material, not demonstrated current Runtime integration.

## Test Strategy

- Planned local command after authoring: python3 adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; git diff --check. Deterministic accounting tests cover empty/zero or censored denominators, malformed/nonfinite metrics, missing output, corpus/model/revision mismatch, contradictory correctness and unsupported speed claims. Existing raw harness flags are --mode target_only|speculative, --target-model, --draft-model, --out, --repeats, --prompt-limit, --warmup-runs and --gpu-memory-utilization; inspect current help before authorized execution. Raw LLM invocation alone is insufficient: record the current Runtime command/path in SPP/VPP before running both actual routes. Retain every attempt; measure output correctness under declared compatible sampling rather than assuming exact equality. Real controlled draft failure/incompatibility must demonstrate healthy normal fallback. Actual hardware/provider execution and required CI remain separate from local fixture proof, with nonzero completed denominator.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Final comparable run reuses the exact Runtime agent identity across both arms, verifies the same immutable GGUF blob plus full model/tokenizer metadata, alternates A/B then B/A, and symmetrically preloads/prewarms every arm switch. Eight exact output pairs passed. Speculative execution was 12.98% faster end-to-end and 17.29% faster in decode throughput. Invalid draft configuration failed closed; operator-selected ordinary generation delivered. Accepted/proposed counters remain unavailable. Independent re-review, publication, CI, merge and terminal closeout remain pending.
