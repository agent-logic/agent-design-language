# Issue 5342 portable records, signing, and trust design

## Dependency and ownership boundary

WP-07 starts only after the retained #5339 and #5340 terminal receipts validate,
their claims are released, and their merge commits are ancestors of current
`origin/main`. The language and engine crates are read-only dependencies.

This issue owns only `adl-v2/crates/adl-records` and issue-local C-SDLC records.
It does not change Runtime v2, Runtime v3, provider/tool adapters, persistence,
transport, telemetry backends, C-SDLC, CLI selection, or cloud services.

## Record model

The crate defines versioned, deny-unknown-fields contracts for stable errors,
events, trace spans, execution results, and artifact descriptors. Every record
has a stable identity, subject identity, monotonic sequence, logical timestamp,
media type where applicable, and bounded metadata. Artifact contracts contain
content digests and metadata only; the crate does not read or write payloads.

A signed envelope contains one canonical record payload, its record kind and
contract version, the signing profile, key id, canonical payload digest, and an
Ed25519 signature. Canonical payload bytes use this exact grammar:

1. Tags are fixed: `0x00` null, `0x01` false, `0x02` true, `0x03` unsigned
   integer, `0x04` signed integer, `0x05` UTF-8 string, `0x06` byte string,
   `0x07` array, and `0x08` object. Signed integers are encoded as their
   two's-complement `i64` bit pattern in eight big-endian bytes; unsigned
   integers are eight-byte big-endian `u64` values.
2. UTF-8 strings are preserved byte-for-byte after JSON escape decoding; no
   Unicode normalization is performed.
3. Objects use a one-byte object tag, an unsigned big-endian `u32` member
   count, then members sorted by raw UTF-8 key bytes. Each member is a
   length-prefixed key followed by its recursively encoded value.
4. Arrays preserve declared order and use a one-byte array tag, an unsigned
   big-endian `u32` element count, then recursively encoded values.
5. Strings and byte fields are one-byte tagged and unsigned big-endian `u32`
   length-prefixed. Booleans and null have distinct one-byte tags. Signed and
   unsigned integers use distinct tags plus fixed-width big-endian `u64`
   values. Floating-point values are rejected.

The signature preimage is the ASCII domain `ADL-RECORD-SIGNATURE`, one NUL
byte, profile version as big-endian `u16`, then individually `u32`-length-
prefixed UTF-8 record kind, contract version, key id, SHA-256 payload digest,
and the complete canonical payload bytes, in that order. The digest is signed
as its 32 raw bytes, not hexadecimal text. Signature bytes, verification
status, and transport metadata are never inside this preimage. Envelope JSON
is only a channel representation and must decode strictly before this preimage
is reconstructed; unknown or duplicate members are rejected.

Canonical payload bytes begin with `ADL-RECORD-CANONICAL`, NUL, and grammar
version `0x0001` before the root value tag. This independently namespaces
payload identities from envelope signatures and future canonical grammars.

## Signing and trust

Real signing uses the audited `ed25519-dalek` crate. The public API accepts an
explicit signing key or a signing trait; it never generates keys, reads key
files, scans environment variables, or contacts a key service. The verifying
side receives an immutable `TrustPolicy` mapping key ids to Ed25519 public keys,
allowed record kinds, profile version, logical validity interval, and explicit
revocation state. Verification requires an operator-supplied logical time and
fails closed for unknown keys, wrong profiles, wrong kinds, expiry, revocation,
digest mismatch, malformed canonical data, or invalid signatures.

Verification also requires an external mutable `ReplayGuard`; the envelope
cannot provide or configure it. After cryptographic and trust checks, the
verifier atomically admits the tuple `(key_id, subject_id, record_id, sequence,
payload_digest)` exactly once. Reuse of an admitted tuple, sequence rollback
for the same key and subject, or a tuple collision with different bytes fails
closed. A stateless verification API is intentionally not public. Tests use a
bounded in-memory guard, while durable or distributed replay storage remains a
consumer implementation of the narrow guard contract.

Trust-policy bytes are canonical and digestible. The policy cannot be supplied
inside the envelope it authorizes. Key rotation is represented by multiple
independently trusted key entries; there is no implicit fallback or trust-on-
first-use behavior.

`ReplayGuard::admit_atomically` is a normative external contract: durable and
in-memory state must remain unchanged on error, and an admitted token must be
durable before success. The crate exports a conformance helper that proves
duplicate rejection, rollback rejection, and successful progress after those
failures; the bundled bounded implementation passes it.

Generated JSON Schema is the checked structural stage. It proves variant,
field, type, unknown-field, and representation shape. Rust `Record::validate`
and `verify_envelope` are the mandatory semantic stage for dynamic limits,
exact version, nonzero sequence, digest grammar, trust, signature, and replay.
Positive and negative fixtures must agree with this declared two-stage result;
schema validity alone is never a trusted verdict.

## Channel and tamper proof

Records cross process/channel boundaries only as canonical serialized signed
envelopes. Tests pass envelopes through byte channels and fresh-process
verification, then prove deterministic bytes and identical verdicts. The
tamper matrix changes every signed field class, payload bytes, key id, profile,
signature, digest, kind, sequence, and trust decision. Truncation, extension,
unknown fields, duplicate keys, invalid UTF-8, oversized values, and reordered
unordered metadata fail closed or canonicalize to the exact same bytes as
declared.

## Bounds

All constructors and decoders enforce explicit limits for canonical payload,
envelope, incoming channel bytes, JSON nesting depth, total container/member
count, message, metadata entries, metadata key/value length, identity/key-id/
kind/profile/version strings, signature and digest encodings, error detail,
trace attributes, artifact descriptors, schema extension count, trust entries,
and replay entries. The channel decoder checks its byte ceiling before parsing
and uses a depth-limited strict JSON deserializer; typed constructors validate
collection counts before cloning. Serialization uses a capped in-memory
writer. No default path is unbounded, and no validation failure returns
partially trusted data or commits replay state.

## COTS

| Concern | Decision |
| --- | --- |
| Typed serialization | `serde` 1.0.229 and `serde_json` 1.0.151 |
| Public schema | `schemars` 1.2.1 |
| Real signatures | `ed25519-dalek` 2.2.0 with default features disabled and `std` only |
| Digests | `sha2` 0.10.9 |
| Text encoding | `hex` 0.4.3 |

No custom cryptography, canonical-JSON crate, async runtime, HTTP/TLS stack,
cloud SDK, database, telemetry exporter, key store, or workflow engine is
introduced. Canonicalization is small issue-owned traversal over
`serde_json::Value`; cryptographic operations are wholly delegated to COTS.

## Budgets and validation

The reviewed WP-07 allocation is at most 3,000 Rust implementation LoC and
3,000 test/fixture LoC. Focused records/signing tests and strict Clippy each
have a 120-second limit, channel/tamper/fresh-process proof 300 seconds, and the
complete dependency/scope/LoC suite 600 seconds. Cargo output, Cargo home, and
temporary test state live under `/Volumes/FastWork`.

Required lanes prove schema alignment, canonical bytes, real signing,
trust-policy enforcement, every bound, tamper rejection, channel and
fresh-process behavior, deterministic repeated runs, exact dependency pins,
scope exclusion, and no Runtime v2/v3 changes.

## Failure and rollback

Any false dependency signal, cryptographic forgery, noncanonical signed bytes,
ambiguous trust decision, unbounded input, schema drift, forbidden dependency,
scope collision, or budget overrun stops publication. Before consumer cutover,
rollback is removal of the isolated crate and its consumer reference; no
existing runtime or persisted format is mutated by this issue.
