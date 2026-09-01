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

Both guest roles can read the issue artifact prefix, but their write authority
is narrower: the GPU role can create only its exact run-specific ready receipt,
and the Runtime role can create only its exact run-specific final receipt.
Neither guest can write model/runtime artifacts, source/config objects, locks,
or authorization markers.

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
bootstrap scripts in the authorized S3 prefix. Terraform injects separate
automatic cloud-init payloads:

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
