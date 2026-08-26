# Issue #300 design: deterministic integrated projection-recovery proof

## Authority, gate, and scope

#300 is the test-only 297.c child. It qualifies exact production revisions from
#298 and #299. Bootstrap, repair, and design approval may occur now. Bind,
implementation, test execution, review, and publication require validated typed
terminal records for both prerequisites and Git ancestry proving both reviewed
heads/merges are in the selected #300 execution base. Missing, stale,
non-ancestral, or ambiguous authority fails before bind.

The issue owns only `csdlc-v2/tests/projection_recovery_integration.rs` plus
issue-local `.csdlc/issues/300` and `.csdlc/evidence/300`. It may not edit
`projection_recovery.rs`, the #299 cleanup module, `store.rs`, `schema.rs`,
owner binaries, `gate5.rs`, or #291/#294/#296/#297/#298/#299 records. A missing
production hook or semantic defect is routed separately and stops #300.

## Harness architecture and authority contract

The table-driven harness creates real registered projections and invokes the
production classify, recover, cleanup, and ordinary-commit routes. Authority
inputs are production terminal records, exact PR/head/merge ancestry, registered
repo/branch/worktree identity, generation/digest tagged CAS, production
classification output, archive/canonical manifests, request digests, and
production-generated immutable recovery/cleanup receipt heads. Tests may retain
these values but never synthesize, edit, or rehash a receipt chain into apparent
authority.

For each row `(operation_id, boundary_id, side)`, where side is `before` or
`after`, the harness constructs the same verified fixture, enables one declared
production failpoint, invokes the typed owner, drops every in-memory object, and
restarts with the same request. It then disables the failpoint and drives the
operation to a terminal typed result.

The allowed state machine is:

`FixtureReady -> IntentDurable -> MutationObserved -> CompletionDurable -> Verified`

An interruption retains the last durable state or one declared
post-mutation/pre-receipt state. Restart adopts only identities bound by durable
production receipts. Every other observation becomes `RejectedPreserved`.
Repeating a verified operation returns the same result; a conflicting operation
id rejects without mutation. The harness fails if a boundary is missing from
the production registry, fires twice, occurs out of order, or lacks its durable
predecessor.

Outputs asserted are the exact typed disposition, canonical generation/digest,
archive/cleanup state, receipt-chain heads, retained rejected-evidence identity,
sentinel/replacement identity, and later ordinary-commit result. Negative
fixtures change filesystem nodes or request fields before production invocation;
they do not manufacture valid authority artifacts.

## Failpoint and adversarial matrix

For recovery and cleanup, interrupt immediately before and after every declared:
intent write, intent fsync, exclusive candidate/namespace/placeholder creation,
content write, node fsync, parent fsync, receipt write, archive/capture exchange,
rename, canonical install, verification, capture receipt, removal intent,
unlink, empty-directory rmdir, placeholder capture/disposal, completion receipt,
and finalization boundary. From every reachable durable state, prove restart,
same-operation repeat, and conflicting-operation rejection.

Across canonical, backup, rollback, recovery candidate, archive, cleanup
namespace, every ancestor, manifest node, placeholder, and receipt path as
applicable, cover: missing/unexpected names; symlink and dangling symlink;
regular-file/directory/type swaps and special nodes; same-byte replacement;
repeated inode and regular-file hardlink; ancestor swap and destination race;
cross-device/mount; uid/gid/mode drift; size/digest corruption; namespace
mismatch; stale generation/digest/tagged CAS; wrong registered topology;
truncated/hash-broken receipt; non-empty directory; conflicting operation; and
ambiguous candidates. Every rejection must preserve canonical/archive/rejected
evidence and immutable ledgers, never delete an unrelated sentinel, and never
delete a replacement inode.

The integrated success path runs classify/recover/cleanup repeatedly and then
performs a later ordinary typed commit. It proves idempotency and that only the
intended recovery gate is released while historical evidence remains.

## Evidence and validation contract

Issue-local evidence records exact prerequisite terminal/head/merge/ancestry,
integrated Git revision, production failpoint-registry digest, each matrix row
and observed state sequence, command argv, exit status, and final artifact
digests. Deterministic platform skips record capability facts separately and do
not count as local PASS. VPP/SOR name only commands actually executed and
results observed.

The focused command is `cargo test --manifest-path csdlc-v2/Cargo.toml --test
projection_recovery_integration`. Existing #298/#299 focused tests, current
#291 regression coverage, and strict all-target Clippy are separate regression
lanes. Hosted checks run only after reviewed publication. A canonical #119
fresh-session exact-head review with no actionable findings is required.

## Stop conditions and non-goals

Stop before bind unless #299 is terminal and both prerequisites are ancestral.
Stop on production API insufficiency, ownership collision, nondeterministic
ordering, fabricated authority, an unprovable matrix row, or review finding.
No production redesign, shared-test mutation, lifecycle absorption, paid runner
without authority, publication, merge, or closeout belongs to preparation.
