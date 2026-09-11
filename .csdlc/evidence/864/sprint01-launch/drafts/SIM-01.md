# [v0.92.2][C-SDLC v3][SIM-01] Make operational diagnostics strictly read-only

## One complete result

An operator can inspect a healthy, missing, corrupt or interrupted issue through the installed diagnostic entrypoints without creating, repairing, recovering or changing lifecycle state, projections, transaction journals or Git worktree registrations. An interrupted transaction is reported as recovery-required with an actionable explicit operation; observation never performs the recovery.

Dependency: own typed readiness; no WP-01 or unrelated closeout dependency. Coordinates under SIM-UMBRELLA and hands the same installed journey corpus to SIM-02.

## Current evidence and bounded owned paths

At the launch baseline, `execute_operational_local_route` in `csdlc-v3/src/commands/local/mod.rs` acquires an issue mutation lock and calls `recover_pending_local_transaction` before dispatching `doctor`, `eligibility` or `validate`. Reconfirm the observed effect before correction. `csdlc-v3/src/main.rs` selects operational/discovery/construction reporting; its fallback must not conceal an operational observation failure.

Own the observation dispatch/effect separation in those two files and focused coverage in `csdlc-v3/tests/operational_cli_commands.rs`, `csdlc-v3/tests/local_commands.rs`, and a narrowly scoped installed-diagnostic journey fixture if needed. Inspect `csdlc-v3/src/commands/release.rs`, `csdlc-v3/src/commands/sprint.rs`, and `csdlc-v3/src/commands/remote/mod.rs` only for the declared read-only surfaces; modify only a demonstrated diagnostic side effect. Share fixture helpers under `csdlc-v3/tests/` with SIM-02/03 rather than making a second harness product.

## Acceptance

1. Exercise installed `doctor`, `eligibility`, `validate`, read-only schedule/shepherd recommendations, `pr-state`, `release-preflight`, and existing local/foundation/sprint observation helpers against every applicable healthy/missing/corrupt/pending-state case. Mark helper coverage separately from issue diagnostics; no implied mutation rights arise from helpers.
2. Compare before/after byte inventories of issue records, cards/values, journals, receipts, staging/backup paths and Git registrations, including lock-file creation/removal. No lifecycle, registration or projection mutation occurs; diagnostic results report zero such effects. Use both a primary checkout and a genuine registered non-primary worktree.
3. Pending local/remote transaction and corrupt/missing evidence observations produce honest recovery-required, blocked or failed states. They neither replay a remote effect nor materialize missing cards/state. Preserve existing authority, topology, exact-head review, corruption and cleanup denials.
4. Run mutation-path regressions to prove actual mutators retain their explicit locking/recovery semantics; separating diagnostics must not silently disable required recovery on a mutating path.
5. Establish the shared attempt-level baseline corpus: healthy prepare/bind/edit/proof/review/publish/finish/eligible-cleanup, expected guard denials, interrupted work and root/non-primary discovery. Record existing failures as failures rather than claiming all baseline journeys pass. Include source/binary/environment, manual input fields and invocation counts, eligible/attempted/completed/abandoned/censored denominators, stable outcome/reason, effects, before/after issue version, correlation ID, invalidation cause, wall/monotonic durations and wait owner. Inject clocks in deterministic fixtures; timings alone do not prove causal savings.
6. Machine JSON remains stdout; human events remain stderr. Validate redaction and documented compatibility-log behavior; writing an explicitly requested external measurement artifact is distinguished from mutating inspected lifecycle storage.

## Validation and PVF

Required local deterministic lane: installed candidate in isolated Git repositories with deterministic fake remote transport, byte/registration snapshots and failure injection; local CPU/Rust/Git, no paid service. Proof role: semantic observation guarantee and mutation regression; required issue and SIM-07 gate. Run focused `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands` and `--test local_commands`, plus any new installed-diagnostic fixture and `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check`. Classify new tests in their tightly coupled proof manifest with lane, role, determinism, resources and release-gate status; CI integration evidence is recorded separately. Warm only trusted same-host dependencies under the repository policy. A zero-test run or model-only unit test cannot prove installed entrypoint purity.

## Exclusions and stop conditions

No intent-CLI redesign, unified semantic record migration, broad source decomposition, full operator manual, active writer conversion or baseline statistical reanalysis. Stop for unresolved owner collision, missing execution authority, an unsupported recovery that would require invented state, or missing proof. Route a distinct defect explicitly instead of widening this task.

## Authority and execution boundary

This is one implementation task in the first v0.92.2 C-SDLC simplification sprint. Use native C-SDLC v3, the current selector/receipt guards, an issue-bound FastWork worktree and an issue-bound session goal. Re-resolve current source and active owners before edits. Root main stays inspection-only. Resolve overlap with CSDLC-MAN/#861, CSDLC-DECOMPOSE/#862, CSDLC-REMOTE and other SIM workers before shared-path edits. Preserve all historical evidence bytes.

This issue authorizes implementation and isolated candidate proof, not coordinated live writer activation, state conversion, replacement of the active operator binary, real GitHub effects, Runtime/provider shutdown, paid cloud/provider execution or a second live writer. The breaking replacement activates only through the separately authorized transition/pilot after SIM-06/07/08. Installation for proof means an isolated fixture-local candidate built from the exact reviewed source. Never work around an authority or stale-review failure.

## Source contract

- `docs/milestones/v0.92.2/cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md`
- `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`
- `docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.md`
- `csdlc-v3/AGENTS.md` and root `AGENTS.md`

Launch inspection baseline: `ace209ad9c701a855d165da817164ff60a755203`. This is source inspection, not execution proof or a promise that defects survive to the implementation baseline. Retain source/candidate revisions, installed binary digest/provenance and clean fixture identity with every run.
