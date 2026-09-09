# Issue771 substantive V3-F review — BLOCKED

Reviewer: `codex:/root/review_771_source`
Exact source: `fba2bdf5c7168faa9dcd2d957c121e33a63c0fd6`
Assigned before review; all149 scope blobs still match. No tracked edits or remote writes.

## Findings

1. **P1 — Known #776 remains unresolved in this source.** `csdlc-v3/src/commands/local/mod.rs:2744-2747` resolves schema one directory too high. Current registry1.0.4 card readiness fails. The repaired fixture now synthesizes historical receipt input and the separate worktree integration test executes a shell observer, not native doctor. These tests cannot establish current native lifecycle parity. Consume the separately owned repair and repeat exact-source proof.
2. **P2 — Blocked finish mutates terminal state before rejecting conflicting receipt.** `csdlc-v3/src/commands/terminal.rs:892-909` writes state before validating an existing receipt. Independently reproduced using exact compiled source: initial issue630/PR641 finish succeeds; second verified PR642 closeout with correct expected digest returns `terminal_receipt_conflict` yet state now names642 while receipt still names641. Preflight receipt conflicts before mutation and prove both preimages remain unchanged on rejection. This is a new bounded repair requiring separately approved scope.

## Substantive conclusions

Canonical native authority verifies selector/receipt/readback bytes against origin/main, digest links, terminal identity, merge ancestry and PR subject. Missing or mismatched authority fails closed. Cutover binds exact reviewed revision, selected binary, canary and rollback references, reviewer assignment, mergedPR591 and closed505. Shared advisory lock is explicitly released. Rollback requires a canonical selector revert and refuses unexpected binary digests; replay and fresh-worktree tests retain the Git-common receipt.

Cleanup checks durable receipt identity/digest, canonical candidate containment, exact Git registration, dirt/live state, exact HEAD and preview digest; actual removal uses Git without force and verifies absence. The finish persistence conflict ordering is the exception identified above.

Historical-to-current terminal-test diff was read substantively. Fixture-local registry/policy removes live-registry drift; modeling historical receipt data preserves operational proof guards. It narrows that fixture to verifier behavior, so it must not be labeled current native doctor execution.

| Criterion | Semantic disposition | Current exact-review mapping |
|---|---|---|
|V3-F-ac-1|Pass:179/180 mapped with retained retirement gates|Blocked by unresolved review|
|V3-F-ac-2|Blocked:current native readiness defect776|Blocked|
|V3-F-ac-3|Pass:historical rollback exercise plus current guarded rollback behavior|Blocked by unresolved review|
|V3-F-ac-4|Pass:cutover/retirement approval gates retained|Blocked by unresolved review|

Full suite188passed is parent-reported supporting evidence. Independent additional regression:1failed,0passed,30filtered,exit101. No historical receipt or CORP-A disposition was modified or reopened.

## Scope and limits

JSON receipt enumerates substantively reviewed paths, selective reads, and unreviewed assigned inventory. This is a concrete blocked review of V3-F terminal/authority and coupled proof boundaries, not blanket approval of every line in the149-file whole-crate/template snapshot. No live cutover, Linux/Windows run, or final parent-authored mapping-validator review is claimed.

Reproduction: `/tmp/review771-terminal-conflict --exact review771_conflicting_terminal_receipt_must_preserve_state --nocapture`. Source and output accompany this receipt. The harness copies existing test helpers and adds one regression, links unchanged exact-source compiled library, and uses a temporary fixture.
