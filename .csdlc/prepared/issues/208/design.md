# #208 Authenticated Guardian-Kernel Continuity Bridge Design

## Purpose and closure boundary

The distributed Runtime runs in the production Guardian process while the live
continuity coordinator and its stateful participants run in the separately
supervised kernel. #208 closes that process boundary. It does not decide whether
migration or recovery is authorized; it gives an already-authorized Guardian a
bounded, authenticated way to drive real kernel continuity effects.

This issue cannot close with a listener/client library alone. The production
Guardian binary must validate and construct the client, establish the private
session to its supervised kernel before distributed-runtime readiness, and pass
the non-forgeable capability into the production polis runtime. The production
kernel binary must construct the private listener from `RuntimeInitConfig`, bind
it before public readiness, and dispatch into the same live assembly and
continuity participants used by the running kernel. Normal-build mocks,
standalone fixtures, a synthetic snapshot, or a listener that is never reached
from both binaries do not satisfy #208.

## Private mTLS control plane

The kernel starts a second internal HTTPS listener dedicated to continuity
control. Configuration requires a concrete IPv4 or IPv6 loopback address with a
nonzero unique port, private configured state and staging roots, a server
certificate/key, a Guardian client trust root, an exact logical Guardian
identity, and bounded frame/blob/service/cache/journal policies. Wildcard,
non-loopback, multicast, public, port-zero, duplicate, symlinked, replaceable,
or overlapping paths and endpoints fail before either listener reports ready.

Only TLS 1.3 mutual authentication authorizes application requests. There is no
second application signing authority and no claim that a bearer token or an
unsigned payload authenticates itself. The server validates the client chain,
client-auth EKU, configured trust root, leaf SPKI, logical Guardian identity,
certificate generation, and explicit succession schedule; the client performs
the corresponding server checks. Public Runtime, agent, voter, Shepherd,
Observatory, and distributed authority identities are distinct and denied.

The application codec is strict RFC 8785 canonical JSON inside bounded
length-delimited frames. Each transient envelope carries an RFC 9266
`tls-exporter` channel-binding digest derived from the current TLS session;
the server compares it before decoding or dispatch and never treats it as a
durable credential. Duplicate keys, noncanonical encodings, unknown fields,
NaN/infinity, trailing bytes, a channel-binding mismatch, or a decode/re-encode
mismatch are denied before dispatch. The canonical operation body binds schema,
trust domain, polis, source
and target node, logical Guardian, durable kernel-control identity, durable
channel epoch, monotonic sequence, operation id/kind, deadline, accepted prefix,
and payload digest. The accepted record also binds the authenticated TLS leaf
generation and SPKI. Responses bind the request digest, result state, receipt
digest, and response digest. TLS authenticates the frame in flight; the durable
record and canonical digest authenticate exact replay after reconnect. A retry
on a new connection uses a new exporter-bound envelope around the identical
durable operation body; it cannot replay bytes captured from an old TLS session.

No continuity route is registered on the public Axum service or included in the
Runtime or Observatory OpenAPI. The private listener exposes only a closed typed
operation enum. Unknown methods and generic public control commands cannot
reach continuity dispatch.

## Durable channel epoch and certificate succession

Guardian and kernel process boot identifiers are diagnostic incarnations, not
replay namespaces. Each side stores an exclusively locked logical control
identity and `channel_epoch` beneath its configured private root. A process
restart with the same durable root preserves both values and reconciles accepted
operations before admitting new work. Replacing the root, logical peer, or
channel authority advances the epoch and cannot read or reopen the old epoch.

Certificate renewal is an explicit two-phase succession inside one durable
channel epoch. The operator-controlled Runtime init lists current and next
private-CA leaf SPKIs and its canonical config digest is compared by both peers;
no new application signing key is implied. The persisted succession record
binds old and new leaf SPKIs,
certificate generations, trust domain, polis, logical peers, activation cut,
and retirement deadline. After activation, the predecessor certificate may
authenticate only an exact retry of an operation accepted under that leaf; it
cannot create a new operation. The successor certificate may reconcile such an
operation only when the persisted succession record proves the exact lineage.
The predecessor is removed only after every accepted predecessor operation is
terminal and its bounded retirement deadline has passed. A reply lost before or
after either process restarts therefore returns the one recorded result instead
of becoming inaccessible or executing twice. Stale, future, skipped, ambiguous,
or cross-domain epochs and certificate generations fail closed.

## Live quiesce, checkpoint, and rollback

`adl-runtime-kernel` replaces the one-way checkpoint participant hook with a
sealed, receipt-bearing quiesce protocol. The live assembly owns one complete
registry derived from every running component that declares continuity state.
Startup fails before public readiness when the registry is missing, duplicates,
or adds a participant relative to the validated live service inventory. The
registry includes the canonical ingress/admission gate, recorder and accepted
prefix, live reasoning/mutation state, live governance state, and production
operation-adapter durable state. Test-only participants cannot enter a normal
build.

`quiesce_and_export` executes a durable two-phase operation:

1. close admission for the exact accepted prefix and persist the operation;
2. prepare every participant and retain its receipt;
3. after every required participant is prepared, snapshot those same handles;
4. commit the signed manifest and blobs with rename and directory-fsync order;
5. persist the bundle receipt and reconcile the external checkpoint/marker;
6. return only after the exact terminal result is replayable.

If any participant fails or cancellation/deadline fires before bundle commit,
the coordinator issues receipt-bound `resume_after_failed_quiesce` to every
participant that acknowledged or may have applied prepare. Admission reopens
only after all resume receipts reconcile. If rollback cannot be proved, the
source remains closed and the operation returns `RecoveryRequired`; it never
reports a clean no-effect result. After bundle commit the source stays quiesced
until exact `resume_source` or downstream #204 fencing makes it ineligible.

The manifest is signed by the existing configured kernel continuity authority,
using its canonical manifest codec and explicit trusted-key generation policy.
That authority signs checkpoint content; it does not authenticate the private
control channel and cannot authorize migration, activation, or serving.

## Target staging, validation, and discard

`stage_target` streams bounded bytes into a new opaque generation below the
fixed isolated root; callers never supply a path. `validate_target` loads that
same generation through the kernel verifier and checks manifest signature,
generation and predecessor, accepted prefix, topology/configuration, service
set/schema, canonical file names, per-file and total sizes, and all content
digests before returning opaque possession evidence.

Every target that has not been activated by downstream #204 remains discardable,
including a fully validated pre-fence target. The state machine is
`Staging|Validated -> Discarding -> Discarded`. `discard_target` requires the
matching accepted operation, bundle/possession receipt, channel epoch, root
generation, and manifest/content digests. It removes only that opaque generation,
fsyncs the parent, proves no open handle or directory entry remains, and emits a
durable independently checkable zero-residue receipt. Exact retry returns that
receipt. An activated generation is outside discard authority and is denied
rather than silently deleted.

## Sealed downstream effect ports

#208 remains the sole owner of every kernel and filesystem effect while exposing
only opaque, sealed ports and receipts to downstream protocols:

- `SourceContinuityEffectPort` performs quiesce/checkpoint/resume and returns
  `SourceQuiesceReceipt`, `SourceCheckpointHandle`, and `SourceResumeReceipt`.
  A downstream decision never receives participant handles, roots, or paths.
- `ContinuityBundleSourcePort` is derived only from an exact committed
  `SourceCheckpointHandle`. It returns the retained signed manifest/catalog
  projection and bounded reads for the exact expected ranges; no caller offset
  outside that signed projection and no raw file handle is accepted.
- `TargetContinuityEffectPort` owns stage, verify, and activate effects and
  returns `TargetStageHandle`, `TargetPossessionEvidence`, and
  `TargetActivationReceipt`. Before any write it independently verifies the
  retained signature and trusted-key generation, then matches the exact signed
  entry order/schema/range/length/digest and chunk index/range/digest/predecessor.
  A digest of caller-selected bytes or caller-supplied expected descriptor is
  never sufficient. `TargetPossessionEvidence` is unavailable until the whole
  signed bundle has passed exact validation.
- Stage creation separately mints `TargetCleanupPermit`, bound to the exact
  trust domain, polis, target node/Guardian, durable channel epoch, root and
  stage generations, bundle/manifest/catalog/content digests, and cleanup
  identity. It permits only handle closure, exact nonactivated-stage discard,
  parent fsync, and durable live `TargetDiscardReceipt`. Transfer deadline
  expiry or cancellation removes transfer authority but does not invalidate
  this cleanup authority. It remains usable until the exact stage is Discarded
  or Activated; activation consumes it and any later discard is denied.

#210 may use the source port and target stage/verify operations and may request
#208 cleanup, but returns only `VerifiedTransferPossession` containing the exact
`TargetStageHandle` and `TargetPossessionEvidence`; it never deletes or
activates material. #204 owns the migration executor and private control
operation adapter that decides when already-verified authority calls source
resume, target activate, or target discard. #208 performs those effects and
returns the opaque receipts. Thus there is exactly one cleanup/effect owner
(#208), without moving migration policy into this issue.

#204 alone decides whether verified possession may activate. Its sealed adapter
may invoke #208's `activate_target` effect only with the exact finalized
migration decision and matching stage, possession, cleanup, route, membership,
certificate, boot, and lineage bindings. #208 revalidates those bindings,
atomically consumes the `TargetCleanupPermit`, durably records the activated
generation, and returns a `TargetActivationReceipt`. Exact retry after process
restart returns that retained receipt without repeating the effect. A stage with
an activation receipt is never discardable; any later cleanup or discard request
for it fails closed. #208 therefore owns the activation effect and receipt, but
does not own or infer the activation decision.

## Crash, bounds, and filesystem safety

Both client and server maintain exclusively locked bounded canonical journals
and replay/result caches. Before any effect the server persists an accepted
operation binding. After every effect it persists an exact receipt, then
reconciles the external checkpoint and completion marker before returning.
Retry is cache-first but completes owed reconciliation. Conflicting retry,
rollback, corrupt/noncanonical state, capacity exhaustion, ambiguous
initialization, or peer/epoch mismatch fails closed.

Deadline and cancellation are checked before every external effect and bounded
stream operation. Before the first effect they may return a transient no-effect
result. Once a durable effect begins, retry/restart reconciles the accepted
operation. Every file is accessed through an already-open handle with metadata,
device/inode, ownership, link-count, and MAX+1 checks. Roots, generation
directories, manifests, blobs, temporary files, locks, and all parents reject
symlinks or replacement. Directory entries, frame sizes, blob sizes, service
counts, total bytes, journals, and caches are bounded before allocation.

## Exact production paths

The issue owns the following production wiring, not merely new modules:

- kernel protocol/coordinator: `adl-runtime-kernel/src/continuity.rs` and
  `adl-runtime-kernel/src/continuity_control.rs`;
- live participant registry and real handles:
  `adl-runtime-kernel/src/assembly.rs`, `live_continuity.rs`, `ingress.rs`,
  `reasoning.rs`, `governance.rs`, and `operations.rs`;
- kernel configuration/startup/export:
  `adl-runtime-kernel/src/config.rs`, `lib.rs`, and
  `bin/adl-runtime-kernel.rs`;
- Guardian client and production initialization:
  `adl-runtime/src/kernel_continuity_client.rs`, `config.rs`, `guardian.rs`,
  `distributed/mod.rs`, `distributed/polis_runtime.rs`, `lib.rs`, and
  `bin/adl-runtime-guardian.rs`.

The Guardian client exposes the sealed ports and opaque handles named above:
#210 receives only bundle-read, target-stage/verify, and cleanup-request access;
#204 receives source resume, target activate/discard, and their receipts through
its own private control-operation adapter. Neither receives a private key, raw
filesystem path, normal-build injected trait, migration authority, owner commit,
fence, activation decision, or serving authority from #208.

## Exact proof contract

The denominator is exactly fifty-six named cases:

`internal_listener_config_valid`, `nonloopback_bind_rejected`,
`unsafe_root_config_rejected`, `guardian_identity_distinct`,
`guardian_mtls_authorized`, `unknown_client_certificate_denied`,
`invalid_client_eku_denied`, `stale_certificate_denied`, `bearer_only_denied`,
`agent_control_identity_denied`, `wrong_trust_domain_denied`,
`wrong_polis_denied`, `wrong_node_denied`, `wrong_guardian_denied`,
`replay_rejected`, `conflicting_duplicate_rejected`,
`reordered_request_rejected`, `wrong_kernel_instance_denied`,
`durable_channel_restart_retry`, `certificate_succession_retry`,
`stale_channel_epoch_denied`, `real_quiesce_checkpoint`,
`partial_quiesce_rollback`, `signed_bundle_export`, `export_bounds`,
`export_exact_retry`, `source_resume`, `source_resume_exact_retry`,
`isolated_stage`, `isolated_import_validate`, `wrong_manifest_signature`,
`wrong_generation`, `wrong_predecessor`, `wrong_accepted_prefix`,
`wrong_topology`, `wrong_config`, `wrong_service_set`, `wrong_service_schema`,
`corrupt_content`, `oversized_bundle`, `caller_path_rejected`,
`symlink_path_rejected`, `deadline_before_effect`, `cancellation_no_partial`,
`restart_after_accept`, `crash_after_bundle_commit`, `target_discard`,
`discard_exact_retry`, `validated_target_discard`, `zero_residue`, `dual_open`,
`opened_handle_replacement`, `evidence_redaction`, `public_surface_absent`,
`guardian_initialization_live`, and `participant_registry_complete`.

The tracked `continuity-boundary-subassertion-map.json` is part of the proof
contract. Its `cases` array is the canonical exact ordered fifty-six-case
denominator: ordinals are contiguous from 1 through 56, every named case occurs
exactly once, and every row declares the exact `proved:case:<name>` marker. It
also contains exactly eight boundary rows and sixty-four named boundary
subassertions: eight each for config, TLS, identity, domain, generation, prefix,
path, and size. A separate exact twelve-row lifecycle subassertion table proves
cleanup-permit survival across expiry/cancellation, cleanup and source-resume
restart reconciliation, #204-only activation decision authority, #208-owned
activation effect and retained receipt, cleanup-permit consumption, and denial
of discard after activation. These lifecycle assertions refine named cases in
the same fifty-six-case denominator; they do not inflate it.

The domain row binds canonical RFC 8785 acceptance and exact rejection markers
for duplicate keys, noncanonical encoding, unknown fields, NaN/infinity,
trailing bytes, decode/re-encode mismatch, and unknown operation kind; the TLS
row separately retains exporter-mismatch proof. The named domain, polis, and
node denial cases remain in the fifty-six-case denominator. The producer and
validator require byte-for-byte map parity, the exact ordered case names,
contiguous ordinals, uniqueness, all declared counts, exact expected
outcomes/markers, and SHA-256
`9a6d7834557f626487aae3115464ee60f19b06609b7ea9e6a24399a60eec8745`.
Retry/crash proof enumerates
accepted journal, each participant prepare/resume receipt, admission transition,
bundle/target effect, result, checkpoint, marker, response cache, certificate
succession, process restart, and reply loss on both client and server.

Validation runs serially: focused Runtime and kernel tests, Runtime Clippy,
kernel library/binary Clippy, exact-range diff hygiene, producer, independent
exact-head review, then the distinct immutable validator. The diff verifier
loads the recorded execution base and proving source revisions, requires both
exact Git objects plus base ancestry, runs whitespace/EOF hygiene across the
complete `base..source` range, and rejects dirty protected paths; a bare
working-tree `git diff --check` is not proof. No review or validator may race the
producer or source-changing validation.

## Non-goals

- Consensus, distributed authority issuance, membership, lease/fence policy,
  migration/recovery decisions, ownership, activation decisions, or serving
  eligibility. #204 owns the activation decision; the target activation effect
  and `TargetActivationReceipt` remain #208-owned and are callable only through
  #204's independently verified finalized migration decision.
- Remote bundle transport (#210) or migration/recovery orchestration (#204/#211).
- Public continuity routes, Shepherd/model execution, AWS resources, live
  Wuji/AWS qualification, final #142 delivery, merge without operator
  authorization, or lifecycle closeout.
