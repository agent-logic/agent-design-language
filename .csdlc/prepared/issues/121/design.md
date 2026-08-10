# Issue 121 Design: Quorum fencing and restart-safe lease epochs

## Outcome and boundary

Repair only the WP-04.07 lease authority contract required for majority-authorized fencing and restart-safe higher-epoch activation. The product scope is exactly `adl-runtime/src/distributed/lease.rs` and `adl-runtime/tests/distributed_lease.rs`. WP-04.08 fencing implementation, module registration, Cargo metadata, and sibling distributed paths remain outside this issue.

## Exact source baseline

The implementation base is PR #120 exact reviewed head `91c47ed3ab5ec060cf2ba790d107b1598aa6ba6f`. The repair remains stacked on `codex/5909-lease-authority-defect` until that parent merges. Issue #121 and its implementation PR belong only to `agent-logic/agent-design-language`.

## Authority transition contract

Activation-key possession is operation-sensitive. `LeaseGrant`, `LeaseRenewal`, `Activate`, and `OwnerCommit` continue to prove possession of the declared activation key. A majority-endorsed `Fence` or `Revoke` is authority-ledger action and does not require cooperation from the holder being removed.

`Fence` binds the existing lineage, holder, prior activation-key digest, signed lease deadline, policy, voter generation, and exactly the next epoch at a newer committed log index. A stale, same-epoch, skipped-epoch, wrong-holder, wrong-incarnation, wrong-deadline, minority, or replayed fence is rejected before state mutation. `Revoke` remains same-epoch terminal denial unless a distinct next-epoch fence is used.

## Durable restart contract

Lease state retains a portable recovery safety floor derived from the prior signed wall-clock deadline plus maximum clock uncertainty and message-delay margin. A committed fence advances the applied prefix, marks the prior owner non-authoritative, and preserves that floor in the canonical bounded snapshot. Restore verifies the snapshot at the current membership committed index, keeps the lineage fenced, and never trusts a prior process's elapsed-time value for replacement activation.

The floor is removed only after a valid majority-endorsed activation at exactly the fenced epoch crosses the portable deadline and proves possession of its new activation key. Unrelated owner-commit, renewal, fence, or revoke operations do not erase it. Mutation authorization remains denied while the lease is revoked or a recovery floor is unresolved.

## Failure semantics

Every invalid certificate, unavailable holder key, stale or missing quorum, index/epoch mismatch, clock uncertainty, overflow, capacity failure, snapshot corruption, replay, or incomplete recovery fails closed without partial mutation. A numerically high local epoch without the committed majority prefix is never authority.

## Proof

The exact nonzero `distributed_lease` target proves unavailable-holder quorum fence and revoke, next-epoch transition rules, fenced mutation denial, snapshot/restore at the current committed index, retained portable recovery floors, delayed safe activation, strict possession for holder-authorized operations, and atomic negative behavior. Machine-derived negative markers must have exact denominator/name/result parity with the retained proof receipt. Strict focused Clippy and fresh independent exact-head review are required before publication.

## Rollback

Do not weaken existing lease safety to recover availability. If the committed prefix, durable fence, or portable floor cannot be established, keep the lineage fenced. The issue does not merge itself and does not unblock #5870 until this repair is merged and ancestral.

## Non-goals

- No `fencing.rs` implementation or #5870 path.
- No module registration, `lib.rs`, `mod.rs`, Cargo, manifest, or lockfile changes.
- No migration, placement, catalog, recovery, projection, or final-integration child work.
- No custom cryptography, transport changes, or merge authorization.
