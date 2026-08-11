# #210 Authenticated Continuity Transfer Design

## Purpose

#210 moves one real #208 continuity bundle between the exact authorized source
and target voters. It extends the existing #191 authenticated polis connection
with a closed typed data-plane service; it does not create a second transport,
use a generic message string, or decide migration/fencing/activation policy.

## Transfer authority

Only #201's private sealed `ContinuityTransferGrantProjection` can create an
`EstablishedContinuityTransfer`. The projection is available only for an exact
finalized continuity-transfer operation and has no public constructor or generic
artifact conversion. It binds the trust domain, polis, operation
and transfer ids, lineage, source and target node/guardian/boot/certificate
generations, exact #199 membership cut and committed index, #208 bundle handle,
retained signed manifest and catalog bytes plus digests and trusted-key
generation, canonical entry order/schema/range/length/digest, canonical chunk
index/range/digest/predecessor, total bytes, chunk count and size, absolute
deadline, uncertainty policy, cleanup identity, and exact operation digest.

Construction revalidates the source and target against the current #191 route
cut and #203 certificate authority. Caller addresses are routing hints only.
The capability has private fields and exposes no generic send method. A closed
server ingress authorizes the session role before payload decoding. Vote,
AppendEntries, InstallSnapshot, unknown kinds, and ordinary polis messages
cannot be confused with continuity frames; conversely a transfer session cannot
send Raft or any future generic message.

The same bindings are revalidated before admission, every read/write, resume,
completion, and result publication. Any route-cut, membership-index,
certificate, boot, lineage, source, target, or token-generation drift closes the
session and requires a new authorized token. Drift after an accepted prefix
stops further I/O while preserving the reconcilable isolated stage and its
separate cleanup authority.
An exact retry of a retained published result is cache-first and does not
reauthorize or retransmit.

## Bounded stream

The source reads only through #208's sealed `ContinuityBundleSourcePort`, derived
from the exact committed `SourceCheckpointHandle`. The target writes only
through #208's sealed `TargetContinuityEffectPort`, which creates an opaque
`TargetStageHandle`. No caller supplies a path, raw handle, expected descriptor,
checkpoint root, or synthetic manifest.

The stream begins with the exact signed manifest and catalog binding. Each
canonical frame binds schema, transfer id, manifest digest, chunk index,
absolute offset, payload length, payload digest, predecessor frame digest,
source/target session generations, and final flag. Per-frame bytes, frame
count, total bytes, service count, open files, queued frames, and in-flight
requests are explicitly bounded. Backpressure permits no unbounded buffering.

Before any target write, #210 validates the retained signed manifest/catalog and
trusted-key generation, resolves the exact canonical entry and chunk descriptor,
and matches entry order, identity, schema, absolute range, length and digest plus
chunk index, range, digest and predecessor. #208 independently repeats that
comparison inside `TargetContinuityEffectPort`; neither layer accepts a
caller-selected expected descriptor or merely hashes caller-selected bytes.

The target journals a pending exact frame effect, performs the #208 verified
write and fsync, persists or reconstructibly checkpoints the incremental
verifier state, and commits the new accepted-prefix receipt before
acknowledgment. These bytes/verifier/prefix stages form one crash-reconcilable
effect: recovery can prove and finish only the same exact transition or fail
closed; it never skips a step or advances the prefix over absent/unverified
bytes. An exact duplicate returns the retained frame receipt without rewriting.
A conflicting duplicate, gap, reordering, overlap, wrong predecessor, wrong
length/digest, zero-length nonfinal frame, wrong final-frame shape, cross-entry
range, integer overflow, or N+1 input fails before effect. Restart reopens the
exact prefix and rebuilds/verifies incremental state from durable bytes before
resuming at the next signed range. A source proposal that disagrees fails closed.

`SnapshotCatalogVerifier` gains a bounded incremental verifier. It validates
the exact signed catalog/manifest once, then feeds each accepted frame into
per-entry and whole-content digests without retaining the whole bundle in
memory. Completion requires exact signing-key generation, signature, service and
file ordering, entry identity/schema/range/length/digest, chunk list and ranges,
total length, and whole digest. Missing, extra, duplicate, reordered, overlapping
or gapped entries/chunks fail closed. Only then does #208 return opaque
`TargetPossessionEvidence`; #210 wraps the exact `TargetStageHandle` and evidence
as `VerifiedTransferPossession`. Possession is isolated and nonauthoritative.

## Durable protocol

Source and target use exclusively locked canonical journals, result caches,
external node-local checkpoints, and completion markers. The order is:

1. persist exact authorized transfer, #208 stage/cleanup handles, and the
   empty/retained prefix before bytes;
2. revalidate live route/identity truth plus the signed entry/chunk/range
   expectation and persist a pending exact frame effect;
3. perform and fsync the #208 verified write, then persist/reconcile incremental
   verifier state and the new prefix receipt before acknowledgment;
4. persist full verifier completion, `TargetPossessionEvidence`, and
   `VerifiedTransferPossession`;
5. persist the canonical completion result;
6. reconcile external checkpoint and local marker; and
7. publish the result and return it.

There is no cross-node filesystem transaction. Until the target published
result exists, the material stays isolated and creates no ownership. Cache-first
retry reconciles any owed prefix, result, checkpoint, marker, and publication
without duplicate writes. Rollback, corruption, noncanonical state, ambiguous
initialization, dual open, or checkpoint disagreement fails closed.

Deadline and cancellation are checked before admission and every network/file
operation. Before the first durable effect they yield no-effect denial. After a
durable stage or accepted prefix exists, cancellation or partition stops I/O but
does not invent a terminal result; exact retry resumes or an authorized abort
requests #208 discard with the separate cleanup permit. Disk-full, target
restart, source restart, reply loss, and connection replacement follow the same
durable-prefix reconciliation. Frame bytes, entries, chunks, services, files,
total bytes, queued frames, in-flight requests, open source readers, open target
stages, concurrent transfers, journals, caches and diagnostics all have exact N
and N+1 behavior checked before allocation or effect.

## Cleanup and evidence

At stage creation #208 separately mints `TargetCleanupPermit`, bound to exact
domain, polis, target node/Guardian, durable channel epoch, root/stage
generations, transfer/bundle/manifest/catalog/content digests, and cleanup
identity. It is discard-only and remains valid after the transfer deadline
expires, cancellation fires, the network session closes, or either process
restarts. It terminates only with `TargetDiscardReceipt` or a downstream
`TargetActivationReceipt`; activation makes later discard invalid.

Abort is transfer-operation-bound and idempotent, but it does not use expired
move authority to delete. It closes the transfer session, reconciles the target
prefix, and requests #208 cleanup using the separate exact permit. #208 alone
closes handles, removes the exact nonactivated stage, fsyncs its parent, and
returns live durable zero-residue proof covering stage, temporary files, open
handles, journal, checkpoint, marker and result namespaces. #210 never deletes
or activates material. A local absence claim is insufficient; #208 must attest
the exact root/stage generation is absent. #204 owns later migration/control
decisions and invokes #208 source resume, target activation, or target discard.

Evidence contains only opaque references, digests, bounded counts, outcomes,
and hosted/runtime attestations. It never includes checkpoint content, raw
identity, certificate, token, address, path, key, signature, or secret.

## Exact proof denominator

The denominator is exactly forty-five cases, with one required marker per exact
name and no prose-only hidden denominator:

`authorized_transfer`, `real_bundle_source`, `exact_target_stage`,
`incremental_catalog_verify`, `resume_after_partition`,
`exact_retry_cached`, `wrong_source_denied`, `wrong_target_denied`,
`wrong_polis_denied`, `wrong_domain_denied`, `wrong_lineage_denied`,
`wrong_membership_cut_denied`, `stale_certificate_denied`,
`wrong_boot_generation_denied`, `generic_send_denied`,
`raft_rpc_confusion_denied`, `unknown_kind_denied`, `frame_n_accepted`,
`frame_n_plus_one_denied`, `reordered_frame_denied`,
`exact_duplicate_frame_cached`, `conflicting_duplicate_denied`,
`wrong_predecessor_denied`, `wrong_chunk_digest_denied`,
`wrong_manifest_denied`, `oversized_frame_denied`,
`oversized_total_denied`, `deadline_before_first_byte`,
`deadline_midstream`, `cancellation_before_effect`,
`cancellation_midstream`, `source_restart_resume`,
`target_restart_resume`, `crash_after_admission`, `crash_after_frame_write`,
`crash_after_prefix_receipt`, `crash_after_completion_result`,
`crash_before_checkpoint`, `crash_after_checkpoint`, `reply_loss_retry`,
`disk_full_no_false_success`, `coherent_rollback_denied`,
`unsafe_path_denied`, `zero_residue_abort`, and `evidence_redaction`.

The tracked `continuity-transfer-acceptance-map.json` is part of the proof
contract. Its canonical `case_manifest` is the sole ordered case denominator:
ordinals 1 through 45 each bind one exact case name, expected `pass` result,
and unique `pass:CASE-<three-digit-ordinal>:<case_name>` marker. SHA-256
`2929794678966f233f8caf4df3131d9188cac3e5107fc0190cee9dd4fd1d71cd`
binds that exact ordered result/marker table, exactly eight acceptance rows,
all forty-five case names, and exactly eighty-four unique named subassertions.
Every case has at least one direct subassertion mapping; in particular,
`exact_retry_cached`, `wrong_polis_denied`, `wrong_domain_denied`, and
`generic_send_denied` are no longer covered only by the case manifest. It
makes route/membership/certificate/boot
drift, framing/final/range errors, signed catalog entry/chunk/range checks,
resource N+1 boundaries, bytes/verifier/prefix crash order, cleanup after
expiry/cancel/restart, effect ownership, redaction, and proof sequencing
machine-addressable rather than prose-only.

Every case and subassertion binds exact result and marker digests. AC-3 lists
`conflicting_duplicate_denied` because its prefix-conflict subassertion maps to
that case. AC-6 includes the explicit machine assertion
`transfer_has_no_activation_or_deletion_authority_and_cleanup_activated_stage_denied`;
the implementation and proof must show that #210 exposes neither activation nor
deletion authority while #208 alone performs discard effects. The focused test,
producer, and validator independently load and hash the map and reject missing,
extra, duplicate, renamed, reordered, wrongly mapped/marked, or nonpassing
case-manifest or subassertion evidence. They also reject any case without a
direct subassertion mapping; no case may expand its own denominator at execution
time.

Validation is strictly serial: focused tests, strict Clippy, exact diff hygiene,
producer, fresh independent exact-head review, then the distinct validator. The
diff verifier loads recorded `execution_base_revision` and
`proving_source_revision`, requires exact Git objects plus base ancestry, checks
whitespace and EOF over the complete `base..source` range, and rejects dirty
protected paths. A working-tree-only diff is insufficient. Review and validator
must bind the exact proving source and cannot race any source-changing lane.

## Non-goals

- Consensus, authority issuance, membership transition, local continuity
  implementation, migration/recovery policy, fencing, activation, OwnerCommit,
  or serving transfer.
- Kernel/filesystem deletion, stage activation, source resume, or cleanup
  ownership; these effects remain #208-owned and migration decisions remain
  #204-owned.
- Model execution, AWS provisioning, live qualification, final #142 delivery,
  merge without operator authorization, or lifecycle closeout.
