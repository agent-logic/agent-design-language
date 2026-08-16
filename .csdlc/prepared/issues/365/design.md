# #365 Sealed Child Eligibility Provenance Design

## Baseline and ownership

Base is exact terminal #274 merge `26de2a048cea436e5140a8ab5afa7524324b3b39`. Terminal #272, #273, and #274 caches must be canonical and their merges ancestral. Product ownership is exactly:

- `adl-runtime/src/distributed/shepherd_serving_eligibility.rs`
- `adl-runtime/src/distributed/observatory_serving_eligibility.rs`
- `adl-runtime/tests/distributed_shepherd_serving_eligibility.rs`
- `adl-runtime/tests/distributed_observatory_serving_eligibility.rs`

No module registration is needed. #275 and #205 remain frozen/read-only.

## Opaque store-derived boundary

Each child module adds one public opaque read-only committed-projection type with private fields and no public or feature-gated caller constructor. Only the owning store can construct it, through `committed_projection()`, from its current durable envelope and the exact stored receipt that produced current state. A caller-created public projection is never an accepted input to this constructor.

The opaque value binds a fixed child-kind/domain, durable envelope generation, committed state revision/log index, durable payload SHA-256, exact stored receipt JCS SHA-256, the child normalized result-state SHA-256, and the already-redacted projection fields. The binding is RFC8785/JCS with `deny_unknown_fields`, bounded identifiers, lowercase SHA-256 digests, I-JSON integers, a fixed domain/version, and SHA-256 over exact canonical bytes. Private construction recomputes every digest from store-owned state/receipt truth and rejects mismatch.

The type exposes only copied scalars, borrowed redacted references, child kind, envelope generation, committed index/revision, state/receipt/provenance digests, and canonical redacted bytes. It exposes no raw state/receipt structs, constructors, mutation, permits, membership, quorum, OwnerCommit, lease, endpoint, secret, authority artifact, or verifier capability.

## Shepherd derivation

`ShepherdEligibilityStore::committed_projection()` returns `None` for empty state. Otherwise it locates the exact receipt whose stored projection equals current state, recomputes the public projection, candidate/final state digest, receipt JCS digest, envelope payload digest, generation, revision, and sealed provenance binding. Missing, ambiguous, stale, or mismatched current/receipt truth fails `Serialization`. Existing acquire/replace/revoke/expire policy is byte-for-byte unchanged.

## Observatory derivation

`ObservatoryEligibilityStore::committed_projection()` returns `None` for empty state. Otherwise it locates the current receipt by the current authority-result key, reruns the existing normalized-final-state digest check, recomputes public projection, receipt JCS digest, envelope payload digest, generation, revision/log index, and sealed provenance binding. Missing, ambiguous, stale, or mismatched truth fails `Serialization`. Existing acquire/renew/transfer/revoke/expiry policy is unchanged.

## Restart, corruption, and substitution

After durable reopen the same child state yields byte-identical sealed projection bytes and provenance digest. Existing `CheckpointedJson::open` authenticates envelope/checkpoint generation and payload SHA; `committed_projection()` additionally revalidates the internal receipt/result binding. Truncated, unknown-field, mutated payload/receipt/state digest, stale generation/index, or mismatched checkpoint fails before any sealed value is returned.

Independent A/B substitution is denied because child kind, envelope generation, payload digest, committed index, receipt digest, result-state digest, and all redacted fields share one canonical provenance preimage. No API accepts separately supplied components. A public-field projection fabricated by a caller cannot be converted into either opaque type; a compile-fail/API-surface contract test proves absence of `new`, `from_projection`, and struct-literal construction.

## Proof matrix

1. authentic committed Shepherd and Observatory stores return opaque values;
2. empty stores return `None`;
3. getters/canonical bytes preserve exact redacted projection truth and durable generation/index provenance;
4. restart/reopen returns byte-identical sealed value;
5. public projection fabrication has no conversion/constructor path; each public opaque type documents a normal-build `compile_fail` struct-literal/private-constructor example that reveals no construction secret, and the exact rustdoc lane must execute it;
6. named in-source unit tests in each owned module exercise the private provenance verifier with A/B component substitution, wrong child kind, corrupt provenance, stale generation/index, and reopen truth; each exact filter must report a nonzero denominator without exposing the seam outside `cfg(test)`;
7. checkpoint/payload/receipt/result/index/generation corruption and rollback fail before projection return;
8. serialized bytes contain no raw subject, permit, membership, quorum, OwnerCommit, lease, endpoint, secret, or artifact;
9. existing #273 and #274 integration lifecycle matrices remain green; separate named source-unit filters, exact rustdoc compile-fail proof, explicit feature-bearing library Clippy, integration-target Clippy, exact four-path diff, fresh exact-head review/CI/finish/cache/ancestry all pass.

## Non-goals

No authority issuance, policy change, new state transition, public constructor, raw projection ingestion, #275 implementation, #205 implementation, module registration, listener/transport/UI/migration/cloud/provider work.
