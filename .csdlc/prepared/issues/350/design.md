# #350 Sealed Authenticated Observatory Authority Projection

## Decision

#350 repairs the predecessor interface exposed by fresh #274 design review. It
adds one sealed projection that can be obtained only from a replicated,
published `PublishedAuthorityResult` and the exact verified #272
`VerifiedServingAuthorityCut`. The projection is authority-bearing input for
#274; callers cannot construct or substitute its fields.

## Owned source

- `adl-runtime/src/distributed/authority_protocol.rs`
- `adl-runtime/src/distributed/authority_protocol_contract_tests.rs` for the
  existing replicated-publication compatibility denominator only
- `adl-runtime/src/distributed/serving_authority.rs`
- `adl-runtime/tests/distributed_observatory_authority_projection.rs`
- one additive test-target declaration in `adl-runtime/Cargo.toml` only if the
  repository requires explicit integration-test registration

#350 does not implement the #274 state machine, change #273 Shepherd behavior,
or touch #203, #205, #275, UI, listener, transport, cloud, or provider code.

## Authenticated contract

Durable authority publication must retain the committed intent's inclusive
deadline alongside the already-retained operation, finalization time, signer
set, artifact, result, retry, committed log index, and publishing identity.
The sealed adapter validates replicated-apply provenance, quorum signer truth,
deadline ordering, exact artifact/operation identity, and exact cross-binding
to the #272 cut before returning an unconstructible projection.

The projection binds trust domain and polis identity, lineage and operation,
committed log/foundation generation, OwnerCommit, fence, lease, foundation
state/result/receipt digests, signer-set digest/count, committed inclusive
deadline, and canonical finalization time. Any mismatch fails before output.

## Canonical cross-binding

The published authority artifact bytes are exactly the RFC 8785/JCS encoding
of one JSON object with `deny_unknown_fields` and these exact keys/types:

```json
{
  "domain": "adl.observatory-serving-authority-binding.v1",
  "trust_domain": "UTF-8 identifier",
  "polis_id": "UTF-8 identifier",
  "lineage_id": "UTF-8 identifier",
  "operation_id": "UTF-8 identifier",
  "committed_log_index": 1,
  "foundation_generation": 1,
  "owner_commit_id": "UTF-8 identifier",
  "fencing_generation": 1,
  "lease_id": "UTF-8 identifier",
  "foundation_state_sha256": "exactly 64 lowercase hexadecimal characters",
  "foundation_result_sha256": "exactly 64 lowercase hexadecimal characters",
  "foundation_receipt_sha256": "exactly 64 lowercase hexadecimal characters"
}
```

All three integer values are positive JSON integers no greater than
`9007199254740991` (the I-JSON exact-integer bound); there is no byte-order
interpretation. Every identifier is 1..128 ASCII bytes and every byte must be
ASCII alphanumeric or one of `_`, `-`, `.`, `:`—exactly the
`authority_protocol::validate_identifier` grammar. No Unicode normalization,
case folding, trimming, escaping equivalence, or alternate serving-authority
validator applies. Unicode, whitespace/control bytes, empty/oversized values,
and every other punctuation character are rejected before JCS serialization.
Digest strings reject uppercase, prefixes, base64,
arrays, whitespace variants after parsing, wrong length, and non-hex bytes.
`domain` is an ordinary mandatory JSON member, not an external prefix. The
accepted artifact bytes must already equal the bytes produced by JCS
re-serialization of the parsed object; semantically equivalent noncanonical
JSON is rejected. The artifact SHA-256 is already retained by the operation.

The sealed adapter reconstructs the same value from the authenticated
published identity/operation and the opaque #272 cut, serializes it with JCS,
and requires byte equality with the verified operation artifact plus equality
of its SHA-256 with the retained artifact hash. It also requires the published
operation ID and committed log index to equal the canonical payload values and
the #272 cut's generation, OwnerCommit, fence, lease, state/result/receipt
fields to equal their canonical payload values. Trust domain, polis, lineage,
and operation may never be accepted as free caller inputs. This one-to-one
domain-separated equality relation prevents two independently valid published
authority and serving-cut objects from being copied together.

## Sealed quorum-basis snapshot

At replicated verification time, the durable published result stores a sealed
`QuorumBasisSnapshot` containing: the 32-byte authenticated configuration
digest, voter-set generation, committed membership log index, and private old
plus optional joint-new configuration entries. Each configuration has a
positive threshold and 1..4096 entries canonically sorted by Guardian ID. Each
entry is exactly `(guardian_id, certificate_generation, boot_generation)`;
Guardian IDs are bounded nonempty bytes, generations are positive, and
duplicate IDs are rejected. The domain-separated digest of the canonical
sorted entries plus threshold is also retained for each configuration. This is
the minimal eligibility basis used by original endorsement validation: it is
sealed durable state, never part of the returned projection, logs, or errors.
No raw key, signature, endpoint, node ID, or public peer list is exposed.

The snapshot digest is included in the durable result/retry digest preimage.
On restore, validation revalidates entry bounds/order/uniqueness/generations,
recomputes both configuration digests, and maps every retained signer Guardian
ID to an exact eligibility entry. A signer counts for a configuration only
when its retained certificate and boot generations equal that entry. The
signers must meet the old threshold and also the new threshold for joint
consensus. Restore also revalidates the aggregate configuration digest, voter
generation, committed index, result and retry digests. Missing snapshot in a
legacy record; missing/extra/duplicate or generation-mutated entries; mutated
signer IDs; threshold/generation/index drift; configuration mismatch; or half
of a joint quorum fails closed and yields no published authority. There is no
default, count-only, or digest-only inferred quorum authority.

## Durability and compatibility

The inclusive deadline is persisted in `DurablePublishedAuthorityResult` and
validated during restore. Existing checkpoints lacking the new field must be
handled explicitly and fail closed or migrate deterministically; silent default
authority is forbidden. Existing non-Observatory consumers retain behavior.

## Recovery provenance boundary

The pre-implementation quarantine created while repairing design review is
lifecycle recovery evidence only, retained under Git-common
`.git/csdlc-v2/quarantine-payloads/350/` with SHA-256 manifests under
`.git/csdlc-v2/quarantine-manifests/`. Manifest lines are standard
`<lowercase-sha256><two spaces><absolute-local-path>` entries and are verified
with `shasum -a 256 -c`. They preserve rejected packet bytes and provenance for
operator audit; they are not tracked product artifacts, runtime checkpoint
schema, validation input, restore fallback, retry authority, or publication
content. Runtime code must not read these paths. A rejected legacy/corrupt
runtime record remains rejected regardless of any quarantine payload. Local
absolute paths stay Git-common and are never emitted by projection/log/error.

## Proof

Focused tests cover exact projection; uppercase/base64/array/wrong-length
digests; unknown/missing fields; noncanonical key order/whitespace/numeric
forms; Unicode, whitespace/control, case, and disallowed punctuation in every
identifier field; every cross-binding mismatch; the 2x2 matrix of authority A/B against
cut A/B (only A/A and B/B pass); insufficient/empty and joint quorum truth;
signer/snapshot/config/threshold mutation after checkpoint restore;
deadline/finalization mismatch; retry; legacy/corrupt durable state; and
redaction. The existing authority-protocol contract denominator must also remain green
after its fixtures use the replicated sealed-publication path; legacy direct
verification remains non-authoritative for durable #350 publication and is
denied rather than supplied a synthetic quorum basis. Strict Clippy,
scope proof, diff hygiene, fresh exact-head review, hosted CI, and typed finish
remain required.
