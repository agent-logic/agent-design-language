# [v0.92.2][PLAT-UTS] Install a versioned UTS package consumed by Runtime tool dispatch

## One complete result and dependency

Install one versioned UTS package and use it in actual Runtime ACC/UTS tool dispatch with explicit supported-version compatibility. Depends on WP-01/#864. This is one working package/consumer delivery; docs, schema and migration notes support that result.

## Source and owned paths

Read `docs/specs/uts/README.md`, `UTS_V1.0_SCHEMA.md`, `UTS_V1.1_SCHEMA.md`, and `adl-spec/schemas/uts/`. Existing types are in `adl/src/uts.rs`; conformance in `adl/src/uts_conformance.rs`; ACC compilation in `adl/src/uts_acc_compiler/`; production consumers include `adl/src/resident_tool_execution.rs`, `adl/src/tool_registry.rs` and `adl/src/governed_executor.rs`. UTS describes tools; ACC retains runtime authority. Documentation calls v1 the guaranteed baseline while source also includes v1.1 types; reconcile actual supported semantics rather than inferring complete v1.1 implementation from type presence.

Selected package location: a new in-repository `adl-uts/` Rust crate, initial package version `0.1.0`, with manifest, canonical types/schema assets and focused tests. Package version and UTS schema version are distinct. Preserve existing supported `uts.v1` and `uts.v1.1` serialization/types and declare their actual implemented compatibility separately; introducing this package does not claim every proposed v1.1 semantic is implemented. Make `adl/Cargo.toml` consume the package and migrate only necessary UTS imports/reexports and the named production dispatch path. Preserve ACC enforcement and unrelated APIs. No external registry publication or standalone repository creation is implied.

## Acceptance and proving cases

1. Build/package/install the selected version from a clean bounded checkout into an isolated consumer environment. One canonical package owns the supported schema/type contract; no silent private fork remains in the named consumer. Include complete installation and migration instructions.
2. Exercise a real Runtime-owned governed tool dispatch that loads/validates the packaged contract, obtains an ACC-governed decision and executes an actual bounded read-only tool. Retain invocation, package/schema version, authority decision and real result. A compiler fixture or package import test alone does not prove consumer integration.
3. Define and execute compatible-version cases and reject unsupported versions, malformed tool declarations, registry-binding mismatch and authority/policy denial before tool effects. Preserve side-effect/replay/idempotence/data-sensitivity semantics; package/schema compatibility cannot grant execution authority.
4. Run schema conformance and named-consumer parity before/after migration. Explicitly distinguish v1 guaranteed semantics from any separately implemented v1.1 surface; reject silent breaking changes. Record package artifact/version and successful isolated consumption.
5. Complete focused source docs and independent review. A published standard document, unused library or fixture-only consumer cannot close the issue.

## PVF and exclusions

PVF: deterministic local schema/package/Runtime integration; role: installability, real consumer and compatibility/authority rejection; resources: bounded CPU/disk and a read-only local tool, no provider/cloud spend; gate: required milestone support result. Record nonzero executed scenarios and exact package/consumer revisions. Stop on ambiguous contract authority, silent breaking behavior, unauthorized tool effect or missing consumer proof. Exclude universal ecosystem standard claims, external package publication, wholesale ACC redesign and unrelated Runtime refactoring.

## Execution and proof boundary

Use native C-SDLC v3 readiness, bound FastWork worktree and an issue-bound goal. Reconcile shared paths before edits. Issue creation does not satisfy dependencies or authorize unrelated work. Deliver the production behavior with necessary tests, failure handling and operator documentation; schema, scaffold, fixture-only caller, prewritten packet or zero executed scenarios cannot close this issue. Run focused proof and independent exact-head review; distinguish local proof from required CI and actual external observations. Record every new test's lane, proof role, determinism, resource profile and release-gate status in the coupled issue proof inventory. Machine-readable output stays on stdout; bounded redacted human diagnostics stay on stderr, with compatibility logging tested when exposed. Preserve source and credentials; no secrets in argv, packets, logs or retained evidence.

Canonical sources: `docs/milestones/v0.92.2/WP_ISSUE_WAVE_v0.92.2.yaml`, `WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md` and the relevant feature document. Shared opening selections are recorded in `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md`: `adl codefriend`, selected new `adl/src/cli/codefriend_cmd.rs` and `adl/src/codefriend/` modules, macOS/Linux qualification, shared OpenAI Responses route with approved credential reference, and Vector revision `410da89a0ed42c523143da89fffeb7f6402833e0` limited to the seven dnsmsg-parser files plus three manifest/license context files (ten files/600 KiB total; 400 KiB per file). Preserve crate MIT and root MPL-2.0 notices. New paths below are intended implementation, not existing functionality. Do not build or run Vector or widen its source scope. Route selection is not paid-provider execution authority; exact execution model/profile is pinned before any proving call.

## Inherited obligation ledger

The following canonical tags must each map to an executed proof or explicit stop condition in the final issue record; they are not self-certifying labels.

acceptance: `canonical_contract`, `supported_consumers_named`, `compatibility_explicit`, `package_installed`, `runtime_consumer_executes_tool`, `version_compatibility_checked`.

pvf: `schema_conformance`, `consumer_fixture`, `compatibility_check`, `package_installed`, `runtime_consumer_executes_tool`, `version_compatibility_checked`, `document_only_standard_rejected`, `fixture_only_consumer_rejected`.

stop_conditions: `ambiguous_authority`, `silent_breaking_change`, `required_proof_not_executed`, `partial_artifact_claimed_complete`.

non_goals: `universal_ecosystem_standard`.
