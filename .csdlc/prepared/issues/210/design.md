# #210 Authenticated Continuity Transfer Design

## Purpose

#210 moves one real #208 continuity bundle between the exact authorized source
and target voters. It extends the existing #191 authenticated polis connection
with a closed typed data-plane service; it does not create a second transport,
use a generic message string, or decide migration/fencing/activation policy.

## Transfer authority

Only a private finalized #201 transfer payload can create an
`EstablishedContinuityTransfer`. It binds the trust domain, polis, operation
and transfer ids, lineage, source and target node/guardian/boot/certificate
generations, exact #199 membership cut and committed index, #208 bundle handle,
signed manifest and catalog digests, total bytes, chunk count and size, absolute
deadline, uncertainty policy, and exact operation digest.

Construction revalidates the source and target against the current #191 route
cut and #203 certificate authority. Caller addresses are routing hints only.
The capability has private fields and exposes no generic send method. A closed
server ingress authorizes the session role before payload decoding. Vote,
AppendEntries, InstallSnapshot, unknown kinds, and ordinary polis messages
cannot be confused with continuity frames; conversely a transfer session cannot
send Raft or any future generic message.

Any route-cut, membership-index, certificate, boot, lineage, source, target, or
token-generation drift closes the session and requires a new authorized token.
An exact retry of a retained published result is cache-first and does not
reauthorize or retransmit.

## Bounded stream

The source reads only through the opaque #208 bundle reader. The target writes
only through the opaque #208 isolated-stage writer. No caller supplies a path,
file handle, raw checkpoint root, or synthetic manifest.

The stream begins with the exact signed manifest and catalog binding. Each
canonical frame binds schema, transfer id, manifest digest, chunk index,
absolute offset, payload length, payload digest, predecessor frame digest,
source/target session generations, and final flag. Per-frame bytes, frame
count, total bytes, service count, open files, queued frames, and in-flight
requests are explicitly bounded. Backpressure permits no unbounded buffering.

The target persists one accepted-prefix record before acknowledging a frame.
An exact duplicate returns the retained frame receipt without rewriting. A
conflicting duplicate, gap, reordering, overlap, wrong predecessor, wrong
length/digest, or N+1 input fails before changing the accepted prefix. Restart
reopens the exact prefix and resumes from its next offset. A source proposal
that disagrees with that prefix fails closed.

`SnapshotCatalogVerifier` gains a bounded incremental verifier. It validates
the exact signed catalog/manifest once, then feeds each accepted frame into
per-entry and whole-content digests without retaining the whole bundle in
memory. Completion requires exact service/file ordering, schemas, chunk list,
total length, and whole digest. Only then does #208 finalize the isolated stage
and return opaque possession evidence.

## Durable protocol

Source and target use exclusively locked canonical journals, result caches,
external node-local checkpoints, and completion markers. The order is:

1. persist exact authorized transfer and empty/retained prefix before bytes;
2. persist each bounded frame and its prefix receipt before acknowledgment;
3. persist incremental verifier completion and #208 possession receipt;
4. persist canonical completion result;
5. reconcile external checkpoint and local marker; and
6. publish the result and return it.

There is no cross-node filesystem transaction. Until the target published
result exists, the material stays isolated and creates no ownership. Cache-first
retry reconciles any owed prefix, result, checkpoint, marker, and publication
without duplicate writes. Rollback, corruption, noncanonical state, ambiguous
initialization, dual open, or checkpoint disagreement fails closed.

Deadline and cancellation are checked before admission and every network/file
operation. Before the first durable effect they yield no-effect denial. After a
durable accepted prefix exists, cancellation or partition stops I/O but does
not invent a terminal result; exact retry resumes or an authorized abort calls
#208 discard. Disk-full, target restart, source restart, reply loss, and
connection replacement follow the same durable-prefix reconciliation.

## Cleanup and evidence

Abort is token-bound and idempotent. It closes the transfer session, reconciles
the target prefix, invokes exact #208 discard, removes only transfer-owned
state, and returns a live zero-residue receipt covering stage, journals,
temporary files, open session, and checkpoint/result namespaces. A local
absence claim is insufficient; the target #208 authority must attest the
staging generation is absent.

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

Every case binds exact result and marker digests. The producer and validator
reject missing, extra, duplicate, renamed, reordered, or nonpassing cases; no
case may expand its own denominator at execution time.

## Non-goals

- Consensus, authority issuance, membership transition, local continuity
  implementation, migration/recovery policy, fencing, activation, OwnerCommit,
  or serving transfer.
- Model execution, AWS provisioning, live qualification, final #142 delivery,
  merge without operator authorization, or lifecycle closeout.

