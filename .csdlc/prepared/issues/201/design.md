# Issue #201 Design — Establish the Committed Authority Protocol

## Problem

WP-04.16a provides authenticated three-voter OpenRaft transport and durable
consensus storage. The Runtime still needs a deterministic protocol that turns
an exact committed intent plus endorsements from current voters into an opaque
quorum-approved operation token. A leader, harness, caller, or replica-local
clock must not self-authorize merely because Raft replicated some bytes.

## Outcome

Add one deterministic committed-command protocol that consumes the exact
current `MembershipState` and `AuthorityMembership`, verifies endorsements from
an opaque local voter authority, durably publishes an exact result and retry
record through an external checkpoint, and emits an opaque operation token whose
private operation-specific view retains the exact bounded store-native signed
artifact bytes, their digest, and operation binding for sealed downstream
membership, reconciliation, and existing-store integrations. For an exact
continuity-transfer operation only, the token also exposes a separate sealed
`ContinuityTransferGrantProjection` to #210. That projection is a borrowed,
read-only, nonconstructible view of the already-finalized variant; it does not
broaden the generic artifact view or create a new authorization path.

This issue does not apply OpenRaft membership changes or mutate certificate,
lease, fencing, migration, or recovery stores. Governed membership is #199;
concrete-store reconciliation is #200. Kernel continuity, Observatory
presentation, models, cloud provisioning, and live qualification remain later
#142 children.

## Command protocol

Every state-changing operation uses two committed entries:

1. `PrepareAuthorityIntent` commits a canonical, domain-separated intent with
   polis id, trust domain, current membership epoch/index and voter-set digest,
   operation kind, expected prior protocol checkpoint, payload digest,
   canonical prepare-time token, inclusive finalization deadline, a unique
   bounded operation id, and one bounded operation-specific artifact envelope.
   The envelope contains the exact store-native signed bytes and their digest;
   its domain and variant must match the operation kind. A digest without the
   retained bytes is not a downstream authority input.
2. Each current voter may endorse only that exact committed intent through an
   opaque `VoterEndorsementAuthority` bound to node, guardian, voter purpose,
   certificate generation, boot generation, and membership index. Each
   endorsement signs the intent digest plus the proposed canonical
   finalization-time token. Raw signing keys and caller-produced endorsements
   are not accepted.
3. `FinalizeAuthorityIntent` carries the intent digest, the exact signed
   finalization-time token, and endorsements. Replicated apply requires
   `prepare_time <= finalization_time <= inclusive_deadline` under the exact
   committed time policy. This proves that quorum authorization occurred at a
   declared time inside the intent window, and a replay cannot regress or
   replace that signed time.
4. A durable protocol journal records the finalized token, its byte-identical
   artifact envelope, and canonical result. The result and private artifact view
   become readable only after the exact external protocol checkpoint CAS and
   retry record are durable.

Exact retries return the retained canonical result. Conflicting reuse,
superseded membership, wrong-domain evidence, missing or duplicate voters,
invalid keys, a declared finalization time outside the inclusive intent window,
artifact byte/digest/operation mismatch, rollback, and reordered finalization
fail before protocol publication.

## Private exact artifact view

`VerifiedAuthorityOperation` has private fields and retains a sealed
operation-specific `VerifiedAuthorityArtifact` envelope. The envelope stores the
exact canonical bytes committed by `PrepareAuthorityIntent`, their SHA-256
digest, the operation class, and the intent digest. It is journaled and replayed
byte-for-byte; retry never asks a caller to resupply it.

Only sealed consumers for #199, #200, and #203 receive the existing borrowed
read-only exact-artifact view.
That view exposes the exact bytes and binding needed by the destination's native
verifier but offers no constructor, replacement setter, raw endorsement, signing
operation, or generic caller-selected payload conversion. #201 validates the
bounded envelope and its committed binding; it does not interpret or apply the
certificate, lease, fencing, or membership effect. #203 remains responsible for
decoding and verifying the retained bytes through the existing store-native
signature or quorum path before any concrete effect.

The distinct sealed #210 projection is available only when the operation class
is continuity transfer. It binds the exact source and target voter/Guardian,
route, membership, certificate and boot cuts; transfer id and lineage; the exact
#208 `SourceCheckpointHandle` identity and its byte-identical bundle-handle
identity; the retained signed bundle manifest and catalog bytes plus their
digests and trusted key generation; canonical entry order and each entry's
schema, range, length, and digest; canonical chunk index, absolute range,
expected digest, and predecessor; total bounds, deadline, uncertainty policy,
and a cleanup identity. The projection does not derive a fresh checkpoint or
bundle handle from those bytes: #210 must present the exact borrowed identities
to #208's sealed `ContinuityBundleSourcePort`, and a lineage or handle mismatch
fails before a read or transfer session can begin.
The projection borrows those values from the finalized journal record and has no
public constructor, replacement setter, generic artifact conversion, or method
that signs, sends bytes, reads or writes a path, discards a stage, decides
migration, fences a source, activates a target, or grants serving. Wrong
operation variants and every consumer other than the sealed #210 adapter are
rejected before a view is returned.

Serialization, snapshot restore, retry-cache load, and checkpoint reconciliation
all compare both retained bytes and digest. Snapshot-replicated current authority
contains only the stable polis id, membership epoch, trust domain, voter-set
generation, committed membership index, exact configuration set, and voter
records. It never contains the restart-scoped current boot-generation map.

The runtime owns the current trusted boot cut outside snapshot-replaceable state.
Every new `PrepareAuthorityIntent` must carry the exact full canonical boot vector
for that runtime cut. Missing or extra voters, stale generations, zero
generations, duplicate guardians, reordered entries, and non-canonical wire bytes
fail before any state mutation. Prepare does not rewrite the replicated current
authority. Instead, each prepared operation freezes its complete canonical boot
vector beside the stable authority and binds both with a custody digest.

Finalization and publication use only that frozen historical custody. Restore and
snapshot install reverify the prepared digest, exact historical voter coverage,
complete finalize proposal, endorsements, quorum, signatures, certificates,
boot generations, time window, committed prepare index, and finalize index.
Therefore a runtime may reopen with a newly trusted current boot cut and
immediately build or install a snapshot before any new Prepare, while older
prepared and finalized operations remain byte-identical and verifiable under the
boot cut they actually committed. Retired owner, Shepherd, Observatory, fence,
and demotion fields must remain empty. Injecting a current boot map into the
snapshot, or omitting, re-encoding, substituting, or corrupting historical
custody or evidence, fails closed.

The prepare and finalize entries contain canonical quorum-attested time tokens.
Replica-local clocks may determine whether a voter is willing to endorse a
proposed finalization time, but replicated apply consumes identical committed
time bytes on every voter and cannot branch on a local wall or monotonic clock.
The protocol defines expiry at quorum authorization time. It does not claim
that replicated apply can discover a leader withholding already-valid signed
endorsements after that point. Enforcing a later submission-freshness window
would require a separate independently advancing committed freshness authority
and is outside #201.

When `AuthorityMembership` contains joint configurations, the intent binds an
exact canonical digest of the ordered configuration set. Finalization requires
strictly more than half of the distinct valid voters in every nonempty current
configuration. A guardian counts at most once within each configuration even if
duplicated in input, and appearing in both configurations does not convert a
majority of the union into a majority of each. Old-only, new-only, and
union-majority endorsement sets fail closed.

## Authority boundary

- Voter identity, guardian, control key, voter purpose, and Raft id derive from
  concrete `MembershipState` plus exact `AuthorityMembership` parity. Caller
  route or configuration data is never voter authority.
- The protocol produces a private-field `VerifiedAuthorityOperation` token.
  Only sealed #199, #200, and #203 consumers may inspect its existing read-only
  exact-artifact view. Only the sealed #210 continuity-transfer adapter may
  inspect the separate exact-variant projection described above. Public
  constructors, replacement bytes, generic conversions, raw endorsements, and
  caller-selected quorum sets are absent.
- Membership coordination, certificate/lease/fence application, and external
  migration/recovery workflows do not run inside deterministic Raft apply here.
- Replay identity binds polis, peer generation, boot generation, committed
  index, operation id, request digest, and canonical response.
- Legacy `PolisCommand` variants that directly mint membership, fence, owner,
  Shepherd, Observatory, migration, or recovery authority are retired or
  explicitly rejected. Retained logs and snapshots are versioned and either
  migrate without minting authority or fail closed.

## Crash and publication boundary

Each voter owns a symlink-safe, exclusively locked, size-bounded canonical
journal. Its external checkpoint object id is a canonical digest of trust
domain, polis id, node id, guardian id, boot generation, and protocol-instance
version. Checkpoint candidates bind the committed log id, protocol generation,
intent digest, result digest, and retry-cache digest. The three voters therefore
advance three independent node-local monotonic authorities; no shared CAS lets
one replica publish for another. Initialization and each finalized result record
expected old and new checkpoints before publication. On restart each voter
compares its journal, result cache, and exact node-local checkpoint authority:

- old everywhere: retry the exact step;
- new everywhere: advance the journal;
- a proved partial protocol commit: finish only the same exact operation;
- conflicting, regressed, ambiguous, or missing checkpoint authority: fail
  closed and require recovery/operator action.

The crash protocol explicitly reconciles initialization collision, local durable
write before CAS, CAS success before the local final marker, and restart at each
of those boundaries independently on nodes A, B, and C. Checkpoint reuse across
node, boot, polis, trust domain, or protocol instance rejects.

No downstream consumer may receive a token whose protocol publication barrier
is incomplete. Deterministic Raft apply records a valid finalize as pending and
returns no token; runtime-owned local reconciliation exposes the published
result only after its retry record, journal, and external checkpoint are durable
and agree. This is per-object protocol atomicity, not a transaction over
downstream authority stores.

## Downstream split

- #199 consumes membership-operation tokens and implements
  `AuthorizedOld -> LearnerCaughtUp -> JointCommitted -> FinalCommitted ->
  AuthorityParityPublished`, including removal fencing and crash reconciliation.
- #200 consumes concrete-operation tokens and publishes reconciliation plans
  without claiming cross-store atomicity.
- #203 consumes the private exact artifact view through #200's published plan and
  applies existing certificate, lease, and fencing store effects. #201 never
  performs or reconstructs those effects.
- #210 consumes only the separate sealed continuity-transfer projection and
  combines it with #208's opaque source, stage, verifier, and cleanup ports. The
  projection binds the exact lineage, `SourceCheckpointHandle`, and
  byte-identical bundle-handle identity consumed by #208; wrong lineage or
  either wrong handle is rejected before source access. #201 neither transports
  bytes nor performs a kernel continuity effect.
- #193 and later children consume those merged authorities for real kernel
  continuity and operational serving. They do not broaden this protocol.

## Proof

- Real three-voter OpenRaft tests prepare and finalize commands through the core
  protocol and never construct endorsements or verified tokens in the harness.
- Positive cases cover strict current quorum, canonical committed time,
  durable token publication, exact retry, downstream opacity, and byte-identical
  private store-native artifact retention.
- Fault cases cover missing/duplicate/wrong-key endorsements, signer
  unavailability and rotation, membership mismatch, local clock skew at the
  endorsement boundary, crash at every journal/checkpoint boundary, rollback,
  replay conflict, artifact substitution, corruption, capacity, symlink paths,
  and every retired legacy authority command.
- Machine evidence binds exact source, commands, a named nonzero denominator,
  strict Clippy, marker parity, protected-source drift, immutable evidence
  introduction, and eventual squash-merge topology.
- The complete current `adl-runtime` library lane is recorded truthfully as
  230/230 with zero skipped; this supersedes the earlier 222-test planning
  expectation without changing the issue-specific 86-case semantic contract.

The replacement denominator is exactly eighty-six cases. Every case emits one
canonical `ADL_ISSUE_201_CASE_V2` marker; the independent immutable manifest
binds the order, name, expected result, and complete marker-line digest. The
retained forty-seven names are:
`current_three_voter_finalize`, `exact_retry_returns_cached_result`,
`signer_rotation_current_generation`, `joint_majority_each_config`,
`finalize_at_deadline`, `three_node_checkpoint_restart_reconcile`,
`missing_quorum`, `duplicate_signer`, `wrong_voter`, `signer_unavailable`,
`expired_signer_cert`, `stale_membership`, `config_digest_mismatch`,
`joint_old_only`, `joint_new_only`, `joint_union_majority_only`,
`joint_duplicate_guardian_reuse`, `declared_finalize_time_after_deadline`,
`finalize_before_prepare_time`, `replay_with_regressed_finalize_time`,
`local_clock_skew_apply_parity`, `checkpoint_object_collision`,
`node_a_local_before_cas`, `node_a_cas_before_final_marker`,
`node_b_local_before_cas`, `node_b_cas_before_final_marker`,
`node_c_local_before_cas`, `node_c_cas_before_final_marker`,
`checkpoint_result_retry_digest_mismatch`, `coherent_rollback_rejected`,
`corrupt_journal_rejected`, `corrupt_retry_cache_rejected`,
`capacity_n_plus_one_no_partial`, `state_symlink_rejected`,
`lock_symlink_rejected`, `legacy_fence_voter_rejected`,
`legacy_activate_owner_rejected`, `legacy_activate_shepherd_rejected`,
`legacy_acquire_observatory_rejected`, `legacy_demote_voter_rejected`,
`exact_store_artifact_bytes_retained`,
`artifact_bytes_digest_substitution_rejected`,
`sealed_continuity_transfer_projection`, and
`continuity_projection_consumer_confusion_rejected`,
`continuity_projection_wrong_lineage_rejected`,
`continuity_projection_wrong_source_checkpoint_handle_rejected`, and
`continuity_projection_wrong_bundle_handle_rejected`.

The added thirty-nine names are:
`snapshot_valid_multi_prepared_finalized_restart`,
`snapshot_current_polis_mismatch`, `snapshot_current_epoch_mismatch`,
`snapshot_current_membership_mismatch`, `snapshot_current_boot_mismatch`,
`snapshot_prepared_polis_mismatch`, `snapshot_prepared_epoch_mismatch`,
`snapshot_prepared_membership_mismatch`, `snapshot_prepared_boot_mismatch`,
`snapshot_later_prepared_custody_mismatch`, `snapshot_legacy_owner_injection`,
`snapshot_legacy_shepherd_injection`, `snapshot_legacy_observatory_injection`,
`snapshot_legacy_fence_injection`, `snapshot_legacy_demotion_injection`,
`snapshot_finalized_missing_proposal`,
`snapshot_finalized_missing_endorsements`,
`snapshot_finalized_wrong_operation`,
`snapshot_finalized_insufficient_quorum`,
`snapshot_finalized_duplicate_quorum`, `snapshot_finalized_bad_signature`,
`snapshot_finalized_stale_certificate`, `snapshot_finalized_wrong_boot`,
`snapshot_finalized_invalid_time`, `snapshot_finalized_wrong_prepare_index`,
`snapshot_finalized_wrong_finalize_index`, `snapshot_custody_omitted`,
`snapshot_custody_reencoded`, `snapshot_custody_injected`,
`snapshot_custody_substituted`, `snapshot_custody_byte_digest_mismatch`,
`snapshot_evidence_omitted`, `snapshot_evidence_reencoded`,
`snapshot_evidence_injected`, `snapshot_evidence_substituted`,
`snapshot_evidence_byte_digest_mismatch`,
`validator_available_divergent_rejected`,
`validator_available_ancestral_passed`, and
`validator_unavailable_protected_fallback_passed`.

Result `passed` is required exactly for these eleven names:
`current_three_voter_finalize`, `exact_retry_returns_cached_result`,
`joint_majority_each_config`, `finalize_at_deadline`,
`three_node_checkpoint_restart_reconcile`, `local_clock_skew_apply_parity`,
`exact_store_artifact_bytes_retained`,
`sealed_continuity_transfer_projection`,
`snapshot_valid_multi_prepared_finalized_restart`,
`validator_available_ancestral_passed`, and
`validator_unavailable_protected_fallback_passed`.

Result `reconciled` is required exactly for these six names:
`node_a_local_before_cas`, `node_a_cas_before_final_marker`,
`node_b_local_before_cas`, `node_b_cas_before_final_marker`,
`node_c_local_before_cas`, and `node_c_cas_before_final_marker`.
Every other named case defaults to result `rejected`, producing exact totals
`passed=11`, `reconciled=6`, `rejected=69`, and `selected=86` without reference
to any superseded proof script or receipt.

## Non-goals

- OpenRaft learner/joint/final membership changes (#199).
- Concrete certificate, lease, fence, owner, Shepherd, migration, or recovery
  side effects (#200).
- Kernel checkpoint bundle export/import or snapshot materialization.
- Guardian/API/WSS/Observatory listener integration.
- Models, AWS infrastructure, live demonstration, final #142 delivery, or
  lifecycle closeout.
- Reimplementation of QUIC/OpenRaft transport or any existing authority store.
