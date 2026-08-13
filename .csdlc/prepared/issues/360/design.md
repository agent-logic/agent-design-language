# #360 Authentic Observatory Transition Fixture

## Boundary

#360 repairs test authority only. Production schemas, verifier behavior, sealed
projection construction, and #274 product code remain unchanged. Owned source is
limited to `authority_protocol.rs`, `serving_authority.rs`, and the existing
`distributed_observatory_authority_projection.rs` integration test, with every
new builder/helper gated by `internal-test-fixtures` or `cfg(test)`.

## Design

Add one test-only operation descriptor carrying distinct operation ID,
committed index, fencing generation, authenticated action, and optional
predecessor. The authority-protocol fixture must use that descriptor when
constructing the real `PublishedAuthorityResult`; the serving-authority fixture
must encode the identical values into canonical artifact bytes and the real
`VerifiedServingAuthorityCut`. Production verification remains the sole path
to `VerifiedObservatoryAuthorityProjection` and rejects any cross-binding
mismatch.

The focused proof constructs four distinct operations: Acquire has no
predecessor; Renew references Acquire; Transfer references Renew; Revoke
references Transfer. It verifies all four through
`verify_observatory_authority_projection`, plus stale index/fence cross-binding,
wrong or self predecessor, and independent A/B cross-pair denial. It also
supplies authentic overlap, superseded, and revoked/expired revival projection
shapes to #274; semantic history denial remains #274's state-machine proof
because the single-projection verifier has no transition history. The helper exposes no raw
quorum basis, membership, OwnerCommit authority, or production constructor.

## Scope

- `adl-runtime/src/distributed/authority_protocol.rs`
- `adl-runtime/src/distributed/serving_authority.rs`
- `adl-runtime/tests/distributed_observatory_authority_projection.rs`
- issue lifecycle/evidence only

No #274 production module, `distributed/mod.rs`, #273, #272, #203, #205, or
#275 change.
