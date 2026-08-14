# #367 Same-Lineage Sealed Child Pairing Design

## Baseline and ownership

Base is terminal #365 merge `a4801fbb3a58bed27ba53367cbda8b31a1f56083`. Terminal #272, #273, #274, and #365 caches must be canonical and ancestral. Product ownership is exactly:

- `adl-runtime/src/distributed/serving_authority.rs`
- `adl-runtime/src/distributed/shepherd_serving_eligibility.rs`
- `adl-runtime/tests/distributed_shepherd_serving_eligibility.rs`
- `adl-runtime/tests/distributed_observatory_serving_eligibility.rs`

#275 and #205 stay frozen. No module registration or Observatory source edit is required.

## Non-caller-mintable pairing identity

`VerifiedServingAuthorityCut` already retains verified raw `lineage_id` and exposes it only from that opaque verifier-returned cut. Add `lineage_ref() -> String`, which derives `keyed_ref("lineage", lineage_id)` inside `serving_authority.rs` using the exact existing `ADL-SERVING-REF-V1\0lineage\0{value}` domain/preimage. #367 adds no new raw-lineage getter and exposes only this redacted reference on the child sealed/integration surfaces; no caller-provided lineage, constructor, mutation, or authority capability is added.

Shepherd `Grant` records `lineage_ref` only from `cut.lineage_ref()` during authenticated acquire/replace. Revoke/expire preserve it. Stored/public projections, exact receipts/state digests, and `SealedShepherdCommittedProjection` bind and expose the redacted reference. The terminal Observatory sealed projection already exposes the identically derived `lineage_ref` from its verified binding.

`verify_committed_child_lineage_pair(&SealedShepherdCommittedProjection, &SealedObservatoryCommittedProjection)` is a read-only verifier in the Shepherd module. It accepts only the two opaque #365 types, checks fixed child kinds, requires both lineage references, and returns a privately constructed `VerifiedCommittedChildLineagePair<'a>` only for byte-equal redacted lineage references. The pair adapter borrows the exact sealed children and exposes only read-only child accessors required by #275; it has no public constructor, deserializer, raw-lineage getter, pairing boolean, or mutable state. #275 must accept this verifier-returned adapter rather than independent child DTOs on its first and every later integrated commit.

## Durable compatibility and failure semantics

Legacy Shepherd grants deserialize with `lineage_ref: None`; this is compatibility only. They cannot produce a sealed committed pairing projection: `committed_projection()` fails `Serialization` rather than inventing lineage. A later authenticated acquire/replace may provide new verified lineage truth through the normal existing transition policy. No migration or synthetic upgrade occurs.

Lineage is included in Shepherd grant input, receipt stored projection, normalized state, public projection, sealed preimage, canonical bytes, and provenance digest. Missing, malformed, raw, mismatched, mutated, or stale lineage fails closed. Existing acquire/replace/revoke/expire decisions remain unchanged except that authenticated outputs preserve additional redacted evidence.

## Proof matrix

1. two authentic committed stores built from the same verified lineage yield equal redacted refs and the opaque pair verifier returns an adapter borrowing exactly those children;
2. two authentic A/B stores with distinct verified lineages are rejected before first integrated use and again after both reopen;
3. caller-created public DTOs, raw lineage, fabricated pairing values, and struct-literal pair adapters cannot enter the opaque verifier or #275 seam; normal-build compile-fail/API inspection remains green;
4. Shepherd acquire/replace derive lineage from the cut and revoke/expire preserve it without policy changes;
5. legacy missing-lineage durable state fails sealed projection/pairing and is never synthesized;
6. mutation, unknown-field, corrupt checkpoint, stale generation/index, and A/B component substitution fail closed;
7. restart returns byte-identical sealed bytes and provenance; raw lineage, permit, OwnerCommit, lease, membership, endpoints, secrets, and artifacts are absent;
8. existing Shepherd and Observatory lifecycle matrices, strict feature-bearing library and exact-target Clippy, exact four-path diff, fresh exact-head review, hosted CI, typed finish, cache canonicality, and ancestry pass.

## Non-goals

No new authority model, new raw lineage getter or raw lineage exposure on child/integration surfaces, caller constructor/deserializer, policy transition, #275 implementation, #205 implementation, listener/transport/UI/migration/cloud/provider work.
