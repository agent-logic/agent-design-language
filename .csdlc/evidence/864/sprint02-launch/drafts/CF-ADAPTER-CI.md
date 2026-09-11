# [v0.92.2][CF-ADAPTER-CI] Ingest CI repository inputs into a repository packet

## One complete result and dependency

The CI ingestion entrypoint consumes declared checkout/revision/scope inputs and emits a usable common packet with truthful partial-input state. Depends only on CF-ADAPTER's merged contract (canonical number supplied at batch creation); shared command/path decisions belong to WP-01/#864. This task provides CI input acquisition, not architecture fitness gating (CF-GOV-CI).

## Source and proposed owned paths

Reuse the predecessor's selected installed command and `adl/src/codefriend/ingestion/` contract. Own proposed `adl/src/codefriend/ingestion/ci.rs`, narrow CLI registration, focused `adl/tests/codefriend_ci_ingestion.rs` and one named `.github/workflows/` smoke job chosen in the issue plan. The workflow must call the installed product path; it is not a competing ingestion implementation. Read the portable-adapter feature and adopted contracts before execution.

## Acceptance and proving cases

1. In an isolated CI-style checkout, invoke the production route with explicit revision and scope, read back its packet through the production reader, and compare to local acquisition of the same content. Environment metadata is provenance, not authority to substitute a different revision or scope.
2. Execute a real required CI smoke using the installed candidate on a bounded fixture checkout. Preserve exact candidate/source revisions and artifact digest; local emulation is local proof, not proof that CI ran. The job uses least necessary read-only repository permission and no provider secrets.
3. Reject missing, malformed, mismatched or unresolvable revision; handle shallow/partial checkout or missing files with explicit omissions/incomplete state. Test input bounds, path escape and credential-shaped environment values. Never serialize broad environment dumps, checkout absolute paths or runner credentials.
4. Ensure deterministic common packet semantics across relocated runner directories and equivalence with local source; retain run-specific provenance separately. Failed upload/transport or missing artifact cannot be represented as successful evidence delivery. Repository instructions remain inert data.
5. Publish concise declared-input/output and failure instructions. Complete the installed acquisition path and focused CI proof, not a workflow stub or schema. No review success or fitness-function result is inferred from ingestion success.

## PVF and exclusions

PVF: deterministic local contract/installed integration followed by required CI smoke; role: declared revision, packet parity and partial/credential negatives; resources: bounded runner CPU/disk, isolated fixture repository, no metered inference; gate: required Beta CI input route. Retain exact command and nonzero outcome in CI independently from local proof. Stop on missing input, failed required proof, contract conflict, credential capture, host-bound packet or unexecuted CI gate. Exclude CF-GOV-CI, GitHub API adapter, review/features, broad connectors and schema/scaffold-only closure.

## Execution and proof boundary

Use native C-SDLC v3 readiness, bound FastWork worktree and an issue-bound goal. Reconcile shared paths before edits. Issue creation does not satisfy dependencies or authorize unrelated work. Deliver the production behavior with necessary tests, failure handling and operator documentation; schema, scaffold, fixture-only caller, prewritten packet or zero executed scenarios cannot close this issue. Run focused proof and independent exact-head review; distinguish local proof from required CI and actual external observations. Record every new test's lane, proof role, determinism, resource profile and release-gate status in the coupled issue proof inventory. Machine-readable output stays on stdout; bounded redacted human diagnostics stay on stderr, with compatibility logging tested when exposed. Preserve source and credentials; no secrets in argv, packets, logs or retained evidence.

Canonical sources: `docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml`, `WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md` and the relevant feature document. Shared opening selections are recorded in `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md`: `adl codefriend`, selected new `adl/src/cli/codefriend_cmd.rs` and `adl/src/codefriend/` modules, macOS/Linux qualification, shared OpenAI Responses route with approved credential reference, and Vector revision `410da89a0ed42c523143da89fffeb7f6402833e0` limited to the seven dnsmsg-parser files plus three manifest/license context files (ten files/600 KiB total; 400 KiB per file). Preserve crate MIT and root MPL-2.0 notices. New paths below are intended implementation, not existing functionality. Do not build or run Vector or widen its source scope. Route selection is not paid-provider execution authority; exact execution model/profile is pinned before any proving call.

## Inherited obligation ledger

The following canonical tags must each map to an executed proof or explicit stop condition in the final issue record; they are not self-certifying labels.

acceptance: `ci_packet_consumed`, `local_contract_parity`, `portable_paths`, `bounded_inputs`, `partial_state_explicit`.

pvf: `ci_packet_consumed`, `local_contract_parity`, `missing_revision_rejected`, `partial_input_explicit`, `negative_inputs`, `portability`.

stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `host_bound_packet`, `credential_capture`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `jira`, `linear`, `slack`, `broad_workspace`.

