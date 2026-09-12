---
issue_card_schema: adl.issue.v1
wp: "PLAT-PAIR"
slug: "v0922-pair-multinode-experiment"
title: "[v0.92.2][PLAT-PAIR] NVIDIA PAIR multi-node local-inference experiment"
labels:
  - "track:roadmap"
issue_number: 904
generated_at: "2026-09-12T00:18:16.017452+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "0.92.2"
required_outcome_type:
  - "executed_hardware_experiment_and_reviewed_decision"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/904"
canonical_files: []
demo_required: true
demo_names: []
issue_graph_notes:
  - "#876 CLOSED; PR #953 MERGED at b6d110c84e11253b392d0bb078f2fb33a36b9a0c, ancestor of selected main; no sibling experiment dependency."
pr_start:
  enabled: true
  slug: "v0922-pair-multinode-experiment"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-12T00:18:16.017452+00:00

# Structured Task Prompt

## Summary

A bounded experiment determines whether NVIDIA PAIR improves multi-node local inference for ADL without becoming a provider type.

Dependencies: PLAT-PROVIDER. Numeric identities are attached during native creation.

## Goal

A bounded experiment determines whether NVIDIA PAIR improves multi-node local inference for ADL without becoming a provider type.

Dependencies: PLAT-PROVIDER. Numeric identities are attached during native creation.

## Required Outcome

A bounded experiment determines whether NVIDIA PAIR improves multi-node local inference for ADL without becoming a provider type.

Dependencies: PLAT-PROVIDER. Numeric identities are attached during native creation.

## Deliverables

Use canonical definitions from PLAT-PROVIDER and existing provider routing in `adl/src/provider/mod.rs`, `provider/profiles.rs`, `provider/http_family.rs` and `provider/local.rs`. Own a proposed reproducible harness `adl/tools/pair_experiment.py` and issue-local raw results/decision. Read current Runtime routes before wiring the experiment; add no new PAIR provider type and do not rewrite production provider behavior. The cited historical TBD plan may be absent; do not invent its contents.

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

## Repo Inputs

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

## Dependencies

#876 CLOSED; PR #953 MERGED at b6d110c84e11253b392d0bb078f2fb33a36b9a0c, ancestor of selected main; no sibling experiment dependency.

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

## Demo Expectations

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

## Non-goals

acceptance: `canonical_provider_definition_consumed`, `raw_pair_route_proven`, `runtime_path_compared`, `concurrency_and_failure_behavior_measured`, `no_provider_type_added`.

pvf: `raw_pair_smoke`, `runtime_v3_pair_smoke`, `concurrent_request_comparison`, `node_loss_fallback`, `null_benefit_detection`.

stop_conditions: `provider_contract_bypass`, `paid_or_network_mutation_without_authority`, `unsupported_performance_claim`.

non_goals: `model_sharding`, `pooled_vram`, `production_rollout`.

Canonical sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` and `CREATION_SELECTIONS_v0.92.2.md`.

All 69 milestone issues must be created and reviewed before any implementation begins, as directed by the operator. Creation/review batches add no execution dependencies beyond that global startup gate and each task's declared prerequisites. Use native C-SDLC v3, current authority/readiness, a bound FastWork worktree and issue-bound goal. Reconcile actual owners before shared-path edits. Preserve repository/provider credentials and inspected source; stdout carries machine output and stderr redacted human diagnostics, including tested compatibility logging when relevant.

Deliver one complete result with required tests, failure handling and documentation; no schema/scaffold/unused library/zero-test or authored-packet-only completion. Classify every new proof case by lane, role, determinism, resources and release gate; distinguish local fixture proof, actual hardware/provider runs and required CI. Required proof not executed remains not proved, even when issue creation succeeds. No implementation, paid execution, activation or remote mutation is performed by this issue-creation batch.

## Issue-Graph Notes

#876 CLOSED; PR #953 MERGED at b6d110c84e11253b392d0bb078f2fb33a36b9a0c, ancestor of selected main; no sibling experiment dependency.

## Notes

#876 CLOSED; PR #953 MERGED at b6d110c84e11253b392d0bb078f2fb33a36b9a0c, ancestor of selected main. Setup only under Sprint 6 umbrella #932; no implementation, model loading/download, provider/service mutation, paid allocation or hardware experiment is authorized by this preparation. All 69 startup gate and #864 are accepted. Keep implementation steps pending and require each future worker to create its child-bound execution goal. Two approved NVIDIA nodes, supported PAIR implementation/version, licensed model, transport, resource/cost ceiling and controlled node-loss scope remain unselected. Local Apple hardware does not satisfy multi-node NVIDIA proof.

## Tooling Notes

Use native v3 with current authenticated authority. Preparation is stored only in resolved Git metadata. Schedule dependencies false; do not bind until accepted prerequisites and owner recheck. Create an issue-bound session goal before implementation. Do not hand-edit rendered cards.
