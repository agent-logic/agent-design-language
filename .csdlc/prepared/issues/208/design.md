# #208 Authenticated Guardian-Kernel Continuity Bridge Design

## Purpose

The distributed Runtime runs in the Guardian process while the live continuity
coordinator and service participants live in the separately supervised kernel.
#208 provides the missing production bridge. It does not decide whether a
migration or recovery is authorized; it gives an already-authorized Guardian a
bounded, authenticated way to ask its own kernel for real continuity effects.

## Private control plane

The kernel starts a second internal HTTPS listener dedicated to continuity
control. Configuration requires a concrete loopback address with a nonzero
port, a private configured state root, a server certificate/key, a Guardian
client trust root, and an exact expected Guardian control identity. Wildcard,
non-loopback, multicast, public, port-zero, duplicate, symlinked, or overlapping
roots fail before either listener binds.

The internal listener requires mutual TLS. Its server trust is distinct from
the public Runtime/Observatory listener, and its client leaf SPKI and certificate
generation must match the configured Guardian control identity. Bearer tokens,
ordinary node/agent certificates, distributed voter keys, Shepherd keys, and
the public control signing identity do not authorize this listener.

After mTLS, every canonical request and response binds schema, trust domain,
polis, node, guardian, kernel instance, Guardian boot generation, Guardian and
kernel certificate generations, monotonic sequence, operation id, operation
kind, deadline, payload digest, and response digest. A bounded durable replay
cache returns the byte-identical response for an exact retry and rejects a
conflicting duplicate or reordered request before dispatch. Receiver and
sender generation change creates a new authenticated namespace; it never
reopens an old cache.

No internal continuity route is registered on the public Axum service or
listed in the public Runtime or Observatory OpenAPI. The internal listener
exposes only a closed typed operation enum. Unknown methods and generic public
control commands cannot reach continuity dispatch.

## Operations

The bridge exposes opaque Guardian-library operations:

- `quiesce_and_export`: close admission through the live kernel gate, run the
  live `CheckpointCoordinator` across the exact active participant set, commit
  its signed manifest and service blobs under the fixed configured checkpoint
  root, then return a bounded opaque bundle handle plus exact manifest,
  accepted-prefix, topology, configuration, service-set, size, chunk, and
  content digests;
- `resume_source`: resume only the exact quiesced generation after the caller
  presents the matching operation/bundle receipt;
- `stage_target`: stream bounded bytes into a new opaque staging generation
  beneath the fixed isolated root; the caller never supplies a path;
- `validate_target`: load the exact staged generation through the kernel
  continuity verifier, checking manifest signature, predecessor/generation,
  accepted prefix, topology/configuration, service set/schema, file names,
  sizes, and content before returning opaque possession evidence; and
- `discard_target`: remove only the exact incomplete isolated generation and
  return a durable independently checkable zero-residue receipt.

The Guardian client never receives private key material or arbitrary kernel
filesystem access. Bundle handles contain private fields and are meaningful
only to the exact configured client/root/kernel generation. Large content is
streamed with explicit per-frame and total bounds; it is not buffered into one
unbounded JSON or protobuf value.

## Durable operation state

Both client and server maintain exclusively locked, bounded canonical journals
and replay/result caches. Before any kernel effect the server persists an
accepted operation binding. After each effect it persists an exact receipt,
then reconciles an external node-local checkpoint and completion marker before
returning success. Exact retry is cache-first but completes owed reconciliation
before returning. Conflicting retry, rollback, corrupt/noncanonical state,
ambiguous initialization, or mismatched peer generations fails closed.

Deadline and cancellation are checked before each external effect and each
bounded stream operation. Before the first effect they can return a transient
no-effect result. Once a durable effect has started, cancellation cannot invent
failure or success: restart/retry reconciles the exact accepted operation.
Source admission remains closed until exact resume or later #204 fencing makes
the source ineligible. Target material remains isolated until #204 activation;
#208 never publishes ownership or serving authority.

Every file read uses an already-open handle, metadata and inode checks, and a
MAX+1 bound. State roots, generation directories, manifests, blobs, temporary
files, locks, and parent components reject symlinks and replacements. Directory
entries and total bytes are bounded before allocation. Rename and directory
fsync order is explicit for pending-to-committed bundle publication.

## Runtime wiring boundary

`adl-runtime-kernel` owns the internal listener and live continuity adapter.
`adl-runtime` owns an opaque `KernelContinuityClient` constructed only by the
production Guardian initialization path from the validated Runtime config and
private key source. #204 consumes that client through a closed library API; it
cannot inject a normal-build mock or caller-defined trait implementation.
Tests use cfg(test)-only deterministic authorities inside their owning module.

#208 does not wire distributed policy, migration state, recovery state, lease,
fence, activation, Observatory transfer, remote transport, or cloud copying.
Those remain #204 and final #142 integration.

## Exact proof denominator

The denominator is exactly thirty-six cases, with exact name/result/marker and
declared subassertion parity:

`internal_listener_config_valid`, `nonloopback_bind_rejected`,
`guardian_identity_distinct`, `guardian_mtls_authorized`,
`unknown_client_certificate_denied`, `bearer_only_denied`,
`agent_control_identity_denied`, `replay_rejected`,
`wrong_kernel_instance_denied`, `real_quiesce_checkpoint`,
`signed_bundle_export`, `export_bounds`, `export_exact_retry`,
`source_resume`, `source_resume_exact_retry`, `isolated_stage`,
`isolated_import_validate`, `wrong_manifest_signature`, `wrong_topology`,
`wrong_config`, `wrong_service_set`, `wrong_service_schema`,
`corrupt_content`, `oversized_bundle`, `caller_path_rejected`,
`symlink_path_rejected`, `deadline_before_effect`,
`cancellation_no_partial`, `restart_after_accept`,
`crash_after_bundle_commit`, `target_discard`, `discard_exact_retry`,
`zero_residue`, `dual_open`, `evidence_redaction`, and
`public_surface_absent`.

The retry/crash cases enumerate accepted-journal, each kernel effect and
receipt, stream interruption, result, checkpoint, marker, response-cache, and
reply-loss windows on both client and server. Bounds cases enumerate N/N+1
frames, blobs, services, bytes, cache records, and opened-handle replacement.

## Non-goals

- Consensus, distributed authority issuance, membership, lease/fence policy,
  or serving eligibility.
- Migration/recovery orchestration or remote bundle transport (#204).
- Shepherd/model execution, AWS resources, live Wuji/AWS qualification, final
  #142 delivery, merge without operator authorization, or lifecycle closeout.

