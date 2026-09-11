# [v0.92.2][CF-ADAPTER-GITHUB] Ingest a pinned GitHub revision or PR into a repository packet

## One complete result and dependency

The GitHub ingestion entrypoint resolves a repository revision or pull request to an exact immutable commit and emits the same consumable portable packet as local ingestion. Depends only on CF-ADAPTER's merged contract (canonical number supplied at batch creation); WP-01/#864 owns shared opening selections. It does not create or modify GitHub issues/PRs.

## Source and proposed owned paths

Use the shared product route selected by WP-01 and the predecessor's new `adl/src/codefriend/ingestion/mod.rs`/`local.rs` packet contract. Own proposed `adl/src/codefriend/ingestion/github.rs`, narrow registration in the selected `adl/src/cli/codefriend_cmd.rs`, and focused `adl/tests/codefriend_github_ingestion.rs`. These are new intended paths, resolved against CF-ADAPTER at execution; do not invent another packet schema or credential resolver. Read the adopted contract and portable-adapter feature. Native C-SDLC GitHub lifecycle authority is separate from this product's read-only repository acquisition.

## Acceptance and proving cases

1. For a pinned commit and a PR input, resolve exact repository identity and commit, fetch bounded source through the actual read-only acquisition path, and emit/read the production packet. A moving PR reference is pinned before reading; record original ref and resolved commit and detect disagreement/changed head rather than mixing revisions.
2. For the same revision/scope/content, compare packet semantics and evidence-object identities to local ingestion. Preserve provenance differences explicitly; credentials and machine-local cache paths cannot affect object identity or enter artifacts.
3. Execute controlled transport failures, not-found/forbidden/missing content, wrong repository/revision, rate limit/pagination or truncation and incomplete input. Fail or report explicit partial state with omissions; never silently mark complete. Honor bounded source limits and path/redaction checks inherited from local ingestion.
4. Secret references use the approved resolver and are never captured in URL/log/packet. Hostile repository content cannot authorize tools or GitHub writes. Test credential-shaped values and traversal/symlink inputs; retained errors are bounded and sanitized.
5. Retain nonzero actual transport/entrypoint execution and local parity. Deterministic fixtures use a controlled Git-compatible HTTP transport; any live GitHub readback is separately recorded and cannot be replaced by a hand-authored packet. No provider is invoked.

## PVF and exclusions

PVF: deterministic transport/installed-command integration plus optional separately authorized read-only external corroboration; role: exact pinning, packet parity and transport/credential negatives; resources: bounded local CPU/disk/loopback transport; gate: required Beta ingestion. Record external dependence/nondeterminism honestly if a live check is used. Stop for missing required input, failed proof, contract conflict, credential capture, host-bound packet or unverified revision. Exclude GitHub lifecycle writes, CI adapter, review implementation, broad connectors and scaffold-only completion.

## Execution and proof boundary

Use native C-SDLC v3 readiness, bound FastWork worktree and an issue-bound goal. Reconcile shared paths before edits. Issue creation does not satisfy dependencies or authorize unrelated work. Deliver the production behavior with necessary tests, failure handling and operator documentation; schema, scaffold, fixture-only caller, prewritten packet or zero executed scenarios cannot close this issue. Run focused proof and independent exact-head review; distinguish local proof from required CI and actual external observations. Record every new test's lane, proof role, determinism, resource profile and release-gate status in the coupled issue proof inventory. Machine-readable output stays on stdout; bounded redacted human diagnostics stay on stderr, with compatibility logging tested when exposed. Preserve source and credentials; no secrets in argv, packets, logs or retained evidence.

Canonical sources: `docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml`, `WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md` and the relevant feature document. Shared opening selections are recorded in `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md`: `adl codefriend`, selected new `adl/src/cli/codefriend_cmd.rs` and `adl/src/codefriend/` modules, macOS/Linux qualification, shared OpenAI Responses route with approved credential reference, and Vector revision `410da89a0ed42c523143da89fffeb7f6402833e0` limited to the seven dnsmsg-parser files plus three manifest/license context files (ten files/600 KiB total; 400 KiB per file). Preserve crate MIT and root MPL-2.0 notices. New paths below are intended implementation, not existing functionality. Do not build or run Vector or widen its source scope. Route selection is not paid-provider execution authority; exact execution model/profile is pinned before any proving call.

## Inherited obligation ledger

The following canonical tags must each map to an executed proof or explicit stop condition in the final issue record; they are not self-certifying labels.

acceptance: `pinned_revision_packet_consumed`, `local_contract_parity`, `portable_paths`, `bounded_inputs`, `partial_state_explicit`.

pvf: `pinned_revision_packet_consumed`, `local_contract_parity`, `wrong_revision_rejected`, `credential_capture_rejected`, `negative_inputs`, `portability`.

stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `host_bound_packet`, `credential_capture`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `jira`, `linear`, `slack`, `broad_workspace`.

