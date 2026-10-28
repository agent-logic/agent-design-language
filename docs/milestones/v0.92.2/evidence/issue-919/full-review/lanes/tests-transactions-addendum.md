# Transaction tests review addendum

**P3 TEST-TRANS-001 — Reader concurrency gate has a scheduler-dependent failure.** `csdlc-v3/tests/transactions.rs:2815–2829` loops only while the writer is unfinished, then requires a successful reader observation. If the writer completes its twelve commits before the reader runs, the assertion fails despite correct storage behavior. An explicit reader/writer handshake should make the required observation deterministic. This is source-confirmed schedule analysis, not a freshly reproduced failure.

Candidate: `5c4a6149771c637f3c805985b86231077965eab4`; base: `c9cdb2f13ff77a06dcc79ad4e4c332f444b2e14f`. Independent static review of the complete added semantic Gate A module, lines 1019–2833, plus immediate context and imports. Thirty lexical test entrypoints, including the environment-gated child helper. No tests or builds executed.

The assertions meaningfully check serialization integrity, immutable record bytes, revision/invalidation behavior, native legacy fencing, receipt census and tamper rejection, repository/issue scoping, stale projections, lock collision and cross-process compare-and-swap. Synthetic authenticated receipts establish local consistency checks only. The inventory records file bytes, not complete empty-directory or symlink topology.

Existing ARCH-001 remains relevant: observing unactivated records as RecoveryRequired does not prove that an operator can recover them through the installed owner. ARCH-002 and ARCH-003 require the previously reported bind-copy and terminal reservation/receipt crash windows; pure lifecycle and process-lock tests do not close them. No duplicate product finding is added here.

All added hunks were read. Unchanged older tests at lines 36–1006 were not reread for this assignment. Exact contiguous ranges, observations and finding details are in `tests-transactions-addendum.json`.
