---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-speculative-decoding-retest-validation-plan"
issue: 905
task_id: "issue-0905"
run_id: "issue-0905"
version: "0.92.2"
title: "[v0.92.2][SPEC-RETEST] Speculative-decoding requalification"
branch: "codex/905-v0922-speculative-decoding-retest"
generated_at: "2026-09-12T00:18:14.558347+00:00"
card_status: "ready"
status: "completed"
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "local_deterministic_accounting_plus_actual_hardware_runtime_comparison"
validation_resource_profile: "Local Apple M4 Pro: 14 CPU cores, 20 GPU cores, 64 GB unified memory; Ollama 0.32.14; resident Qwen3.5:9b immutable GGUF blob; no download or paid/cloud allocation."
validation_family: "speculative_current_runtime_requalification"
validation_size_split: "focused per source acceptance; no reflexive full workspace suite"
expected_proof_cost: "Planning estimate: local CPU/disk plus normal CI; reestimate after predecessor integration, not a budget authorization"
planned_validation_seconds: "3600"
planned_validation_tokens: "20000"
issue_goal_ref: "Active goal: complete #905 current Runtime requalification, publish and close, then close Sprint 6 #932."
sprint_goal_ref: "issue-932; Sprint 6 setup and coordination"
goal_metrics_rollup_ref: ".csdlc/evidence/905/goal-metrics.json (planned; absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/905"
  - kind: "stp"
    ref: ".csdlc/issues/905/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/905/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/905/cards/spp.md"
selected_lanes:
  - "runtime; Planned local command after authoring: python3 adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; git diff --check. Deterministic accounting tests cover empty/zero or censored denominators, malformed/nonfinite metrics, missing output, corpus/model/revision mismatch, contradictory correctness and unsupported speed claims. Existing raw harness flags are --mode target_only|speculative, --target-model, --draft-model, --out, --repeats, --prompt-limit, --warmup-runs and --gpu-memory-utilization; inspect current help before authorized execution. Raw LLM invocation alone is insufficient: record the current Runtime command/path in SPP/VPP before running both actual routes. Retain every attempt; measure output correctness under declared compatible sampling rather than assuming exact equality. Real controlled draft failure/incompatibility must demonstrate healthy normal fallback. Actual hardware/provider execution and required CI remain separate from local fixture proof, with nonzero completed denominator."
parallel_groups:
  - "Future disjoint adapter/harness authoring may run in parallel after execution authorization. Serialize shared provider registration edits and any use of the same accelerator/models; no hardware runs during setup."
validation_commands:
  - "python3 adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; python3 -m py_compile adl/tools/issue905_runtime_speculative_retest.py; bundled Python adl/tools/issue905_runtime_speculative_retest.py with current Runtime binaries, source model Qwen3.5:9b, output .adl/runs/905/runtime-run-14 and four counterbalanced repeats; git diff --check; native validate. Final tracked report: .csdlc/evidence/905/RUNTIME_RETEST.json."
failure_policy: "Required failures, skipped or zero-test proof block acceptance. Preserve guards; record durable anomalies; repair and rerun affected proof and independent exact-head review. CI evidence is separate from local proof."
notes: "Correctness and bounded recovery passed. Performance robustness failed: end-to-end wins 1/4 with -5.96% median benefit; decode wins 2/4 with -11.33% median benefit. Positive aggregates are not qualification evidence. Accepted/proposed counters are unavailable. Repair requires a stationarity criterion, larger paired denominator and robust aggregate/confidence rule."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Verify accepted WP-01 and reconcile current harness/Runtime owner; locate a real current Runtime route for both target-only and speculative execution before making production claims; pin target/draft/tokenizer revisions, engine/container/toolchain, approved hardware, fixed corpus/repeats, correctness criterion, resource/cost ceilings and warmup policy; update only bounded retest harness and accounting tests; execute both routes on the same corpus retaining all outputs/timings/token counts and failed attempts, then controlled draft failure/incompatibility with healthy ordinary-generation fallback; compare correctness and benefit including null/negative results; deliver reviewed keep/repair/retire disposition with actual evidence, no runtime repair or decommissioning.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`

## Selected Validation Lanes

- runtime; Planned local command after authoring: python3 adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; git diff --check. Deterministic accounting tests cover empty/zero or censored denominators, malformed/nonfinite metrics, missing output, corpus/model/revision mismatch, contradictory correctness and unsupported speed claims. Existing raw harness flags are --mode target_only|speculative, --target-model, --draft-model, --out, --repeats, --prompt-limit, --warmup-runs and --gpu-memory-utilization; inspect current help before authorized execution. Raw LLM invocation alone is insufficient: record the current Runtime command/path in SPP/VPP before running both actual routes. Retain every attempt; measure output correctness under declared compatible sampling rather than assuming exact equality. Real controlled draft failure/incompatibility must demonstrate healthy normal fallback. Actual hardware/provider execution and required CI remain separate from local fixture proof, with nonzero completed denominator.

## Parallelization Plan

- Parallel groups: Future disjoint adapter/harness authoring may run in parallel after execution authorization. Serialize shared provider registration edits and any use of the same accelerator/models; no hardware runs during setup.
- Validation runtime class: `local_deterministic_accounting_plus_actual_hardware_runtime_comparison`
- Validation resource profile: `Local Apple M4 Pro: 14 CPU cores, 20 GPU cores, 64 GB unified memory; Ollama 0.32.14; resident Qwen3.5:9b immutable GGUF blob; no download or paid/cloud allocation.`
- Validation family: `speculative_current_runtime_requalification`
- Validation size split: `focused per source acceptance; no reflexive full workspace suite`

## Goal Accounting Hooks

- Issue goal ref: `Active goal: complete #905 current Runtime requalification, publish and close, then close Sprint 6 #932.`
- Sprint goal ref: `issue-932; Sprint 6 setup and coordination`
- Goal metrics rollup ref: `.csdlc/evidence/905/goal-metrics.json (planned; absent until execution)`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Planning estimate: local CPU/disk plus normal CI; reestimate after predecessor integration, not a budget authorization`
- Planned validation seconds: `3600`
- Planned validation token budget: `20000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- python3 adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; python3 -m py_compile adl/tools/issue905_runtime_speculative_retest.py; bundled Python adl/tools/issue905_runtime_speculative_retest.py with current Runtime binaries, source model Qwen3.5:9b, output .adl/runs/905/runtime-run-14 and four counterbalanced repeats; git diff --check; native validate. Final tracked report: .csdlc/evidence/905/RUNTIME_RETEST.json.

## Failure Semantics

- Required failures, skipped or zero-test proof block acceptance. Preserve guards; record durable anomalies; repair and rerun affected proof and independent exact-head review. CI evidence is separate from local proof.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Correctness and bounded recovery passed. Performance robustness failed: end-to-end wins 1/4 with -5.96% median benefit; decode wins 2/4 with -11.33% median benefit. Positive aggregates are not qualification evidence. Accepted/proposed counters are unavailable. Repair requires a stationarity criterion, larger paired denominator and robust aggregate/confidence rule.
