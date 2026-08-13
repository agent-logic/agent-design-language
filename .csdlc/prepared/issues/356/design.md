# #356 Minimal sealed Observatory projection accessors

## Decision

Terminal #350 correctly makes `VerifiedObservatoryAuthorityProjection`
constructible only through `verify_observatory_authority_projection`. #356 adds
only read-only accessors for its already-redacted fields so sibling #274 can
implement deterministic state transitions without accepting caller authority.

Owned product paths are:

- `adl-runtime/src/distributed/serving_authority.rs`
- `adl-runtime/tests/distributed_observatory_authority_projection.rs`

The accessors return borrowed strings or copied scalar values for trust-domain,
polis, lineage, and operation opaque references; committed log index; foundation
generation; fencing generation; authority-result digest; signer-set digest and
count; inclusive deadline seconds; and finalization seconds. No constructor or
mutation is added. Raw identifiers, Guardian IDs, membership/configuration,
thresholds, OwnerCommit, lease ID, foundation state/receipt digests, artifact
bytes, tokens, signatures, keys, endpoints, paths, and provider data remain
unavailable.

## Proof

The existing focused projection target gains a test that builds a valid A/A
pair through the sealed verifier and asserts each accessor against the verified
fixture. It also proves A/B substitution fails before projection creation and
that debug/serialized projection text contains none of the excluded raw
authority values. Existing #350 mismatch, durable restore, canonical encoding,
and redaction tests remain the denominator. Strict Clippy and diff hygiene run
before a fresh exact-head implementation review.

## Boundaries

No #274 state machine, #273 behavior, #272 foundation, #203 registry, #205,
#275, UI, listener, transport, cloud, provider, or deployment work. The exact
implementation base is terminal #350 merge
`5bff0099858f005bcc045b0aa7548be4892a2acb`.
