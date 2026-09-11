# [v0.92.2][SIM-06] Execute a fenced and restorable copied-record conversion rehearsal

## Outcome

One repeatable conversion rehearsal transforms a representative copied C-SDLC record census into the new semantic-record/projection layout and proves lossless mapping, old-writer fencing, interruption recovery and safe pre-resume restore. These are inseparable safety properties of the conversion, not separate planning deliverables. Deliver the runnable rehearsal and its retained execution evidence.

## Dependencies and execution boundary

- Depends on SIM-05's merged projections and SIM-04's semantic transaction owner.
- Supplies conversion/restore proof to SIM-07 independent qualification and SIM-08 operations instructions.
- Bind a dedicated FastWork issue worktree and goal after reconciling existing owners. Rehearse only on copies in an isolated repository and an actual non-primary worktree; never convert current operational issue records, stop another session or replace the stable operational binary.

## Current source and bounded implementation

- Ground conversion in `csdlc-v3/src/storage/mod.rs` transaction intents, durable activation and recovery classification; use `application/mod.rs` issue-record/projection validation and `lifecycle/mod.rs` semantic rules.
- Reuse operational identity/authority observation in `commands/local/mod.rs`, remote intents/reconciliation in `commands/remote/mod.rs`, terminal evidence handling in `commands/terminal.rs`, and authority validation in `authority.rs`. Re-resolve these paths after predecessors' decomposition.
- Inventory actual used layouts, including `.csdlc/issues/<issue>/index.json`, cards, resolved Git lifecycle metadata and legacy paths. Map them to the planned `.csdlc/v3/issues/<issue>/state.json` and generated views; never assume an absent path means an empty record.
- Keep source files, immutable receipts, historical projections and prior executable snapshots. Build a hash-bound source-to-destination census and restartable staging/activation journal for the rehearsal. The same conversion mechanism must be usable by the later authorized procedure; a descriptive packet or synthetic mapping without execution is insufficient.

## Acceptance and executed proof

1. Capture source revision, installed-binary provenance, copied issue generations/digests, branches/worktrees, pending local transactions, remote intents, review/publication/terminal references, exact record and evidence counts. Preserve dirty/untracked source bytes; redact secrets without altering operational source material.
2. Execute conversion across prepared, bound/dirty, implemented, reviewed, published, terminal and pending-recovery cases. Compare scope, intent/plan, validation, binding, lifecycle and evidence identities before/after for every record. Report zero silent omissions; unsupported/ambiguous records stop with an issue-specific disposition, never inferred review or approval.
3. Exercise the old installed generation against the isolated writer fence. Prove that old writers cannot write during or after simulated conversion; a new writer that only fences itself is insufficient. Drain or explicitly classify in-flight work in the fixture census. This is a rehearsal, not authority to pause live operators.
4. Convert into staging, validate the whole census, then activate the copied layout and candidate binary using restartable journal steps without deleting original records. Inject failure before/after intent durability, conversion writes, state activation, receipt persistence and projection completion. Restart yields one coherent outcome without lost evidence or duplicate operations.
5. Include an ambiguous remote outcome and remote-success/local-crash case: reconcile the existing operation identity through deterministic fake authenticated transport; do not replay a remote mutation merely because local recording was interrupted.
6. While the isolated writer fence holds, verify per-issue semantic equivalence, exact checkout bindings, evidence digests, selector/cutover provenance, candidate binary provenance, command effects and absence of competing writers. Run observational status/validate from the isolated primary checkout and genuine non-primary worktree. Old schemas fail with an explicit conversion diagnostic.
7. Before any new-format operational write or remote effect, restore the complete copied snapshot and prior executable under the same fence and prove hash/count equivalence. After a simulated new-format write or remote effect, automatic snapshot restore is refused; freeze writers and report the need for separately reviewed reconciliation/forward repair. Never discard new evidence or repeat external work.
8. Retain per-scenario commands, exact revisions, counts, faults, readbacks and pass/fail/not-proven dispositions. Focused validation and required CI pass at independent exact-head review. Fixtures must execute the real converter/storage/application path; caller-set success labels, copied historical claims and zero-test runs cannot establish completion.

## Validation / PVF

Use existing `csdlc-v3/tests/transactions.rs`, `foundation.rs`, `proof_worktree_binding.rs`, `operational_cli_commands.rs`, and `terminal_cleanup_cutover_commands.rs` as source-grounded starting points, adding a focused rehearsal suite if needed. Inventory new tests at authoring: lane = deterministic local CPU integration; proof role = conversion safety and restore boundary; determinism = immutable copied records, controlled interruption points and fake remote transport; resource = bounded local disk/CPU and isolated worktrees, no paid execution; release gate = mandatory SIM-07 pre-resume qualification input, not permission for live conversion. Preserve stdout/stderr separation, explicit effect reporting, redaction and supported-platform denial.

## Stop conditions and non-goals

Stop on missing/ambiguous census, unsupported records, mismatched hashes, missing immutable evidence, old-writer fence failure, unresolved ownership, unclassified remote uncertainty, failed restore, unexecuted required scenario or absent execution authority. Capture durable bug/disposition evidence. No live conversion, operator pause, Runtime/provider/cloud service mutation, paid launch, release ceremony, source deletion or activation. Actual pause, coordinated writer replacement and post-resume pilot require separate authorization and the completed SIM-07/SIM-08 gates; after operational writes, restore is not a shortcut around forward reconciliation.
