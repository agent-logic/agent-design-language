# #607 Design — Warm AWS Polis startup

## Objective

Turn the proven #605 two-node AWS qualification into a warm-launch path. Normal
launch must create the Runtime and GPU instances, attach sealed persistent data
volumes, validate their immutable identities, and start Guardian, Runtime, and
Ollama without compiling source, installing packages, or downloading models.

## Authority and boundary

- Canonical issue: `agent-logic/agent-design-language#607`.
- #605 / PR #606 is the behavioral and security baseline.
- This issue owns startup packaging, warm storage, Terraform attachment, timing
  receipts, and one bounded AWS validation under the existing USD 20 ceiling.
- Production HA, DNS, ACM, routing, autoscaling, multi-region replication, and
  24/7 cutover remain outside scope.

## Terraform state and ownership

Two Terraform roots have disjoint state and resource ownership:

- `infra/aws/runtime/gpu-proof/warm-storage` owns the two encrypted persistent
  EBS data volumes and their exact tags. Its state is stored separately beneath
  `.adl/local/issue607/warm-storage`. It never owns launch instances,
  attachments, security groups, or disposable root volumes.
- `infra/aws/runtime/gpu-proof` remains the compute root. It receives exact warm
  volume IDs and seal digests as inputs, reads their metadata, and owns only
  `aws_volume_attachment` resources plus the existing disposable #605 graph.
  It never imports, creates, adopts, or destroys the warm volumes.

Both roots bind the same account, region, availability zone, KMS key ARN, owner
token, and artifact generation. Preflight rejects missing tags, wrong KMS key,
wrong AZ, attachment to another instance, or a volume present in both states.
Before every paid apply and destroy, a machine-readable saved-plan validator
inspects `terraform show -json` and rejects any create, update, replace, or
delete action against a warm volume, KMS key, seal object, or retained snapshot
from the compute root. The storage root is never destroyed by a compute trap.

## Two-phase contract

### Preparation phase

An explicit `prepare` action may perform slow work once. It:

1. Builds the reviewed Linux/x86-64 Runtime and Guardian binaries through the
   repository build path and records their source revision and SHA-256 digests.
2. Uses the storage Terraform root to create one encrypted Runtime data volume
   and one encrypted GPU data volume in the selected availability zone. A
   preparation instance may attach them only under a separate preparation plan.
3. Hydrates a complete launch and qualification closure. Runtime content
   includes Guardian, Runtime kernel, `adl`, `csm`, the Shepherd proof binary,
   Vector, lifecycle and six-resident driver scripts, task-panel inputs,
   Runtime-init/OpenAPI/schema files, TLS/config material, required dynamic
   libraries, and exact invocation manifests. GPU content includes the pinned
   Ollama binary and complete exact model store from immutable versioned S3
   objects. No launch dependency may be omitted merely because #605 built or
   installed it dynamically.
4. Writes a canonical sealed-volume manifest to each filesystem and to the
   authorized S3 evidence prefix. The manifest binds volume ID, availability
   zone, filesystem UUID, artifact manifest VersionId/digest, source revision,
   file digests, model digests, and preparation generation.
5. Detaches the volumes cleanly and leaves no preparation instance running.

The preparation instance is governed as a short-lived, least-privilege build
node rather than an administrative escape hatch. Its security group admits SSH
only from the same operator CIDR used by the launch nodes and exposes no Runtime
or Ollama service port. It requires IMDSv2, uses the single shared operator key,
and receives only exact-prefix read access to authorized S3 inputs, exact-prefix
write access to preparation receipts, and the minimum KMS and EBS permissions
needed for the two named volumes. SSM is recovery-only. Its root volume deletes
on termination, an independent deadline terminates it even if the controller is
interrupted, and preparation success is impossible until plan/receipt evidence
shows the instance, ENI, security group, temporary IAM attachment, and root
volume are absent while only the separately owned warm volumes remain.

Preparation is never hidden inside normal launch. A missing or stale seal causes
launch to fail before paid compute is created when it can be detected locally,
or before services activate when guest filesystem validation is required.

### Warm launch phase

The compute Terraform root receives the two prepared volume IDs and their
expected seal digests as explicit inputs and owns only their attachments. It retains the existing two instances,
security groups, shared SSH key, IAM profiles, private Ollama route, public SSH,
and deadline termination protections. It adds deterministic EBS attachments.

Cloud-init performs only bounded local operations: mount by volume identity,
activate a dm-verity mapping from the authorized root hash, validate required
file identities without rescanning every content block, install no packages,
copy no large artifacts, start the packaged systemd units, and emit readiness receipts.
The Runtime node waits for the GPU ready receipt and private Ollama health before
starting Guardian. Both data volumes remain outside disposable-instance cleanup.

## Image and package boundary

Normal launch cannot rely on `apt`, `dnf`, `pip`, `cargo`, `rustup`, Git, Snap,
or an Ollama pull. Preflight resolves mutable SSM aliases once, then binds the
exact Runtime and GPU AMI IDs plus owner, creation date, architecture, root
device, virtualization, boot mode, and image digest metadata into the
preparation authorization and both seals. Preparation boots those exact AMIs
and records an executable facility/ABI inventory covering cloud-init, systemd,
mount/dm-verity, filesystem support, libc/dynamic libraries, NVIDIA driver,
CUDA ABI, and GPU device access. Both warm launches must use the same exact AMI
IDs and repeat the bounded facility/ABI check before activation. Application,
validation, Vector, and model content comes only from the sealed data volumes;
normal launch does not repair an AMI dynamically.

## Timing truth

Controller and guest receipts record these boundaries:

- Terraform apply start;
- each EC2 instance entering `running`;
- each cloud-init activation script start;
- GPU service and complete model-set ready;
- Guardian/Runtime API ready;
- service-ready receipt observed by the controller;
- qualification-complete receipt after the complete #605 assertions.

The controller measures apply-to-observed-service-ready with one local monotonic
clock; it never subtracts guest wall-clock timestamps. Each guest uses
`CLOCK_BOOTTIME` through the packaged readiness helper for cloud-init-start to
local-ready duration. UTC timestamps are correlation metadata only and include
clock source and synchronization status. The `service_ready` schema requires
both exact instances, seal generation, dm-verity roots, complete model residency,
private Ollama health, Guardian supervision, Runtime health/readiness, and
authenticated HTTPS/WSS. It cannot be emitted by either guest alone. The later
`qualification_complete` schema additionally requires both per-model Shepherd
inferences, six governed ACC executions, restart/state/degradation/Vector/log/
shutdown assertions, and exact receipt digests from #605.

Guest activation from that guest's cloud-init activation start to that guest's
`local_ready` receipt must be at most 30 seconds for each node.
Controller apply-start to observed `service_ready` targets 120 seconds or less.
Qualification duration is reported separately and is not hidden inside either
startup number. A missed target is a failing result, never rounded, excluded, or
relabeled as success.

## Performance feasibility

The GPU warm volume uses gp3 with explicit size, 3,000 IOPS, and at least 500
MiB/s throughput for qualification; Runtime storage uses an explicit measured
profile. Preparation writes and reads every allocated content block before seal,
proves there is no snapshot lazy-initialization dependency, and records exact
artifact bytes. dm-verity makes launch integrity verification proportional to
manifest/root-hash verification rather than a full-volume scan. The timing
receipt budgets mount and verity activation, Ollama start, model page-in,
Guardian/Runtime activation, dependency convergence, and controller receipt
observation separately; their sum must fit the 30-second guest target rather
than relying on an unmeasured aggregate.

## Storage safety

- Warm volumes are encrypted, non-root, single-writer, and AZ-bound. Immutable
  content is exposed read-only through dm-verity. Runtime state, Ollama scratch,
  logs, sockets, caches, and temporary files use disposable root-volume paths or
  tmpfs and are never written into trusted content.
- Ordinary compute destroy removes attachments and disposable root volumes but
  preserves the two warm data volumes.
- Explicit storage cleanup owns deletion of obsolete warm volumes only after a
  replacement seal is proven and a separate single-use authorization selects
  the exact IDs. It is not reachable from compute cleanup.
- Cross-AZ, wrong filesystem UUID, stale generation, partial hydration, missing
  artifact, digest mismatch, or unexpected writable residue fails closed.
- TLS private material, when present, remains in a mode-0600 service-owned
  encrypted subtree; public immutable content cannot modify it. A post-run
  dm-verity/root-hash check proves trusted content did not change.

## Authorization and cost

Preparation, warm launch 1, and warm launch 2 each require separate consumed-once
authorizations. An aggregate envelope binds all three action manifests, exact
saved-plan digests, AMI IDs, volume IDs, KMS/AZ/network/SSH identities, run IDs,
deadlines, and cumulative USD 20 ceiling. Failure of one action never permits a
retry under the same authorization.

The cost receipt includes preparation compute, both launches, root and warm EBS
size/IOPS/throughput, public IPv4, requests, S3/snapshots, and a seven-day warm
volume retention interval. It also reports the continuing daily/monthly storage
rate. Retention beyond seven days requires an explicit extend authorization;
otherwise the operator receives a delete-or-snapshot terminal action naming the
exact volumes. Persistent means surviving compute replacement, not unbounded
unpriced retention.

## Validation

1. Terraform formatting and offline validation prove the attachment graph and
   preserved-volume boundary.
2. Executable shell fixtures reject compilation, package-manager, Git, model
   download, mutable S3, stale seal, wrong AZ/volume, and cleanup deletion paths.
3. Artifact tests build or inspect the real repository binaries and verify the
   seal digest algorithm without paid AWS mutation.
4. Three separately authorized AWS actions prepare the volumes and launch twice
   from the same sealed generation, prove the second warm launch timing and full #605 behavior,
   destroys compute, and retains only the intentional itemized warm volumes.
5. Independent exact-head review passes before publication.

## Rollback

Revert the issue commit and launch through the existing #605 cold qualification
root. Do not delete the last known-good warm volumes during code rollback.
