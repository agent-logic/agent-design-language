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
