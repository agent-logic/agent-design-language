---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-pair-multinode-experiment-validation-plan"
issue: 904
task_id: "issue-0904"
run_id: "issue-0904"
version: "0.92.2"
title: "[v0.92.2][PLAT-PAIR] NVIDIA PAIR multi-node local-inference experiment"
branch: "codex/904-v0922-pair-multinode-experiment"
generated_at: "2026-09-12T00:18:16.017452+00:00"
card_status: "ready"
status: "planned"
initial_pvf_lane: "provider"
planned_pvf_lane: "provider"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "local_contract_and_approved_hardware_experiment"
validation_resource_profile: "Local isolated CPU/Python/Git harness tests plus separately approved two-node hardware/network/model bounds and cost ceiling; no provisioning, provider or hardware execution during preparation"
validation_family: "pair_raw_runtime_comparison"
validation_size_split: "Focused deterministic harness/accounting negatives, real hardware measurements and required CI reported separately"
expected_proof_cost: "Low-confidence estimate 1200 local validation seconds; excludes hardware experiment duration/cost, cold build, CI queue and independent review"
planned_validation_seconds: "1200"
planned_validation_tokens: "7000"
issue_goal_ref: "not_created; required before implementation"
sprint_goal_ref: "issue-932; Sprint 6 setup and coordination"
goal_metrics_rollup_ref: ".csdlc/evidence/904/goal-metrics.json (planned, absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/904"
  - kind: "stp"
    ref: ".csdlc/issues/904/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/904/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/904/cards/spp.md"
selected_lanes:
  - "provider; deterministic harness/integrity negatives plus hardware-dependent raw and Runtime route measurements; required milestone experiment gate; all runs pending"
parallel_groups:
  - "Future disjoint adapter/harness authoring may run in parallel after execution authorization. Serialize shared provider registration edits and any use of the same accelerator/models; no hardware runs during setup."
validation_commands:
  - "1. Before experiment execution, pin supported PAIR implementation/version, licensed model revision, two available approved nodes, transport, concurrency levels, baseline route, request corpus, time/memory/resource bounds and cost ceiling in the issue plan. These execution facts are not claimed selected merely by creating this issue. Missing suitable resources is a visible blocker, never synthetic success. 2. Run a real bounded raw PAIR route and the same requests through current Runtime v3 using canonical provider definitions. Retain exact code/model/hardware, per-request result, correctness/response checks, latency/throughput and resource measurements, startup and failure data. Repeat the declared same-corpus baseline for a fair comparison. 3. Exercise concurrent requests and actual controlled node-loss/unavailability with bounded recovery or declared fallback. Record failures, exclusions, cache/warmth and null/no-benefit outcomes; no caller-set success flags or reference trace substitutes for execution. 4. Produce one reviewed keep/repair/retire decision justified by complete observed data, including negative results. A negative benefit result can complete the experiment; missing raw or Runtime execution cannot. Separate this experimental decision from production readiness and preserve all evidence. PVF: deterministic harness/accounting/negative contracts plus hardware-dependent integration/measurement; resources restricted to explicitly approved nodes/network and budget, required milestone experiment gate. No automatic paid provisioning, model sharding, pooled-VRAM claim, production rollout or public benchmark marketing. Returned deficiencies become scoped follow-up work rather than unbounded repairs in this experiment. All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant. Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch. Proposed harness `adl/tools/pair_experiment.py` does not exist at preparation. Add its bounded deterministic contract tests at proposed `adl/tools/test_pair_experiment.py`; run `python3 adl/tools/test_pair_experiment.py` after authoring. Fixtures reject omitted requests, mismatched corpus/model/warmth/concurrency, fabricated success, missing Runtime or raw route, absent node-loss effects and invalid latency/resource accounting. These are local harness proofs, not hardware measurements. Resolve the implemented harness argv and exact approved environment only after #876 lands, then execute both real routes and same-corpus baseline with complete results and a current independent review. Do not guess a PAIR version, node identity, model license or performance result now. Use focused existing provider/profile tests only if their production owners are touched; inspect registered names first. Source currently routes through build_provider_for_id and provider_profile_materialization_projection; no PAIR provider type exists or is authorized. Keep ordinary provider behavior unchanged. Required raw/runtime/node-loss and null-benefit evidence cannot be replaced by test-only transports or a zero-match Rust filter. `git diff --check` accompanies focused proof. Required qualification harness repairs and scenario proofs are future work; existing source alone is not acceptance. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation."
failure_policy: "Missing, skipped, zero-scenario or failed required proof blocks acceptance. Replan absent/renamed tests explicitly. No synthetic benchmark, omitted request, static trace, caller-set success, mismatched baseline or inferred node/network permission. Actual raw and Runtime effects, null-benefit detection and independent review remain required. Keep machine-readable stdout and redacted stderr; preserve failed evidence and refresh affected exact-head review."
notes: "#876 CLOSED; PR #953 MERGED at b6d110c84e11253b392d0bb078f2fb33a36b9a0c, ancestor of selected main. Setup only under Sprint 6 umbrella #932; no implementation, model loading/download, provider/service mutation, paid allocation or hardware experiment is authorized by this preparation. All 69 startup gate and #864 are accepted. Keep implementation steps pending and require each future worker to create its child-bound execution goal. Two approved NVIDIA nodes, supported PAIR implementation/version, licensed model, transport, resource/cost ceiling and controlled node-loss scope remain unselected. Local Apple hardware does not satisfy multi-node NVIDIA proof."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

After accepted #876 and current shared-provider owner review, select a supported PAIR implementation/version, licensed pinned model, two approved nodes and bounded transport/resource/cost profile; define one same-request-corpus baseline and correctness/accounting rules; implement a reproducible harness using canonical provider definitions without a new provider type; execute actual raw PAIR and Runtime v3 routes with repeated baseline, concurrency, controlled node-loss/recovery and complete per-request measurements; independently review null/negative/positive benefit data and issue one evidence-bound keep/repair/retire decision without production rollout.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `provider`
- Planned PVF lane for execution: `provider`

## Selected Validation Lanes

- provider; deterministic harness/integrity negatives plus hardware-dependent raw and Runtime route measurements; required milestone experiment gate; all runs pending

## Parallelization Plan

- Parallel groups: Future disjoint adapter/harness authoring may run in parallel after execution authorization. Serialize shared provider registration edits and any use of the same accelerator/models; no hardware runs during setup.
- Validation runtime class: `local_contract_and_approved_hardware_experiment`
- Validation resource profile: `Local isolated CPU/Python/Git harness tests plus separately approved two-node hardware/network/model bounds and cost ceiling; no provisioning, provider or hardware execution during preparation`
- Validation family: `pair_raw_runtime_comparison`
- Validation size split: `Focused deterministic harness/accounting negatives, real hardware measurements and required CI reported separately`

## Goal Accounting Hooks

- Issue goal ref: `not_created; required before implementation`
- Sprint goal ref: `issue-932; Sprint 6 setup and coordination`
- Goal metrics rollup ref: `.csdlc/evidence/904/goal-metrics.json (planned, absent until execution)`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Low-confidence estimate 1200 local validation seconds; excludes hardware experiment duration/cost, cold build, CI queue and independent review`
- Planned validation seconds: `1200`
- Planned validation token budget: `7000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- 1. Before experiment execution, pin supported PAIR implementation/version, licensed model revision, two available approved nodes, transport, concurrency levels, baseline route, request corpus, time/memory/resource bounds and cost ceiling in the issue plan. These execution facts are not claimed selected merely by creating this issue. Missing suitable resources is a visible blocker, never synthetic success. 2. Run a real bounded raw PAIR route and the same requests through current Runtime v3 using canonical provider definitions. Retain exact code/model/hardware, per-request result, correctness/response checks, latency/throughput and resource measurements, startup and failure data. Repeat the declared same-corpus baseline for a fair comparison. 3. Exercise concurrent requests and actual controlled node-loss/unavailability with bounded recovery or declared fallback. Record failures, exclusions, cache/warmth and null/no-benefit outcomes; no caller-set success flags or reference trace substitutes for execution. 4. Produce one reviewed keep/repair/retire decision justified by complete observed data, including negative results. A negative benefit result can complete the experiment; missing raw or Runtime execution cannot. Separate this experimental decision from production readiness and preserve all evidence. PVF: deterministic harness/accounting/negative contracts plus hardware-dependent integration/measurement; resources restricted to explicitly approved nodes/network and budget, required milestone experiment gate. No automatic paid provisioning, model sharding, pooled-VRAM claim, production rollout or public benchmark marketing. Returned deficiencies become scoped follow-up work rather than unbounded repairs in this experiment. All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant. Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch. Proposed harness `adl/tools/pair_experiment.py` does not exist at preparation. Add its bounded deterministic contract tests at proposed `adl/tools/test_pair_experiment.py`; run `python3 adl/tools/test_pair_experiment.py` after authoring. Fixtures reject omitted requests, mismatched corpus/model/warmth/concurrency, fabricated success, missing Runtime or raw route, absent node-loss effects and invalid latency/resource accounting. These are local harness proofs, not hardware measurements. Resolve the implemented harness argv and exact approved environment only after #876 lands, then execute both real routes and same-corpus baseline with complete results and a current independent review. Do not guess a PAIR version, node identity, model license or performance result now. Use focused existing provider/profile tests only if their production owners are touched; inspect registered names first. Source currently routes through build_provider_for_id and provider_profile_materialization_projection; no PAIR provider type exists or is authorized. Keep ordinary provider behavior unchanged. Required raw/runtime/node-loss and null-benefit evidence cannot be replaced by test-only transports or a zero-match Rust filter. `git diff --check` accompanies focused proof. Required qualification harness repairs and scenario proofs are future work; existing source alone is not acceptance. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

## Failure Semantics

- Missing, skipped, zero-scenario or failed required proof blocks acceptance. Replan absent/renamed tests explicitly. No synthetic benchmark, omitted request, static trace, caller-set success, mismatched baseline or inferred node/network permission. Actual raw and Runtime effects, null-benefit detection and independent review remain required. Keep machine-readable stdout and redacted stderr; preserve failed evidence and refresh affected exact-head review.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

#876 CLOSED; PR #953 MERGED at b6d110c84e11253b392d0bb078f2fb33a36b9a0c, ancestor of selected main. Setup only under Sprint 6 umbrella #932; no implementation, model loading/download, provider/service mutation, paid allocation or hardware experiment is authorized by this preparation. All 69 startup gate and #864 are accepted. Keep implementation steps pending and require each future worker to create its child-bound execution goal. Two approved NVIDIA nodes, supported PAIR implementation/version, licensed model, transport, resource/cost ceiling and controlled node-loss scope remain unselected. Local Apple hardware does not satisfy multi-node NVIDIA proof.
