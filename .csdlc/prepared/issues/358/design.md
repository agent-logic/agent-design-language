# #358 Sealed Observatory transition intent and canonical time

## Decision

Extend terminal #350/#356's authenticated Observatory artifact and sealed
projection, without adding a parallel authority path. The canonical artifact
adds `transition_action` (`acquire`, `renew`, `transfer`, `revoke`) and an
optional `predecessor_operation_ref`. Acquire requires no predecessor; renew,
transfer, and revoke require one nonempty redacted predecessor reference that
differs from the successor operation reference. Unknown actions, missing or
unexpected predecessors, self-predecessors, and noncanonical bytes fail before
projection creation.

The verifier keeps private construction and returns the action plus predecessor
reference only inside `VerifiedObservatoryAuthorityProjection`. Read-only
accessors expose those values and full committed inclusive-deadline and
finalization components: seconds, nanos, and uncertainty milliseconds. Exact
expiry ordering is `(unix_seconds, nanos)`; equality is not expired and a
strictly greater tuple is expired. Uncertainty is retained as authenticated
receipt/projection truth and does not silently alter ordering.

Owned product paths:

- `adl-runtime/src/distributed/authority_protocol.rs`
- `adl-runtime/src/distributed/serving_authority.rs`
- `adl-runtime/tests/distributed_observatory_authority_projection.rs`

Focused proof covers every action/predecessor shape, A/B substitution, action
mutation, second-equal nanos-before/equal/after vectors, durable restart,
unknown/noncanonical fields, and redaction. No constructor, caller DTO, raw
quorum/membership, OwnerCommit, lease, artifact, secret, #274 state machine,
#273, #272, #203, #205, or #275 behavior is added.
