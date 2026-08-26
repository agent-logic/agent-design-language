# Issue #203 Design — Apply Committed Identity, Lease, and Fencing Authority

## Problem

The committed authority protocol in #201 and the reconciliation barrier in
#200 do not by themselves govern the existing certificate, lease, and fencing
stores. Those stores currently expose direct mutation and authorization
methods. A wrapper that leaves those methods usable would let an in-crate
caller bypass a Pending reconciliation generation. Lease state also persists
replica-local elapsed-time values, so identical committed authority can produce
different canonical bytes on different voters.

## Outcome

Add a sealed concrete-adapter registry that consumes only opaque finalized
#201 tokens through #200 plans, verifies the exact store-native signed artifact,
and reconciles certificate, lease, and fencing effects in a fail-safe order.
Every authority-restoring read or mutation consults the current #200 published
view. Canonical replicated lease state contains only committed deterministic
time evidence; conservative node-local monotonic safety anchors are separate
and can defer an operation without producing a durable canonical result.

This issue does not implement Shepherd or Observatory eligibility, migration or
recovery workflows, process serving, API/WSS routes, model execution, cloud
infrastructure, or the live #142 demonstrations.

## Store-bound authority gate

`authority_store_adapters.rs` owns a private sealed registry and the only
normal-build constructors for authority-bound store handles. Each handle keeps
an opaque reference to the concrete #200 reconciliation barrier. The store,
not its caller, asks that barrier to validate the current published generation,
lineage, adapter kind/version, and action class on every authorization or
mutation boundary.

The existing raw certificate mutations, ledger apply/authorization, and
fencing commit/authorization methods become crate-private low-level operations
that additionally require an unforgeable, single-operation concrete-use grant
issued by the sealed adapter. Merely being in the same crate is insufficient:
ordinary runtime modules cannot construct that grant. Ungated store open and
raw-operation helpers exist only under `cfg(test)` for the path-included
store-specific unit fixtures; they are absent from the normal library build.

Beginning any newer #200 Pending generation invalidates every earlier read or
mutation grant. Retaining a store handle, a result receipt, or an older permit
cannot bypass that live check. Read grants cannot reach mutation methods;
lineage, operation, adapter version, and published generation are exact.

## Token and artifact binding

The private #201 operation payload for each concrete adapter contains the exact
canonical bytes and digest of the artifact the existing store already knows how
to verify:

- certificate operations carry the issuer-signed `AuthorityCertificate` plus
  exact holder, purpose, generation, prior-certificate digest, operation class,
  and committed-time binding;
- lease and fence operations carry the exact encoded quorum-endorsed
  `AuthorityCertificateV1`, its operation class, membership digest, prior lease
  and floor digests, activation-possession digest where required, and committed
  time binding.

The adapter byte-compares the private token view with the sealed plan, decodes
and verifies through the existing store-native verifier, and never signs,
endorses, substitutes, or reconstructs authority. Missing typed artifact views
in the merged #201 API are a stop condition; #203 must not introduce a
caller-supplied replacement.

## Deterministic time and local safety

Canonical lease state and receipts are versioned to bind the quorum-attested
authorization time, absolute inclusive deadline, duration, uncertainty policy,
operation digest, membership index, epoch, and committed log index. They do not
contain replica-local elapsed timestamps.

Each node separately checkpoints a bounded `LeaseSafetyAnchor` containing its
boot generation, local observation generation, monotonic start/deadline, and
the digest of the canonical lease state it protects. The anchor and its digest
exist only in a node-local checkpoint plus node-local audit record. Neither the
anchor bytes nor anchor digest may appear in replicated state, a concrete step
receipt, canonical failure, canonical result, result digest, published store
receipt view, or retry identity. Before an operation that depends on elapsed
safety, a trusted local gate returns one of `Ready`, `NotReady`, or `Unsafe`:

- `Ready` allows the sealed adapter step to begin;
- `NotReady` and `Unsafe` perform no external effect and write no step receipt,
  result, canonical failure, or phase advance;
- restart may conservatively re-anchor only when the exact canonical state,
  boot transition, absolute-time sample, elapsed budget, and uncertainty policy
  prove safety; otherwise authorization remains denied.

Replicated results therefore remain byte-identical while local uncertainty can
only delay authority restoration.

## Sealed operation plans

Every plan is durable as #200 Pending before its first concrete effect. The
ordered steps are fixed by adapter kind and version.

### Certificate operations

- **Enroll/rotate:** validate token and issuer-signed artifact, verify exact
  prior generation, activate through the certificate store, then verify the
  exact active/superseded overlap state before recording the step receipt.
- **Revoke:** Pending first, revoke the exact certificate, verify the exact
  revoked state, and retain denial through publication.
- **Compromise:** Pending first, execute the store-native atomic compromise and
  identity-fence operation, then reconcile the exact token-declared fencing
  floor and ledger revoke/fence effects. No operation may restore authority
  between those steps because every read consults the Pending barrier.

### Lease and fencing operations

- **LeaseGrant/LeaseRenewal:** verify certificate and exact current membership,
  apply the exact quorum certificate to the ledger, persist/checkpoint the
  canonical ledger state, create or refresh the local safety anchor, and verify
  active-lease authorization through the exact floor state. These operations do
  not grant serving eligibility.
- **Revoke/Fence:** Pending denial is visible first, persist the exact fencing
  floor, then apply and verify the exact revoked ledger state. The reverse order
  is not a registered plan.
- **Activate:** require the exact floor, safety-window proof, activation
  possession, and current membership before applying Activate; then revalidate
  the resulting active lease through the fencing store. Pending denial remains
  until the complete result publishes.
- **OwnerCommit:** require exact active lease and floor truth, apply the exact
  OwnerCommit certificate, and revalidate it. It still does not mint Shepherd or
  Observatory authority; #205 consumes only its published receipt.

## Crash and retry boundary

Every concrete step emits an opaque canonical receipt binding adapter version,
lineage, operation, token/artifact digest, exact before/after store digest,
canonical time digest, and membership/log coordinates. It never binds a local
safety-anchor byte or digest. #200 verifies and fsyncs that receipt before the
next step; a separate node-local checkpoint/audit binds the anchor to the
canonical state digest without entering the canonical receipt or result.

An exact retry is cache-first only in the #200 sense: it does not reauthorize a
token or repeat an already-proved effect, but it must reconcile every expected
store digest, receipt, checkpoint, marker, and published view before returning.
Already-applied exact states advance; conflicting, missing, regressed,
noncanonical, or ambiguous states fail closed. There is no transaction across
the three stores; partial progress is safe because the live barrier denies all
authority-restoring reads and mutations until exact publication.

Initialization, dual open, each store effect, each receipt, local-anchor write,
result cache, external checkpoint CAS, final marker, and published-view flip
are independently crash-tested. Files are exclusively locked, canonical,
size-bounded through an opened handle, symlink-safe, and rollback checked.

## Exact normal-build compatibility surface

The raw-access closure is not allowed to rely on an undeclared downstream
ripple. #203 owns the mechanical signature/handle migration of these exact
normal-build consumers: `polis_runtime.rs`, `transport.rs`,
`capability_advertisement.rs`, `placement.rs`, `projection.rs`,
`resource_weather.rs`, `snapshot_catalog.rs`, `migration.rs`, and `recovery.rs`,
all beneath `adl-runtime/src/distributed/`. In particular, the production Polis
bootstrap in `polis_runtime.rs` must stop accepting or retaining a raw
`Arc<DistributedCertificateStore>` and instead receive the authority-bound
certificate handle supplied by the sealed registry.
The migration and recovery edits replace raw certificate/ledger/fencing
references with authority-bound handles only; #204 retains their workflow,
failure-policy, orchestration, and execution semantics.

The exact integration-fixture migration surface is:
`distributed_authority_snapshots.rs`,
`distributed_capability_advertisement.rs`, `distributed_certificates.rs`,
`distributed_discovery.rs`, `distributed_fencing.rs`,
`distributed_guardian.rs`, `distributed_lease.rs`,
`distributed_migration.rs`, `distributed_placement.rs`,
`distributed_projection.rs`, `distributed_recovery.rs`,
`distributed_resource_weather.rs`, `distributed_snapshot_catalog.rs`,
`distributed_transport.rs`, and `distributed_runtime_transport.rs` beneath
`adl-runtime/tests/`, plus the new focused
`distributed_identity_lease_authority.rs`. The runtime-transport fixture must
construct the same authority-bound certificate handle as production and may
not preserve a raw `Arc<DistributedCertificateStore>` bootstrap shortcut.
Store-specific low-level cases that must reach raw primitives move into
`#[cfg(test)]` modules inside
`certificates.rs`, `lease.rs`, or `fencing.rs`; the normal library and every
integration-test crate use only authority-bound handles. A compile-time proof
must compile both `polis_runtime.rs` and `distributed_runtime_transport.rs` and
reject raw constructors, grants, authorization, and mutation in a normal build.

## Published receipt projection boundary

`AuthorityStoreAdapterRegistry` exposes a read-only opaque
`PublishedStoreAuthorityReceiptView` only for an exactly Published OwnerCommit
or Fence result. The view contains lineage, action class, adapter version,
published generation, and canonical result/receipt digests. It exposes no raw
store handle, token/artifact bytes, local safety anchor, or serving decision.
#205 consumes this projection from its separate `serving_authority.rs`; it does
not modify or register concrete effects in `authority_store_adapters.rs`.

#205 adds durable Shepherd and Observatory serving eligibility from published
OwnerCommit/fence truth. #204 runs external migration and recovery workflows.
Actual Guardian/kernel/listener enforcement and the serial Wuji/AWS proof remain
later #142 integration work.

## Exact proof denominator

The denominator is exactly forty-four cases, with exact name/result/marker and
declared subassertion parity:

`certificate_enroll`, `certificate_rotate_overlap`,
`certificate_successor_post_overlap`, `certificate_revoke`,
`certificate_compromise_identity_fence`, `lease_grant`, `lease_renewal`,
`lease_revoke`, `fence_commit`, `activate_after_safety`, `owner_commit`,
`exact_retry_published`, `restart_reanchor_safe`,
`barrier_pending_blocks_all_reads`, `unsigned_certificate_rejected`,
`wrong_issuer_rejected`, `wrong_certificate_purpose_rejected`,
`wrong_certificate_domain_rejected`, `stale_certificate_generation_rejected`,
`token_artifact_digest_mismatch`, `reconstructed_endorsements_rejected`,
`wrong_authority_membership_rejected`, `stale_lease_index_rejected`,
`stale_lease_epoch_rejected`, `wrong_activation_possession_rejected`,
`activate_before_safety_rejected`, `floor_precedes_ledger_revocation`,
`local_clock_unsafe_no_effect`, `local_clock_rollback_no_effect`,
`crash_after_certificate_effect`, `crash_after_fence_floor`,
`crash_after_ledger_effect`, `crash_after_local_anchor`,
`crash_after_result`, `crash_before_checkpoint`, `crash_after_checkpoint`,
`stale_read_permit_rejected`, `stale_mutation_permit_rejected`,
`read_to_mutation_escalation_rejected`, `wrong_lineage_permit_rejected`,
`coherent_rollback_rejected`, `corrupt_noncanonical_oversized_rejected`,
`state_or_lock_symlink_rejected`, and `capacity_n_plus_one_no_partial`.

For every canonical case name `C`, the machine denominator contains exactly
three ordered unique subassertion ids: `C::expected_outcome`,
`C::canonical_store_state`, and `C::publication_barrier_state`. The complete
denominator is therefore exactly 132 ids (44 cases times 3), generated only from
the ordered case list above. Missing, extra, duplicate, reordered, or differently
named ids fail proof production and validation. `canonical_store_state` proves
that canonical receipts/results exclude local anchor bytes and digest;
`publication_barrier_state` proves that any anchor is node-local
checkpoint/audit-only and that partial state remains denied.

The crash cases mechanically enumerate before/after effect and receipt writes,
old/new checkpoint outcomes, marker/view publication, exact restart, dual-open
writer denial, initialization CAS ambiguity, and opened-handle growth or inode
replacement. The capacity case proves no partial store or barrier mutation.

## Non-goals

- Shepherd or Observatory serving eligibility (#205).
- Migration or recovery workflow execution (#204).
- OpenRaft membership (#199) or learner transport/exclusion (#202).
- Guardian/kernel/API/WSS integration, model execution, AWS provisioning, live
  qualification, final #142 publication, merge without operator authorization,
  or lifecycle closeout.
