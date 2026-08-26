# #273 Shepherd Serving-Eligibility Authority — Design

## Authority and dependency boundary

#273 consumes terminal #272 at merge `df9a61644b8620f225a4eaccecd1d8eda5b5eb15`. The public redacted `ServingAuthorityProjection` is caller-constructible and is not eligibility authority. #273 adds one non-policy foundation API in `serving_authority.rs`: after the existing sealed `PublishedStoreAuthorityReceiptView` and private `ServingAuthorityBinding` verification and durable Published commit, `reconcile_and_verify_cut` returns an opaque `VerifiedServingAuthorityCut`. Its fields are private, it has no public constructor, and read-only accessors expose the exact generation, OwnerCommit id, fencing generation, lease id, candidate-state digest, result digest, and receipt digest. It validates an already-committed authority cut but makes no Shepherd or Observatory eligibility decision. The parent #205 remains coordination-only.

Every declared predecessor (#191, #199, #200, #201, #202, #203, #272) must have a canonical merged terminal cache whose merge is ancestral to the implementation base. A missing, stale, noncanonical, or nonancestral cache blocks bind and execution.

## Exact file ownership

Product ownership is limited to a new `adl-runtime/src/distributed/shepherd_serving_eligibility.rs`, its `adl-runtime/tests/distributed_shepherd_serving_eligibility.rs` test, one additive non-policy verified-cut API in `adl-runtime/src/distributed/serving_authority.rs`, and one additive registration line in `adl-runtime/src/distributed/mod.rs`.

#274 owns a different future `observatory_serving_eligibility.rs` module and test. Implementation is serialized: #273 merges and becomes terminal/ancestral first; #274 then rebases against that head, consumes `VerifiedServingAuthorityCut`, and must not edit `serving_authority.rs`. The only eventual same-file integration touch is additive registration in `distributed/mod.rs`.

## State model

The module owns a bounded, deterministic Shepherd eligibility state machine:

- `Vacant`: no Shepherd is eligible.
- `Eligible`: exactly one opaque Shepherd subject is bound to one exact published #272 foundation generation, OwnerCommit digest, fencing generation, lease identity/digest, permit digest, and expiry.
- `Revoked`: the prior grant is explicitly invalidated and cannot be replayed.
- `Expired`: the grant crossed its declared logical expiry and cannot regain eligibility.

Inputs contain opaque subject/permit identifiers, an unconstructible `VerifiedServingAuthorityCut` returned by the foundation operation, caller-supplied monotonic logical time, and an operation id. Caller-created projections, naked digests, wall-clock reads, environment state, cached booleans, process state, and caller claims are not authority.

## Operations

`acquire` succeeds only from `Vacant`, for a current exact #272 published foundation binding, a nonexpired permit, and a fencing generation strictly newer than the retained floor. `replace` atomically revokes the old grant and installs exactly one newer fenced grant; no intermediate result exposes two eligible owners. `revoke` records the exact target grant and advances the retained fence floor. `expire` deterministically transitions a matching eligible grant when supplied logical time reaches its bound.

Same operation plus byte-identical input is idempotent and returns the prior receipt. Same operation with different input fails closed. Stale fence, stale/revoked permit, mismatched OwnerCommit/lease/foundation digest, wrong subject, expired lease, rollback, or generation reuse fails without mutation.

## Receipts and projection

Every successful transition returns a deterministic receipt binding operation, previous-state digest, candidate-state digest, exact #272 foundation state/result digests, opaque subject reference, permit digest, fence, lease digest, expiry, transition kind, and resulting state digest. Public projection is redacted: schema, coarse state, opaque keyed subject reference, foundation generation and state/result digests, fence, expiry class, and receipt/state digests only. It exposes no raw token, permit, lease, OwnerCommit, path, endpoint, or process data.

## Persistence, retry, and capacity

The issue may use an issue-local bounded journal/store abstraction inside its new module, following the #272 reconcile-before-publish contract. At most one eligible grant is visible. Pending or unreconciled bytes are never eligible. Restart/retry returns the exact committed receipt or fails closed. Bounded capacity failure preserves the last committed state and makes no partial mutation.

## Validation

The focused integration test must prove no verified cut exists before sealed verification and Published commit; fabricated projection/naked-digest input is impossible; acquire, idempotent retry, conflicting retry, atomic replacement, revoke, expiry boundary, stale fence/permit/OwnerCommit/lease/foundation rejection, no dual eligibility, restart boundaries, corruption/rollback/capacity failure, receipt binding, and redaction. Exact test and library Clippy run with warnings denied. Changed-path proof requires exactly the new module, test, bounded foundation API file, and registration file while rejecting #274, parent #205, process/listener/transport/UI/cloud, and unrelated lifecycle paths.

## Non-goals

No Observatory quorum lifecycle (#274), combined proof (#275), parent #205 implementation, #272 durable policy/state-machine change beyond the explicitly authorized additive non-policy verified-cut API, #203 registry change, process launch, listener/transport/HTTP/WSS wiring, migration #204, UI, cloud/provider action, paid runner, or release qualification.
