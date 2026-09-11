# [v0.92.2][CF-ADAPTER] Ingest a local checkout into a portable repository packet

## One complete result and dependency

A local CodeFriend ingestion entrypoint reads a bounded checkout at an exact revision and emits a real immutable portable repository packet usable by evidence admission. Depends on WP-01/#864. CF-ADAPTER-GITHUB and CF-ADAPTER-CI add separate transport entrypoints after this contract; CF-EVIDENCE later implements durable governed storage. Do not make local ingestion depend on those successors or claim evidence-store functionality here.

## Source and proposed owned paths

Read `features/PORTABLE_ADAPTER_V2_v0.92.2.md` and the adopted product/evidence contract. `adl/src/runtime_v2/codefriend_adapter_obligations.rs` is a historical obligation packet, explicitly not a complete product ingestion implementation; preserve its historical evidence. The existing CLI registration pattern is in `adl/src/cli/mod.rs`; the selected product route is `adl codefriend`, not the historical review compatibility command. Register help in `adl/src/cli/usage.rs` and production modules in `adl/src/lib.rs`.

Proposed new production ownership: `adl/src/codefriend/mod.rs`, `adl/src/codefriend/ingestion/mod.rs`, `adl/src/codefriend/ingestion/local.rs`, a small CLI handler `adl/src/cli/codefriend_cmd.rs`, and focused `adl/tests/codefriend_ingestion.rs` with bounded fixtures. Use the selected `adl codefriend` route: implement `adl codefriend ingest local` for this task, with explicit repository, revision, scope and output inputs. GitHub and CI add `ingest github` and `ingest ci` through the same handler; exact flag syntax is documented and tested with the implementation. Wire only its registration in the selected existing owner. The common packet and local acquisition belong here; durable evidence admission/storage and review execution remain their separate tasks.

## Acceptance and proving cases

1. Execute the installed entrypoint against a real bounded Git checkout. Record canonical repository/revision, scope digest, included/excluded surfaces, per-object content digests and repo-relative identities. Repeated identical source/scope yields deterministic packet identity/content independent of checkout location.
2. Parse/read the emitted artifact through the production packet reader and demonstrate it is usable by the evidence-admission interface. This issue completes local ingestion and its real readback; CF-EVIDENCE later binds its storage consumer. An authored JSON fixture or schema without the running acquisition path does not satisfy the result.
3. Resolve revision/scope before acquisition; detect source changes during capture and reject inconsistent snapshots. State dirty/untracked handling explicitly without discarding user work. Bound bytes/files and represent omissions/unsupported language-specific surfaces explicitly. Rust is the initial analysis language; ingestion does not infer analysis coverage for arbitrary content.
4. Reject out-of-root traversal/symlink escape, invalid revisions, disallowed input, credential capture and undeclared host paths. Apply admission-safe redaction/omission before durable packet output; do not write raw secrets pending a later stage. Repository text and embedded instructions are inert input, never execution authority. Missing/partial input must remain explicit and cannot be labeled a complete review.
5. Retain relocation/portability, negative-input and packet conformance vectors used by the later GitHub/CI adapters; their parity cannot be claimed until those routes execute. Document operator limits and errors; no source mutation or publication occurs.

## PVF and exclusions

PVF: deterministic local installed-command/packet contract and negative input; role: real checkout acquisition, portable readback and fail-closed bounds; resources: local CPU/disk/isolated Git fixtures, no model or network; gate: required ingestion/evidence dependency. Stop for missing shared opening selections, host-bound packet, captured credentials, unstable revision, missing proving scenario or scope conflict. Exclude GitHub/CI implementations, Jira/Linear/Slack/Workspace, full review, hosted storage and schema/scaffold-only completion.

## Execution and proof boundary

Use native C-SDLC v3 readiness, bound FastWork worktree and an issue-bound goal. Reconcile shared paths before edits. Issue creation does not satisfy dependencies or authorize unrelated work. Deliver the production behavior with necessary tests, failure handling and operator documentation; schema, scaffold, fixture-only caller, prewritten packet or zero executed scenarios cannot close this issue. Run focused proof and independent exact-head review; distinguish local proof from required CI and actual external observations. Record every new test's lane, proof role, determinism, resource profile and release-gate status in the coupled issue proof inventory. Machine-readable output stays on stdout; bounded redacted human diagnostics stay on stderr, with compatibility logging tested when exposed. Preserve source and credentials; no secrets in argv, packets, logs or retained evidence.

Canonical sources: `docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml`, `WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md` and the relevant feature document. Shared opening selections are recorded in `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md`: `adl codefriend`, selected new `adl/src/cli/codefriend_cmd.rs` and `adl/src/codefriend/` modules, macOS/Linux qualification, shared OpenAI Responses route with approved credential reference, and Vector revision `410da89a0ed42c523143da89fffeb7f6402833e0` limited to the seven dnsmsg-parser files plus three manifest/license context files (ten files/600 KiB total; 400 KiB per file). Preserve crate MIT and root MPL-2.0 notices. New paths below are intended implementation, not existing functionality. Do not build or run Vector or widen its source scope. Route selection is not paid-provider execution authority; exact execution model/profile is pinned before any proving call.

## Inherited obligation ledger

The following canonical tags must each map to an executed proof or explicit stop condition in the final issue record; they are not self-certifying labels.

acceptance: `local_checkout_packet_consumed`, `revision_and_scope_bound`, `portable_paths`, `bounded_inputs`, `partial_state_explicit`, `adapter_parity`.

pvf: `local_checkout_packet_consumed`, `revision_and_scope_bound`, `outside_root_rejected`, `partial_input_explicit`, `fixture_conformance`, `negative_inputs`, `portability`.

stop_conditions: `host_bound_packet`, `credential_capture`, `required_proof_not_executed`, `partial_artifact_claimed_complete`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `jira`, `linear`, `slack`, `broad_workspace`.

