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
