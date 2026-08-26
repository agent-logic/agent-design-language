# #275 Integrated Serving-Authority Proof Design

## Baseline and serial gate

The immutable implementation base is terminal #367 merge `c46b7cd8265a7e81566cdf82153c387595a6cccf`. Terminal caches for #191, #199, #200, #201, #202, #203, #272, #273, #274, #365, and #367 must be canonical and each merge must be ancestral to the base. #205 remains coordination-only.

## Owned implementation surface

The issue owns exactly:

- `adl-runtime/src/distributed/integrated_serving_authority_snapshot.rs`
- `adl-runtime/tests/distributed_integrated_serving_authority.rs`
- one additive `pub mod integrated_serving_authority_snapshot;` line in `adl-runtime/src/distributed/mod.rs`

The acceptance-support surface also owns `.csdlc/prepared/issues/275/validate_bound_contract.py`, `.csdlc/prepared/issues/275/run_exact_unit.py`, `.csdlc/prepared/issues/275/run_exact_focused_matrix.py`, `.csdlc/prepared/issues/275/run_exact_rustdoc.py`, and `.csdlc/prepared/issues/275/validate_exact_scope.py`. The first is retained as preparation history only after implementation begins. The unit wrapper requires the one literal private test through `--list` equality, executes its exact filter, and parses exactly one passed, zero failed, and zero ignored. The focused-matrix wrapper enumerates the exact eight approved test names, compares them to Cargo's selected `--list` output with set equality, rejects missing, extra, or renamed tests, then runs every name through an exact filter and parses each invocation for exactly one passed, zero failed, and zero ignored so exit-zero/zero-run is non-proving. The rustdoc wrapper selects the module-qualified doctests and parses exactly three passed, zero failed, and zero ignored. The scope validator compares the immutable base through current HEAD plus worktree tracked and untracked paths against exactly the three product paths plus issue-owned lifecycle/evidence surfaces and exact `.csdlc/locks/275.lock`, checks committed and uncommitted diff hygiene from the base, and requires `distributed/mod.rs` to contain exactly one additive registration hunk and no other modification. None of these validators substitutes for product behavior proof.

The #272 foundation, #273 Shepherd module/test, #274 Observatory module/test, #365 sealed-provenance implementation/tests, #367 same-lineage pair adapter, authority protocol, adapters, reconciliation, and parent #205 are read-only inputs. Any required edit to those paths stops implementation for a separately reviewed defect.

## Model

`IntegratedServingAuthoritySnapshot` is a deterministic, bounded, redacted observation assembled exclusively from a borrowed `VerifiedCommittedChildLineagePair<'_>` returned by terminal #367 after authentic child-store verification. It is evidence, never an issuer or serving decision. The sole production mutation signature accepts the pair exactly as `&VerifiedCommittedChildLineagePair<'_>` plus an explicit integration outcome (`success`, `no_op`, `rejection`, or `recovery`) and a bounded operation reference. It does not accept the adapter by value. No API accepts separate sealed children, either caller-constructible public projection DTO, raw authority or lineage, permits, membership, quorum, OwnerCommit, lease, endpoints, secrets, pairing booleans, or caller eligibility booleans.

The integration path copies only redacted read-only child getters reached through the borrowed #367 pair adapter and binds each opaque value's canonical bytes and `provenance_sha256`. It cannot create, deserialize, mutate, promote, replace, or separately select either sealed child input. Child-kind mismatch, authentic A/B substitution, corrupted child state/receipt/envelope, and restart drift are denied before adapter construction; the integrated constructor validates every exposed digest/scalar before committing and retains no caller-mintable pairing identity.

The canonical receipt preimage is RFC8785/JCS over a deny-unknown-fields schema with a fixed domain/version; each child's fixed kind, provenance digest, canonical-bytes digest, envelope generation, committed revision/log index, receipt/state digest, redacted status and authority references; the operation reference; outcome; and prior/result integrated state-prefix digests. Identifiers use the existing ASCII identifier grammar and 128-byte bound; digests are lowercase 64-hex; integers are positive and within the I-JSON exact range.

Hashing is ordered, prefix-bound, and nonrecursive. Operations are stored in committed revision order; the operation reference remains a unique lookup key but is never used as the chronology. The exact prefix preimage is the deny-unknown-fields JCS object `{"domain":"ADL-INTEGRATED-SERVING-AUTHORITY-STATE-PREFIX-V1","revision":N,"receipts":[...]}` where `receipts` is the revision-ordered array through N. The canonical empty-prefix sentinel is SHA-256 of JCS `{"domain":"ADL-INTEGRATED-SERVING-AUTHORITY-STATE-PREFIX-V1","revision":0,"receipts":[]}`. Revision 1 binds that sentinel as `prior_state_sha256`; revision N binds the already validated prefix digest for revisions 1..N-1. To create revision N, append the new receipt, normalize only that receipt's `result_state_sha256` and `receipt_sha256` self-fields to the empty string, and hash the exact prefix object to obtain `result_state_sha256`. Next set `result_state_sha256`, leave only `receipt_sha256` empty, JCS-encode the receipt, and SHA-256 it to obtain `receipt_sha256`. Finally populate both fields and commit the full state. Validation starts from the empty sentinel and recomputes every prefix incrementally in revision order, requiring each receipt's prior digest to equal the preceding validated prefix. It rejects missing, duplicate, zero, noncontiguous, or reordered revisions and any operation-map key mismatch. All prior receipts remain byte-exact and no other field is normalized. The durable checkpoint envelope digest is the existing `CheckpointedJson` digest over the fully populated latest payload and is distinct from every prefix and receipt digest. Tests independently recompute prefix, receipt, and checkpoint bindings, tamper each, append multiple operations, and reopen. Receipt bytes and redacted snapshot bytes reproduce byte-for-byte across retry and reopen.

## State and failure semantics

The snapshot store is a bounded `CheckpointedJson` state keyed by operation reference. A new observation is committed atomically only after both opaque inputs pass exact kind/digest/scalar validation. Exact replay returns the prior receipt. Same operation with different input fails `RetryConflict`. Capacity failure, serialization failure, checkpoint CAS failure, corruption, or rejected input leaves the last committed state authoritative and publishes no new receipt.

The integrated snapshot never claims two Shepherds or overlapping Observatory authority. It records one current redacted Shepherd projection and one current redacted Observatory projection. Terminal/revoked/expired projections remain terminal evidence; replacement or transfer is represented only by a later already-committed projection with strictly newer fencing/generation truth. The module does not recreate either child state machine and cannot turn rejection evidence into eligibility.

On reopen, durable envelope validation, the exact normalized final-state digest, and the receipt digest are recomputed in the declared order. Unknown fields, missing fields, malformed/noncanonical values, mismatched receipt/state/checkpoint digest, truncated/corrupt state, or capacity overflow fail closed. Recovery can emit an exact `recovery` receipt only from successfully reopened committed state; it cannot synthesize authority.

## Proof matrix

The focused integration target must prove:

1. deterministic redacted success snapshot from authentic #273 and #274 committed stores through the terminal #367 opaque pair adapter; the new public integration API documents exactly three normal-build `compile_fail` examples selected by the module-qualified rustdoc filter: (a) pair construction or by-value pair use, (b) separate sealed-child arguments, and (c) public DTO or raw-lineage input; the rustdoc lane must report exactly three selected module doctests passed with zero failed or ignored, while unrelated crate doctests filtered out by the module-qualified selector are permitted and do not count toward the selected denominator;
2. exact retry/no-op and conflicting-retry denial;
3. restart/reopen byte equality and recovery receipt binding;
4. Shepherd replacement/revocation/expiry combined with Observatory renew/transfer/revoke/expiry without overlap claims;
5. crash/CAS failure, capacity failure, corruption, truncated/unknown-field checkpoint, and rollback/stale-checkpoint denial preserve the last commit;
6. every raw secret/permit/OwnerCommit/lease/member/endpoint value is absent from snapshot and receipt bytes;
7. success, no-op, rejection, and recovery receipts are distinct and bind operation, inputs, prior state, result state, authority identities, generations/fences, and redacted outcomes;
8. genuine A/B store substitution, separate-child input, wrong child kind, stale generation/index, and corrupted child state/receipt/envelope are denied before integrated commit while authentic paired reopen remains byte-identical;
9. the fail-closed focused-matrix wrapper proves exact set equality for these eight literal names and runs each exact filter: `authentic_pair_snapshot_retry_restart_and_redaction`, `immutable_multi_operation_prefix_and_four_outcomes`, `capacity_and_invalid_operation_fail_closed`, `checkpoint_cas_failure_preserves_last_commit`, `corrupt_truncated_and_unknown_state_fail_closed`, `terminal_child_combinations_remain_evidence_only`, `authentic_ab_substitution_is_denied_before_commit`, and `independent_prefix_receipt_and_checkpoint_tamper_is_denied`; the exact-scope validator proves only the three product paths changed and `distributed/mod.rs` has one additive registration hunk; normal-build rustdoc compile-denial examples, strict feature-bearing Clippy, diff hygiene, fresh exact-head review, hosted CI, typed finish, cache canonicality, and merge ancestry all pass with nonzero denominators.

## Non-goals

No new authority model, verifier, child lifecycle semantics, process/listener enforcement, HTTP/WSS, migration #204, UI, cloud deployment/qualification, provider mutation, or #205 implementation. No raw authority exposure and no edits to #272/#273/#274/#365/#367 owned source.
