# [v0.92.2][SIM-04] Route operational mutations through one semantic issue transaction owner

## Outcome

Every operational local transition and authenticated remote or terminal outcome uses the same versioned semantic issue record and transaction owner. An operator can complete and recover a real installed-command journey without independent local and terminal interpretations of lifecycle truth. Deliver the complete production routing change, its tests and recovery documentation together.

## Dependencies and execution boundary

- Depends on SIM-03's merged installed intent-command contract; predecessor SIM-01 and SIM-02 guarantees remain required.
- Supplies the transaction owner consumed by SIM-05; do not absorb projection derivation or conversion rehearsal.
- Reconcile current ownership with CSDLC-DECOMPOSE/#862, CSDLC-REMOTE and CSDLC-MERGE/#849 before editing shared command paths. Re-resolve current source paths after their merges.
- Execute only in this issue's native-bound FastWork worktree after readiness and an issue-bound goal. Candidate behavior runs in isolated repositories. This issue does not activate an incompatible writer against live state or convert active records.

## Current source and bounded implementation

- Extend `csdlc-v3/src/storage/mod.rs` (`StateRecord`, `DurableTransactionStore`, transaction/recovery classification) and `csdlc-v3/src/lifecycle/mod.rs` (`decide`, transitions, invalidations); reuse their existing semantics rather than introducing another state machine.
- Wire `csdlc-v3/src/application/mod.rs` and `csdlc-v3/src/main.rs` to that owner; enumerate and replace independent mutation paths in `csdlc-v3/src/commands/local/mod.rs`, `commands/proof.rs`, `commands/remote/mod.rs`, `commands/remote/merge.rs`, and `commands/terminal.rs`.
- The target `.csdlc/v3/issues/<issue>/state.json` contains repository/issue identity, typed intent and plan, validation requirements, binding, lifecycle phase, generation, semantic digest, and immutable review/publication/terminal evidence references. This is the planned target layout, not a claim that existing records already use it.
- Keep remote intent durability, idempotency and authenticated observation in the effect protocol. Never hold a local filesystem transaction across a network mutation. Feed verified outcomes into the semantic transaction and retain operation identity across uncertainty.
- Keep CLI intent parsing, application orchestration, pure lifecycle decisions and effects/storage distinct. Retire alternative writers only through the separately qualified conversion/activation boundary; no intermediate deployment introduces two live writers.

## Acceptance and executed proof

1. Inventory every supported mutating route from SIM-03 and show actual production entrypoints committing through one issue generation/audit history. No command independently writes lifecycle phase or terminal completion. An unused library, model-only test or partial route inventory cannot close this issue.
2. Through the installed candidate in an isolated repository, execute prepare → bind → edit → proof → review → publish → finish → clean, using deterministic fake remote transport. Evidence identifies exact candidate, initial/final versions, operation identities and observed effects. Live GitHub mutation needs separate scoped authorization.
3. Inject failures before/after intent durability, external success, state activation, receipt persistence and projection handoff. Restart recovers the same result without duplicate remote mutation, invented completion, or hidden repair by status/validate. An ambiguous remote result remains unresolved until authenticated reconciliation establishes its outcome.
4. Concurrent requests on the same generation cannot both commit. Authority, repository, branch/worktree or exact revision changes before mutation fail closed. Reject stale review, altered receipts, wrong reviewer, wrong PR base/head/linkage and missing terminal evidence.
5. Recommended next transitions remain admissible when relevant facts have not changed. Supported scope/plan/proof/binding/review amendments and interruptions have finite legal recovery; retain causal invalidation evidence without weakening exact-head review.
6. Keep observation read-only, stdout machine-readable and human events on stderr. Test redaction and documented compatibility-log behavior. Preserve canonical selector/authenticated cutover provenance and explicit denial on unsupported platforms.
7. Focused tests and required CI pass at an independently reviewed exact revision; document actual outcomes and unexecuted scenarios separately. No zero-test result or caller-authored success marker establishes routing/recovery proof.

## Validation / PVF

Extend relevant production-path suites in `csdlc-v3/tests/transactions.rs`, `operational_cli_commands.rs`, `local_commands.rs`, `remote_publication_commands.rs`, and `terminal_cleanup_cutover_commands.rs`, selecting only touched suites. Use the focused C-SDLC owner lane when applicable. Each new test is declared in the issue's validation inventory: lane = deterministic local CPU contract/integration; proof role = single-owner routing, crash/concurrency and negative authority guard; determinism = fixed fixtures and fake remote transport; resource = bounded local CPU/disk, no paid provider/cloud; release gate = required pre-resume C-SDLC qualification input, not live activation authority. Candidate binaries are built/installed into isolated proof destinations; do not replace the stable operational writer during validation.

## Stop conditions and non-goals

Stop on unresolved ownership, absent authority, ambiguous checkout, an unenumerated writer, unreconciled remote outcome, guard regression, failed/unexecuted required proof, or any attempt to activate without explicit pause/fence authority. Capture a durable bug packet for tooling anomalies. No Runtime/provider/cloud changes, paid execution, new lifecycle generation, weakened review/cleanup guards, speculative features, live conversion, or activation. SIM-06/07/08/09 own rehearsal, qualification, operations packet and separately authorized pilot.
