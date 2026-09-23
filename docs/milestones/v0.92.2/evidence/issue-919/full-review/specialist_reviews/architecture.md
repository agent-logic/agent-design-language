# Architecture review — completed assigned source scope

Frozen candidate `5c4a6149771c637f3c805985b86231077965eab4`. All 67 assigned changed source paths are accounted for; see exact path ledger. No acceptance PASS is claimed.

## ARCH-001 — P2: Connect retained semantic journal activation to the installed recovery owner

`csdlc-v3/src/application/intent/mod.rs:301`

A process dies after immutable next-generation intent and commit are durable, before current.json activation; current.next may or may not exist. All semantic operations stop on RecoveryRequired. The installed recover path has no native-approved way to activate the exact fully retained commit, despite a storage recovery implementation. Operators must modify storage or build custom recovery tooling.

Evidence: activate() creates intent, commit, current.next then renames. observe_issue translates an unactivated intent/current.next to RecoveryRequired. Router chains effect/projection recovery only; these accept Current/ProjectionRepairRequired. Repository-wide callsite search finds describe_journal_recovery and execute_journal_recovery only in their unit tests. JournalRecoveryApproval constructor is explicitly production dead_code. Checked legacy fallback commands/local/intent.rs193-250: without a legacy transaction it reports expected_noop, leaving semantic state unchanged. Checked native doctor routing and installed-manual source: no alternate semantic journal activation route. Storage creation journal has equivalent unused native approval constructor; kept under same bridge remediation.

Validation limit: Source trace plus inspected exact-activation unit test retained_commit_activation_is_exact_observational_and_replayable; no executed CLI crash fixture in this lane.

Fix: Expose authenticated exact preview/execute journal activation before business-effect recovery, enforce authority/topology/freshness and retain immutable bytes. Add installed-entrypoint crash regression. Inspect equivalent creation journal bridge.

Owner: SIM-04 native intent/recovery owner. Acceptance: SIM-04-A01, SIM-04-A05.

## ARCH-002 — P2: Verify or resume the complete bind staging copy before activating it

`csdlc-v3/src/commands/local/transactions.rs:441`

Interrupt native bind during recursive copy_local_issue_tree(source_stage,target_stage), after target_stage is created but before all files are copied. Resume the supported recover path with source stage and backup intact. commit_bind_local_transaction treats existence of target_stage as completed copying, renames its partial tree to canonical issue root, writes completion and primary binding, then deletes complete source stage and backup. Subsequent integrity checks cannot reconstruct missing compatibility files from native transaction evidence and the issue can become stuck; native completion claims a fully bound image it never verified.

Evidence: Copy helper creates destination before looping files; recovery skips copy whenever destination exists and verifies neither complete inventory nor expected lifecycle digest before target rename/cleanup. The semantic recovery wrapper calls that native operation before rereading integrity, so a later rejection does not preserve deleted source images.

Validation limit: Static exact source trace through partial directory-copy crash window and supported public recovery wrapper. No executed crash harness in this lane.

Fix: Authenticate a complete staged image against retained expected digest/inventory before activation and before source cleanup. Repair or resume only the exact incomplete staging copy under lock; preserve source images until complete target readback succeeds. Add interruption test inside copy, not only after copy returns.

Owner: SIM-04 native bind/recovery owner. Acceptance: SIM-04-A01, SIM-04-A05.

## ARCH-003 — P2: Recover a reserved finish when no terminal receipt was written

`csdlc-v3/src/application/intent/terminal.rs:1378`

Process dies after a Finish or FinishWithoutPr semantic reservation has committed but before the terminal receipt is written; also covers state written but receipt not yet written. GitHub closeout remains authenticated and unchanged. Exact finish replay requires the absent receipt in AlreadyPending and fails intent_terminal_pending_readback_required. Public recover has no Finish owner, so the healthy semantic pointer retains an operation that cannot complete through supported commands, blocking terminal completion and clean.

Evidence: Finish stages read-only authenticated observation, reserves effect, then invokes native persistence. AlreadyPending only reads and attaches an existing receipt. recover dispatch has no terminal finish branch; its only terminal route reconciles absent cleanup. Native persist_terminal_finish already compares retained state and immutable receipt identity and writes state then receipt, but replay never reaches it in this window.

Validation limit: Static public-entrypoint callgraph and exact durable crash-window analysis; no executed CLI crash reproduction. Distinct from ARCH-001: current semantic pointer is healthy and has a pending business effect, not an unactivated journal.

Fix: Add an exact preview/reconciliation path for pending Finish/FinishWithoutPr that authenticates the original terminal decision and safely completes missing native state/receipt through its idempotent owner, then attaches the result. Test interruption both after reservation and between state/receipt writes.

Owner: SIM-04 terminal finish/recovery owner. Acceptance: SIM-04-A01, SIM-04-A05.

## Independence and scope

Same specialist performed code and architecture lenses, separately recorded; this is not two independent reviewers. Candidate source is read-only. No operational lifecycle, paid provider, cloud, or GitHub writes. Embedded tests have only selective source inspection unless the ledger says otherwise. Integration test-source and fixture-family classification retain separate test-lane coverage. All 12 changed CodeFriend command paths are complete in the code ledger. Exact moved baseline blocks may be provenance-accounted rather than rereviewed; architecture-moved-diffs records those boundaries.
