# AWS two-node Runtime and GPU proof

Issue #345 owns a bounded AWS portability proof. It is not a production
inference fallback or 24/7 service definition. The proof uses one regular
On-Demand Runtime node and one On-Demand GPU node:

- Runtime node: Guardian, Runtime, six resident agents, UTS, ACC, and the
  Freedom Gate.
- GPU node: one Ollama process with every configured model simultaneously
  GPU-resident; at least two immutable models are required.

The Runtime calls Ollama through the GPU node's private IPv4 address. TCP/11434
is admitted only from the Runtime security group and is never public. Both
nodes always receive public IPv4 addresses and share exactly one
Terraform-managed EC2 key pair. Each node admits TCP/22 only from the required
operator IPv4 `/32`. `AmazonSSMManagedInstanceCore` is attached to both node
roles, and cloud-init enables the packaged SSM agent on both nodes as a recovery
channel, but bootstrap never uses controller-side SSM commands.

## Infrastructure ownership

The issue-local Terraform root is `infra/aws/runtime/gpu-proof`. A single
plan/apply/destroy lifecycle owns both instances, both security groups, the
shared key pair, separate instance roles and profiles, encrypted gp3 root
volumes, and the one-time EventBridge Scheduler termination action. The
scheduler targets both exact instance IDs, has no flexible window, and retains
only tag-constrained `ec2:TerminateInstances` authority. There is no Lambda,
Python reaper, Spot fallback, controller-side `run-instances`, or SSM
SendCommand path.

Both guest roles can read only the exact immutable artifact and run-object keys
needed for bootstrap, including the Runtime node's exact GPU-ready receipt key.
Their write authority is narrower: the GPU role can create only its exact
run-specific ready receipt, and the Runtime role can create only its exact
run-specific final receipt. Neither guest can write model/runtime artifacts,
source/config objects, locks, or authorization markers.

Terraform uses explicit account, region, VPC, subnet, AMIs, instance types,
disk sizes, SSH inputs, owner token, deadline, artifact prefix, and cost ceiling
variables. It performs no infrastructure discovery or pricing lookup. The
runner's read-only preflight resolves and authorizes those inputs before apply.

All Terraform state, plans, variables, generated bootstrap files, and receipts
remain below the checkout-local `.adl/local/issue345` directory. The root has no
remote backend. Do not move state to `/tmp` or the Git common directory.

## Required operator inputs

Set these before preflight or execution:

- `ADL_ISSUE345_SSH_INGRESS_CIDR`: the operator's public IPv4 address as an
  exact `/32`; `0.0.0.0/0` is rejected.
- `ADL_ISSUE345_SSH_PUBLIC_KEY_FILE`: an existing OpenSSH public key file.
  Only the public key enters Terraform; private key material must never enter
  configuration or state.
- `ADL_ISSUE345_VPC_ID`: the VPC containing the selected subnet.

The immutable S3 manifest defaults identify the retained two-model bundle. Any
override must retain a version ID and SHA-256. `models` must be a unique set of
at least two identities, with one version-pinned model-store archive per model,
one Ollama runtime, and one rustup installer.

## Read-only preflight

```bash
bash adl/tools/run_issue345_aws_gpu_shepherd_proof.sh preflight
```

Preflight verifies the `agent-logic-admin` business account, Terraform
configuration, required SSH `/32` and public-key hash, immutable model bundle,
pre-resolved Runtime and GPU AMIs, a GPU-capable subnet in the explicit VPC,
an active internet-gateway default route, a permissive first IPv4 network-ACL
rule in both directions, GPU quota, both On-Demand prices, both gp3 volumes,
two public IPv4 charges, request overhead, the total ceiling, and zero stale
issue-tagged instances or volumes. It performs no paid mutation.

## Retained paid authorization

Execution requires `adl.issue345.paid_run_authorization.v3`. It binds:

- exact source commit and typed exact-head review revision;
- unique run ID and expiry;
- Agent Logic account hash, region, VPC, subnet, and both resolved AMI hashes;
- the selected subnet's effective route-table and network-ACL fingerprints;
- Runtime and GPU instance types plus individual and combined hourly ceilings;
- full two-node billable deadline, both disk costs, two public IPv4 costs,
  request overhead, and total ceiling no greater than USD 20;
- the exact SSH `/32` and normalized public-key SHA-256;
- immutable manifest coordinates/digest and the complete model set.
- the exact reviewed repository commit, restored from a run-specific versioned
  S3 archive rather than a live Git checkout on every node start.
- the canonical tracked typed issue, review, authored-design, and diagram state
  present when the single-use authorization is created.

The runner derives a canonical JSON digest and writes a create-only versioned
S3 consumption marker. Formatting or object-key order cannot create a second
authorization identity, and cleanup never deletes the marker. The exact typed
review must still be current, and changes to the runner, focused test,
Terraform root, or this runbook after review are rejected.

```bash
bash adl/tools/run_issue345_aws_gpu_shepherd_proof.sh run \
  --commit EXACT_REVIEWED_SHA \
  --run-id adl-issue345-YYYYMMDD-HHMMSS \
  --authorization-file .adl/local/issue345/operator-authorization.json \
  --execute
```

The run attempts one Terraform apply and has no retry, Spot, or alternate
instance fallback.

## Automatic proof flow

Before apply, the runner stores versioned configuration and two digest-bound
bootstrap scripts in the authorized S3 prefix. Large run artifacts use the AWS
CLI multipart transfer path and then bind the bucket's exact returned version.
The exact source archive contains only the tracked `adl`, `adl-runtime`,
`adl-runtime-kernel`, `adl-resilience`, and `adl-spec` build/proof trees plus
the small API, parity-matrix, and stock-league files embedded at compile time,
and the 12 KiB `infra/runtime-v3` lifecycle-validator input directory. This is
the full local Rust dependency, compile-input, and Guardian validation-input
closure required by the Runtime proof while avoiding unrelated repository media
and historical evidence. The Runtime bootstrap passes
the authorization-bound source revision to validators because an archive does
not contain mutable Git metadata. Lifecycle qualification recognizes that
Git-free root only when the generated init template remains under
`.adl/runtime-v3` and all required ADL component and Runtime-init markers are
present; lock and residue paths remain contained under that archive root.
Terraform injects separate automatic cloud-init payloads:

1. The GPU node downloads exact object versions, starts Ollama as a persistent
   systemd service, requests every configured model with infinite keep-alive,
   requires the complete set to have nonzero VRAM residency, and uploads a GPU
   readiness receipt.
2. The Runtime node waits for that S3 receipt and the private Ollama endpoint,
   restores the exact reviewed repository archive from versioned S3, proves
   Guardian-supervised Runtime v3 lifecycle, exposes the private GPU endpoint
   only through a loopback-bound proxy required by the local Shepherd contract,
   runs the governed Shepherd adapter once per model, and executes six real
   Runtime-agent proposals using the task panel explicitly rooted in that
   restored repository through UTS, ACC, the Freedom Gate, and
   `runtime.observe`. It uploads a final success or failure receipt.
3. The controller polls only the versioned S3 receipt surface. It does not send
   shell commands through SSM.

The final evidence keeps the Guardian/Runtime-v3 lifecycle and governed
Runtime-agent-to-remote-Ollama paths distinct. Runtime v3 still has no Ollama
provider ingress, so the receipt explicitly records
`runtime_v3_to_ollama_transit_proved: false`; co-location or topology is never
reported as nonexistent transitive integration.

## Cleanup and deadline

The exit trap always runs Terraform destroy after an apply attempt, then uses
read-only EC2 queries to require zero issue/run instances and volumes before it
releases the owner-bound S3 lock. Both root volumes are encrypted and
delete-on-termination. Independently, the one-time EventBridge Scheduler action
terminates both exact node IDs at the authorized deadline and retries only
within its bounded five-minute event age.

Immediately after lock acquisition, the runner writes a mode-0600
`recovery.json` inside the run's worktree-local state directory. It retains the
raw owner token and exact lock version needed for manual cleanup after a
controller crash; this private recovery file is never uploaded or included in
public evidence.

Manual retry uses the original local state and owner identity:

```bash
bash adl/tools/run_issue345_aws_gpu_shepherd_proof.sh cleanup \
  --run-id adl-issue345-YYYYMMDD-HHMMSS \
  --owner-token OWNER_TOKEN \
  --lock-version-id LOCK_VERSION
```

## Focused local contract

```bash
bash adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
```

This validates Terraform and executable fail-closed contracts without fake AWS
responses and without a paid launch. It asserts two Terraform-owned nodes, one
mandatory shared managed key pair, public SSH `/32` on both nodes, private
SG-only Ollama, SSM recovery without SSM bootstrap, both encrypted disposable
volumes, exact-receipt-only guest write authority, public-subnet proof, both-node
deadline targeting, private-IP injection, loopback Shepherd forwarding,
explicit six-agent task-panel rooting, mode-0600 recovery state, single-use
authorization, exact review equality, and zero controller-owned launch
commands.

## Issue #607 warm two-node qualification

Issue #607 retains the #605 network and service topology but removes builds,
package installation, and model downloads from normal launch. It uses three
disjoint local Terraform states:

1. `infra/aws/runtime/gpu-proof/warm-storage` owns only the two encrypted,
   retained, AZ-bound gp3 data volumes.
2. `infra/aws/runtime/gpu-proof/warm-storage/preparation` owns the two
   short-lived hydrator instances and their disposable infrastructure. It
   builds the repository binaries, installs package-managed build facilities,
   copies the exact versioned Ollama/model closure, prepares reusable Runtime
   state, writes every volume block, and seals each content partition with
   dm-verity. It does not survive preparation.
3. `infra/aws/runtime/gpu-proof` owns the two normal-launch instances,
   security/IAM/deadline resources, and warm-volume attachments. It never owns
   or deletes the retained volumes.

All generated state remains under `.adl/local/issue607` in the bound issue
worktree. Use only `agent-logic-admin` in `us-west-2`, one existing SSH public
key, and one exact public IPv4 `/32`.

### Read-only preflight

```bash
ADL_ISSUE607_SSH_PUBLIC_KEY_FILE=/absolute/path/to/public-key.pub \
AWS_PROFILE=agent-logic-admin AWS_REGION=us-west-2 \
bash adl/tools/run_issue607_warm_polis.sh preflight
```

Preflight resolves and records exact AMI metadata, account/network/KMS and SSH
identity hashes, current On-Demand prices, two retained EBS performance
profiles, disposable root EBS, two public IPv4 addresses, S3/request allowance,
two retained sparse data snapshots plus two prepared-image root snapshots with
a conservative 260 GiB changed-block allowance, and
seven days of retained storage. It fails when the aggregate
estimate exceeds USD 20 and launches nothing.

### Prepare once

Run preparation first without an authorization file. The controller builds a
local source archive, creates and validates the exact storage saved plan, writes
`authorization-request.json`, and exits before S3 upload or AWS mutation:

```bash
bash adl/tools/run_issue607_warm_polis.sh prepare \
  --commit EXACT_REVIEWED_SHA \
  --run-id adl-issue607-prepare-UNIQUE \
  --storage-id adl-issue607-warm-v1 \
  --execute
```

The operator-authorized `adl.issue607.authorization.v3` file must copy the
request's exact action, commit, run, storage, saved-plan, preflight, action
manifest, and campaign fields; add a unique `action_id`, future
`expires_at`, `authorized: true`, and `single_use: true`. Execute the same
command with `--authorization-file`. The authorization is consumed through a
create-only S3 marker immediately before the first mutation.

Successful preparation leaves two sealed 200 GiB volumes, completed immutable
snapshots of both volumes, two prepared launch AMIs and their root snapshots,
their storage state, and the exact AMI/facility/seal
receipts, a GPU snapshot-to-volume availability timing receipt, and an aggregate
cost ledger. Unused volume extents are not zero-filled, preserving sparse
snapshot economics. The temporary restored timing volume is deleted. The
preparation instances, ENIs, security group, IAM resources, scheduler, shared
key pair, and root volumes must be absent according to both Terraform and live
tag inventory.

The Ubuntu 24.04 preparation guests install the officially supported AWS CLI v2
Snap package because that release does not provide the `awscli` APT package.
This package-manager work occurs only during one-time preparation; normal warm
launch performs no package installation. The controller also fails preparation
after three stopped-instance observations when neither a success nor failure
receipt exists, avoiding a full timeout after an early cloud-init failure.

Before image capture, both preparation guests clear cloud-init state and logs,
reset machine identity, and remove SSH host keys so first launch regenerates
per-instance state. Raw EC2 provider IDs are written immediately to
`preparation-resources.json`, including the first data snapshot before the
second snapshot request begins. The exit trap is active before warm-storage
apply. If preparation is interrupted before its result is durable, it removes
the incomplete warm-storage state as well as disposable preparation and raw
resources. Rerun exact cleanup from that worktree and run ID; it does not
require a completed preparation result and also discovers tagged resources
that were created immediately before a local ledger write:

```bash
bash adl/tools/run_issue607_warm_polis.sh recover-preparation \
  --commit EXACT_REVIEWED_SHA \
  --run-id adl-issue607-prepare-UNIQUE \
  --storage-id adl-issue607-warm-v1 --execute
```

### Launch twice

Use a unique run ID and distinct single-use authorization for each ordinal:

```bash
bash adl/tools/run_issue607_warm_polis.sh launch \
  --commit EXACT_REVIEWED_SHA \
  --run-id adl-issue607-launch-1-UNIQUE \
  --storage-id adl-issue607-warm-v1 \
  --ordinal 1 --execute
```

As with preparation, the first invocation emits an exact authorization request
and performs no mutation. Add `--authorization-file` to the identical command
only after authorization. Repeat with ordinal `2` and another unique run ID and
authorization.

Each guest boots the exact prepared launch AMI and verifies its facility inventory, volume ID,
generation, manifest, and dm-verity root. GPU readiness requires all configured
models resident with nonzero VRAM. Runtime readiness requires the persistent
Guardian process to pass authenticated HTTPS and WSS probes. Each guest must
reach local readiness in 30 seconds; controller apply-to-observed readiness
must remain within 120 seconds. The later qualification receipt separately
requires both Shepherd model proofs, six governed Runtime-agent ACC executions,
and restart/state/degradation/Vector/log/shutdown proof. Compute is then
destroyed and live tag inventory must show zero disposable residue while the
two warm volumes remain `available`.

### Retention decision

Inspect the exact volumes and seven-day deadline without mutation:

```bash
bash adl/tools/run_issue607_warm_polis.sh retention-status \
  --storage-id adl-issue607-warm-v1
```

Extending retention or deleting the generation requires a separate
`adl.issue607.storage_authorization.v2` that binds the controller-emitted exact
saved plan, both volume IDs, both prepared AMIs, and all four retained snapshot
IDs. None of these paths is reachable from compute cleanup:

```bash
bash adl/tools/run_issue607_warm_polis.sh extend-retention \
  --storage-id adl-issue607-warm-v1 \
  --retention-until 2026-09-15T00:00:00Z --execute

bash adl/tools/run_issue607_warm_polis.sh retire-storage \
  --storage-id adl-issue607-warm-v1 --execute

bash adl/tools/run_issue607_warm_polis.sh retire-snapshots \
  --storage-id adl-issue607-warm-v1 --execute
```

Run either command once without `--authorization-file` to obtain its exact
request, then repeat it with the separately approved file. Retirement accepts
only a saved plan that deletes exactly the two retained EBS volumes and no
other resource. Snapshot retirement separately deregisters the exact two
prepared images and deletes their root snapshots plus the two sealed-data
snapshots. Both recovery and retirement treat only an explicit AWS not-found
result as absence; API or transport errors fail the action, and terminal success
is emitted only after exact-ID absence readback. The preparation result stores
both root snapshot IDs, so an interrupted snapshot retirement can resume with
the identical already-consumed authorization and manifest even after an AMI is
gone; a different authorization is rejected.
