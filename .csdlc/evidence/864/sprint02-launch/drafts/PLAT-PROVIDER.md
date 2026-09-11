# [v0.92.2][PLAT-PROVIDER] Consume validated editable provider definitions

## One complete result and dependencies

The Runtime provider-definition loader consumes operator-editable endpoint/profile data through the existing adapter boundary, atomically retains the last-known-good snapshot on invalid replacement, and exposes the change to subsequent real production dispatch. Depends on WP-01/#864, RT-COST/#854 and the verified merged provider hot-loading predecessor #622. RT-PROVIDER/#855 separately owns dynamic agent lifecycle; this issue supplies configuration, not attach/detach implementation.

## Source and owned paths

Extend `adl/src/provider/reload.rs`, `adl/src/provider/profiles.rs`, and the minimum wiring in `adl/src/provider/mod.rs` and `adl/src/execute/runner.rs`. Read `docs/providers/provider-profile-hot-loading.md` and `docs/provider/inference-profiles.md`: existing `ProviderReloadOwner`/`ProviderReloadSnapshot` already validate a provider-only sidecar and use the kernel watcher. Reuse that production owner; do not add another registry/watcher. Own focused reload/profile tests and accompanying provider docs. Add a named data/schema/example file only after inventorying the current format and recording its exact path; do not expand into a provider rewrite.

## Acceptance and proving cases

1. Load valid editable definitions using the production entrypoint, execute a local controlled provider request and prove the observed endpoint/model/profile matches the selected validated snapshot. Edit definitions without rebuilding; a later dispatch uses the new complete snapshot. In-flight dispatch retains its original snapshot and concurrent readers never observe a mixed map.
2. Preserve existing endpoint/profile behavior, declared compatibility and provider generations. Separate instance data from adapter behavior; document supported fields and explicit unsupported cases. No hardcoded instance branch may stand in for consumption.
3. Through the production loader reject malformed, unsupported, incomplete and credential-shaped definitions, including secret values hidden under neutral nested keys. Keep only approved credential references. Invalid initial configuration fails closed; invalid replacement retains the prior whole snapshot and yields a bounded diagnostic. Execute last-known-good readback, not just schema validation.
4. Verify #622 hot-load authority and #854 cost-control behavior are consumed. Reload must not introduce recurring metered inference probes. Tests use a deterministic local provider endpoint and make no hosted-provider qualification claim.
5. Retain executed endpoint/profile parity, reload concurrency, schema negatives and secret-redaction proof. A configuration schema or unused parser is insufficient; real dispatch is the consumer.

## PVF and exclusions

PVF lane: deterministic local provider integration/contract; role: production definition consumption, atomic reload and invalid-input rejection; resources: bounded local CPU/filesystem and controlled loopback endpoint, no paid inference; gate: required provider-platform completion and downstream RT-PROVIDER input. Live/provider-cost execution needs separate explicit authority. Stop on missing hot-load authority, unresolved owner collision, hardcoded instance data, credential capture, failed or missing proving scenario, or incompatible unreviewed format changes. Exclude provider behavior rewrite, agent lifecycle, MLX/PAIR implementation, benchmark marketing, broad Runtime changes and schema-only completion.

## Execution and proof boundary

Use native C-SDLC v3 readiness, bound FastWork worktree and an issue-bound goal. Reconcile shared paths before edits. Issue creation does not satisfy dependencies or authorize unrelated work. Deliver the production behavior with necessary tests, failure handling and operator documentation; schema, scaffold, fixture-only caller, prewritten packet or zero executed scenarios cannot close this issue. Run focused proof and independent exact-head review; distinguish local proof from required CI and actual external observations. Record every new test's lane, proof role, determinism, resource profile and release-gate status in the coupled issue proof inventory. Machine-readable output stays on stdout; bounded redacted human diagnostics stay on stderr, with compatibility logging tested when exposed. Preserve source and credentials; no secrets in argv, packets, logs or retained evidence.

Canonical sources: `docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml`, `WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md` and the relevant feature document. Shared opening selections are recorded in `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md`: `adl codefriend`, selected new `adl/src/cli/codefriend_cmd.rs` and `adl/src/codefriend/` modules, macOS/Linux qualification, shared OpenAI Responses route with approved credential reference, and Vector revision `410da89a0ed42c523143da89fffeb7f6402833e0` limited to the seven dnsmsg-parser files plus three manifest/license context files (ten files/600 KiB total; 400 KiB per file). Preserve crate MIT and root MPL-2.0 notices. New paths below are intended implementation, not existing functionality. Do not build or run Vector or widen its source scope. Route selection is not paid-provider execution authority; exact execution model/profile is pinned before any proving call.

## Inherited obligation ledger

The following canonical tags must each map to an executed proof or explicit stop condition in the final issue record; they are not self-certifying labels.

acceptance: `editable_definition_consumed`, `endpoint_profile_parity`, `last_known_good`, `behavior_data_separated`, `definitions_validated`, `endpoint_and_profile_parity`.

pvf: `editable_definition_consumed`, `endpoint_profile_parity`, `last_known_good`, `secret_value_in_config_rejected`, `invalid_reload_rejected`, `schema_negative_suite`, `provider_fixture_parity`.

stop_conditions: `hardcoded_instance_data`, `credential_in_config`, `missing_v0921_hotload_authority`, `required_proof_not_executed`, `partial_artifact_claimed_complete`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `provider_rewrite`, `public_benchmark_marketing`.
