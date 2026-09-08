# Issue 663 design: GCP warm two-node Polis

Status: proposed for bounded review.

## Decision

Reuse the existing GCP two-node Runtime/Ollama module and the AWS #607 warm-storage pattern. Use two versioned GCP snapshots as the inexpensive durable authority, restore disposable zonal Persistent Disks for each run, and keep preparation separate from normal launch:

1. An explicit preparation operation creates two staging disks, attaches each to a temporary preparation VM, copies the already-versioned Runtime or Ollama/model bundle from private GCS, verifies the manifest digest, writes one immutable generation marker, stops writers, runs `sync`, unmounts the filesystems, and detaches the disks.
2. A separate snapshot-catalog Terraform root creates and owns the two versioned snapshots from the sealed staging disks. During preparation it also owns temporary verification disks and one verification VM, checks the snapshot identity, generation marker, and content seal, then removes the verifier resources before the generation becomes launchable. Its retained steady state contains only the snapshots.
3. Normal launch restores disposable Runtime and Ollama/model disks from the exact snapshots, creates the Runtime/Guardian and G2/L4 VMs concurrently, attaches the restored disks, verifies the expected generation marker, mounts them, and starts the preinstalled services.
4. Normal launch never installs packages, accesses Git, compiles Rust, runs `ollama pull`, or downloads model data.
5. A controller records the launch request before snapshot restoration and records host-visible timestamps through full readiness. Guest scripts record boot-relative readiness. The final receipt reports both denominators without pretending that local contract tests measured live GCP time.

Exact immutable machine-image self-links provide the base OS, NVIDIA driver, and launch dependencies; image-family aliases are forbidden in the warm root. The snapshots remain the durable Runtime and model-content authority. This avoids keeping provisioned Persistent Disks while idle and avoids rebuilding images when only a Runtime or model generation changes.

## Existing surfaces reused

- `infra/gcp/workloads/modules/two-node-ollama-runtime`: two-node topology, private addressing, service account, OS Login, and G2 scheduling.
- `infra/gcp/workloads/drt-d-six-resident`: qualified GCP provider shape, models, and private Runtime-to-Ollama path.
- `infra/gcp/workloads/xcl-01`: stable retained Runtime disk attachment and mount precedent.
- `infra/aws/runtime/gpu-proof/warm-storage`: separation between preparation, launch, identity verification, and readiness timing.

## Terraform shape

The reusable two-node module receives optional restored disk self-links and stable device names. When supplied, it creates `google_compute_attached_disk` resources. The warm launch root owns the disposable restored disks, creates each from an exact snapshot, and pins the expected artifact generation and manifest digest.

Ownership is split across three small Terraform states so ordinary cleanup cannot cross boundaries:

- `preparation`: temporary hydration VMs and staging disks only;
- `snapshot-catalog`: the two durable versioned snapshots plus temporary restored-content verification resources that are removed before preparation completes;
- `launch`: restored disposable disks, Runtime VM, and GPU VM only.

After the catalog verifies restored content, the preparation root is destroyed. Ordinary launch teardown destroys only launch state. Snapshot retirement is an explicit destroy of the exact snapshot-catalog generation through a command that requires the expected snapshot IDs; no `prevent_destroy` workaround or targeted destroy is used.

## Startup contract

Each guest startup script:

1. locates its disk through `/dev/disk/by-id/google-<device-name>`;
2. verifies the filesystem exists and mounts it by UUID;
3. reads a small generation manifest from the snapshot-restored disk;
4. compares the expected generation and manifest SHA-256 from instance metadata;
5. starts the already-present service through systemd;
6. verifies local health and writes a readiness JSON record using Linux boot-relative time.

Any missing disk, filesystem, manifest, identity, executable, model, or health endpoint fails closed. Runtime remains supervised by Guardian. Ollama listens only on the VM's private interface and firewall exposure remains limited to the Runtime node.

## Timing contract

The primary controller records launch request time before Terraform begins snapshot-to-disk restoration, disk creation completion, Compute Engine `RUNNING` observation for each VM, guest readiness timestamps and boot-relative durations, two-model residency confirmation, and full Polis ready time.

Terraform creates both restored disks and both nodes concurrently where dependencies permit. The final result reports snapshot-launch-to-ready and guest-boot-to-ready separately. A secondary restart measurement may be reported for already-created stopped VMs but cannot satisfy the primary cheap-idle acceptance path. No fixed success deadline kills either service; polling has a configurable observation timeout only for the caller.

## Security and lifecycle

- Existing service-account and OS Login authority is reused.
- No second SSH keypair is introduced.
- Ollama receives no public ingress.
- GCS is used during preparation only, under existing private artifact-read authority.
- Snapshots and restored disks are encrypted by GCP and labeled with issue, role, generation, and retention intent.
- Warm-root boot image inputs must be exact immutable image self-links or IDs; image-family aliases fail validation, and receipts record expected and observed image identities.
- Normal workload teardown deletes the restored disks and cannot delete the snapshots because snapshot ownership is isolated in the snapshot-catalog root.
- Live snapshot launch and readiness measurement require explicit operator approval of the GCP project and budget.

## Validation

Focused Terraform tests assert the three-state ownership boundary, snapshot-to-disk restoration, attachment topology, teardown boundaries, immutable image IDs, private networking, and required metadata. A startup-policy test rejects Git, builds, package installation, and model downloads from normal-start scripts. A sealing test requires writer shutdown, `sync`, unmount/detach before snapshot creation, and restored-content verification. Shell syntax and Terraform formatting/validation run locally. A live snapshot-to-ready measurement remains a separate paid lane and cannot be inferred from local tests.

## Rollback

Remove the optional restored-disk inputs or deploy the already-qualified #509 disposable root. Existing snapshots are left untouched. AWS #607 is unaffected.
