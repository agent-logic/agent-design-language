# #272 Durable Serving-Authority Foundation

## Decision and boundary

#272 is slice 205.a. It creates only the durable, reconcile-before-publish
foundation consumed later by #273 (Shepherd), #274 (Observatory), and #275
(integrated proof). The coordination parent #205 owns no product code.

The implementation owns a new serving-authority module and its focused
foundation tests. It consumes the terminal #203
`PublishedStoreAuthorityReceiptView` as an opaque, read-only receipt. It must
not edit `authority_store_adapters.rs`, widen
`AuthorityStoreAdapterRegistry`, or manufacture #203 authority. It may use the
sealed #200 reconciliation interfaces only through their current crate-private
contracts.

The current #203 view intentionally exposes no OwnerCommit, fence, or lease
accessors. #272 therefore verifies those exact fields through one byte-exact
authenticated preimage rather than widening #203. The preimage is at most 4096
bytes and is exactly:

`b"ADL-SERVING-AUTHORITY-FOUNDATION-BINDING-V1\0" || u32_be(jcs.len()) || jcs`

Here `jcs` is RFC 8785/`serde_jcs` canonical JSON for a
`#[serde(deny_unknown_fields)]` object with exactly these fields and no
optionals: `schema`, `trust_domain`, `polis_id`, `lineage_id`, `operation_id`,
`adapter_kind`, `adapter_version`, `action_class`, `published_generation`,
`owner_commit_id`, `fencing_generation`, `lease_id`, `prior_state_sha256`,
`candidate_state_sha256`, and `receipt_digest`. `schema` is exactly
`adl.serving-authority-foundation.binding.v1`; identifiers are nonempty UTF-8
strings of at most 128 bytes; generations are positive canonical JSON integers;
both state digests and `receipt_digest` are lowercase 64-character SHA-256 hex.

#272 recomputes SHA-256 over the complete framed bytes and requires exact
equality with the opaque view's `result_sha256`. Lineage, operation, adapter
kind, action class, adapter version, published generation, and receipt digest
must independently match the view's direct accessors. The preimage is
untrusted until every direct and digest check passes and is never retained or
projected. This uses the existing sealed digest as authority; it does not
invent a public DTO, decode opaque #203 state, or require a #203 interface
change. If terminal producers do not emit this exact canonical preimage, #272
fails closed.

## Frozen ownership

The #272 implementation allowlist is:

- `adl-runtime/src/distributed/serving_authority.rs`
- `adl-runtime/src/distributed/mod.rs`, solely to register that module
- `adl-runtime/tests/distributed_serving_authority_foundation.rs`
- `.csdlc/issues/272`
- `.csdlc/prepared/issues/272`
- `.csdlc/evidence/272`

No other product or test path is owned. In particular, #272 must not edit:

- `adl-runtime/src/distributed/authority_store_adapters.rs` (#203)
- certificate, lease, fencing, membership, authority protocol, or authority
  reconciliation implementation
- Runtime kernel ingress (#265)
- C-SDLC projection recovery/cleanup (#300/#330)
- #114 conversation-history surfaces

The single-file module is intentionally foundation-only. #273 and #274 may not
implement concurrently against it unless their own reviewed designs establish
disjoint module files and registration ownership; otherwise they serialize.

## Inputs and authority

The prerequisite chain #191, #199, #200, #201, #202, and #203 is terminal,
canonically reconciled, and ancestral to the exact preparation base. #272
accepts no caller-built authority. A candidate transition is bound to:

- trust domain, polis, lineage, and operation identity;
- opaque committed operation/result identity from #201/#200;
- #203 published receipt lineage, operation, action class, adapter version,
  generation, receipt digest, and result digest;
- OwnerCommit identity, fencing generation, lease identity, prior-state digest,
  and candidate-state digest encoded in the exact framed JCS preimage above,
  whose SHA-256 must equal the sealed #203 view's `result_sha256`;
- exact prior durable generation and state digest.

Configuration, node-local bytes, retained permits, cached booleans, raw tokens,
and caller DTOs are never authority.

## State machine

`ServingAuthorityStore` is a bounded node-local replica of committed authority,
not an independent issuer. Each operation is exactly one of:

1. `Pending`: durable intent and exact prior-state digest exist; no derived
   projection is publishable.
2. `Reconciled`: the current sealed authority cut and #203 published receipt
   have been revalidated and the candidate durable state is exact.
3. `Published`: a canonical result and redacted projection for the same
   operation/generation are durable and visible.

Opening and every retry reconcile journal, canonical state, result cache, and
published projection before returning. A different operation, prior-state
digest, authority binding, generation, or receipt digest fails closed. Any
incomplete, corrupt, oversized, noncanonical, rolled-back, symlinked, or
identity-mismatched state denies publication.

The store uses the repository's existing checkpointed durable JSON authority
instead of inventing file persistence. Paths and identities are derived from a
validated trust-domain/polis/node/guardian/boot binding. Capacity is explicit;
N is accepted and N+1 fails before mutation.

## Redacted base projection

The base projection contains only:

- schema/version;
- keyed opaque polis and lineage references;
- published generation and state/result digests;
- coarse `empty`, `pending`, or `published` readiness;
- the exact #203 receipt generation and redacted digest bindings.

It contains no raw token, permit, certificate, lease, fence, OwnerCommit,
identity, endpoint, network address, filesystem path, key, signature, exact
deadline, Shepherd identity, or Observatory identity. It does not decide
eligibility; #273/#274 own those state transitions.

## Public API posture

Normal-build construction is crate-private and requires the sealed authority
inputs. Public output is redacted and read-only. The module exposes no process,
listener, HTTP/WSS, UI, projection-v1, migration, or cloud behavior. Test
fixtures remain test-only and cannot mint production authority.

## Proof plan

The focused integration target
`adl-runtime/tests/distributed_serving_authority_foundation.rs` proves:

1. exact sealed binding and rejection of wrong lineage/operation/adapter
   kind/action, adapter version, generation, receipt/result digest, canonical
   framing/schema/field set/limits, OwnerCommit, fence, lease, prior-state and
   candidate-state digests, including single-byte preimage substitution;
2. Pending-before-publish and reconcile-before-publish ordering;
3. cache-first exact retry and conflicting retry rejection;
4. restart from each durable boundary without premature publication;
5. corruption, rollback, noncanonical data, capacity N/N+1, and unsafe path
   rejection without partial publication;
6. deterministic redacted projection and secret-field absence;
7. compile-time/product-scope guards showing #203 and unrelated owned files are
   unchanged.

The issue-local preparation validator proves only packet identity, exact typed
card/design bindings, dependency-cache identities, declared ownership, lane
budgets, and deferred product target. It is not product proof. Product tests,
strict Clippy, `git diff --check`, exact-head review, and hosted CI occur only
after bind.

## Serial gates

1. Bootstrap all six #272 cards from this design and diagram.
2. Obtain a new `fresh-session:<UUID>` design review on the exact initialized
   generation/digest and artifact hashes.
3. Record typed design approval, run validate/doctor, then bind only after PASS.
4. Implement only the frozen allowlist and run the focused proof plus strict
   Clippy and diff hygiene.
5. Finalize to implemented, assign a new fresh exact-head reviewer, resolve all
   findings, record review, publish, shepherd required CI, and finish.
6. #273/#274 remain blocked until #272 is terminal and ancestral.

## Non-goals

No Shepherd acquire/replace/revoke/expiry (#273), Observatory
acquire/renew/transfer/revoke/expiry (#274), integrated failure matrix (#275),
process or listener enforcement, HTTP/WSS, Runtime kernel ingress, migration
#204, projection v1, UI, cloud deployment, provider action, paid runner, or
parent #205 implementation/closeout.
