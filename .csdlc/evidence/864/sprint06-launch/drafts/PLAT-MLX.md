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
