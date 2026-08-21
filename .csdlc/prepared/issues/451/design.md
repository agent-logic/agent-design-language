# Issue #451 Design — Production Birthday And Runtime Feature Convergence

## Outcome

Compose the merged #446 and #448-#450 authorities into one production resident
activation transaction. The transaction must deny before readiness, commit one
birthday and witness packet for one identity lineage, restore without replaying
the birthday, and continue useful resident work with all governed authorities.

## Source-grounded baseline

- `LiveAssembly` provisions verified Memory Palace, capability, cognitive
  profile, resident-cycle, continuity, and birth-witness authorities.
- `resident_cycle` binds identity, continuity, capability and governed profile
  but does not yet commit a birthday transaction.
- `adaptive_learning` produces restart-safe governed learning receipts but is
  not part of the resident-cycle record.
- `long_lived_agent` performs the real resident work cycle and ACC-governed tool
  execution, but does not consume one complete birthday activation aggregate.
- `birth_witness` validates a candidate and witness set and can commit a receipt,
  but no production resident path owns exactly-once activation and recovery.

## Design

Add one versioned production-birthday aggregate and service at the Runtime
kernel boundary. The input contains only verified or independently validated
authorities: identity, continuity, Memory Palace, resident cycle/capability/
profile, adaptive-learning terminal receipt/history, ACC tool-authority receipt,
birthday candidate, witness attestations, implementation revision, and trusted
time. The service revalidates every cross-binding before it writes anything.

The ACC input is not the current unsigned `ResidentToolReceiptV1` by itself.
This issue owns bounded receipt-contract hardening in
`adl/src/resident_tool_execution.rs`: deny unknown fields; bind resident,
cycle, capability/profile, continuity, policy, request/result, implementation
revision, and terminal decision into canonical bytes; and authenticate those
bytes through a Runtime-provisioned authority and canonical validator. The
birthday service accepts only the validated handle. Stored JSON or a recomputed
storage hash is never execution provenance.

The durable store is an explicit state machine keyed by identity lineage and a
canonical transaction digest. `absent -> pending(owner, transaction, input
digest) -> committed(generation, receipt digest)` is the only success path.
One same-directory create-new ownership file provides CAS exclusion across
independent store/service instances. Intent, witness packet, and committed
receipt are generation-scoped create-new files; each file is synchronized, the
final pointer is atomically renamed and parent-synchronized, and ownership is
released last. Existing committed state is returned only as an idempotent replay
of the exact same transaction. A live foreign owner conflicts; abandoned pending
state is recovered only after validating every retained byte and either
finishing the exact transaction or retaining a typed corrupt/indeterminate
failure. Changed input, rollback, stale generation, copied identity, or a
conflicting witness fails closed.

Deterministic failpoints cover before intent creation, after intent sync, after
witness construction, after witness sync, before final rename, after rename
before directory sync, and after directory sync before ownership release. Tests
use two independently opened service/store instances contending on one lineage
and prove exactly one commit and deterministic recovery at every boundary.

The real long-lived resident cycle receives a narrow adapter that supplies the
production observations and consumes the committed aggregate. It may continue
post-birthday work only after exact aggregate validation. Restart restores the
same aggregate and governed authorities without invoking activation again.

## Renewed feature-wiring audit

The implementation retains a machine-readable audit row for identity,
continuity, birthday decision, birth witness, Memory Palace, capability
envelope, governed cognitive profile, Adaptive Learning, and ACC/tool
authority. Every row must identify construction, production consumption,
behavioral proof, negative proof, exact source revision, and disposition.
`library_only`, `fixture_only`, `metadata_only`, `documentation_only`, or
`unreachable` is a blocking disposition.

## Scope

- new production activation module and focused tests in `adl-runtime-kernel`;
- the smallest live-assembly and public export changes required by that module;
- a narrow `adl` long-lived-resident adapter and production integration test;
- issue-owned validator and retained evidence;
- canonical v0.92 feature, proof-coverage, birthday, and quality-gate truth.

No Runtime v4 redesign, provider rewrite, restoration of retired demos,
subjective-state claim, or unrelated cleanup is authorized.

## Proof

Focused proof must cover denial before every prerequisite, exact successful
activation, concurrent and sequential duplicate denial, crash/restart recovery,
rollback/copy/replay/conflict refusal, post-restore useful continuation, exact
feature-audit completeness, redaction, and unchanged no-birthday behavior for
ordinary residents. Retained evidence is checked against an exact schema and
redaction/path policy. Formatting runs explicitly for both manifests with
`cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml -- --check` and
`cargo fmt --manifest-path adl/Cargo.toml -- --check`. Strict Clippy runs for
both changed crate surfaces with
`cargo clippy --locked --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`
and
`cargo clippy --locked --manifest-path adl/Cargo.toml --all-targets -- -D warnings`.
Unit-only or fixture-only proof is insufficient.
