# #494 GCP-E GPU readiness smoke

This packet owns one bounded GCP L4 readiness decision. It maintains stable
#494 support resources, creates a single disposable On-Demand L4 VM, captures
exact hardware/runtime evidence, destroys only the per-run VM, and independently
proves per-run VM/disk cleanup.

It does not own six-resident qualification, production deployment, public
ingress, DNS, Observatory, Unity, AWS work, or persistent GPU infrastructure.

## One-command proof

From the repository root:

```sh
GOOGLE_APPLICATION_CREDENTIALS=/path/to/operator-approved-gcp-service-account.json \
  docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh
```

The credential file path is operator-owned, host-local, and must not be copied,
printed, or committed. The script records only command results and redacted
resource identity.

## Terraform layout

The proof follows the same split as the AWS runtime work:

- `infra/gcp/workloads/modules/gpu-smoke-support`: stable service account and
  IAP SSH firewall module.
- `infra/gcp/workloads/modules/gpu-smoke-instance`: disposable L4 VM module.
- `infra/gcp/workloads/gpu-smoke-support`: support root; apply when support is
  missing or intentionally rotated. The runner imports existing service account
  and firewall resources before apply, so a fresh worktree can reuse stable
  support without relying on untracked local Terraform state.
- `infra/gcp/workloads/gpu-smoke-instance`: per-run instance root; apply and
  destroy on every smoke run.

## Required proof

- paid authorization: USD 20 maximum, On-Demand only;
- exact inputs: project, region, zone, machine type, L4 accelerator, image,
  service account, labels, deadline, and model label;
- GPU readback: `nvidia-smi`/driver/memory plus host memory/disk telemetry;
- inference decision: either a local model runtime proof if already available
  on the image, or a fail-closed reason retained in the proof packet;
- cleanup: targeted `terraform destroy` for the per-run VM plus independent
  per-run VM/disk cleanup checks for the #494 run selector. Stable support
  resources remain so repeated runs recreate only the instance.
- quota guard: read-only `GPUS_ALL_REGIONS` quota check before Terraform apply,
  so zero-GPU projects fail without creating resources.

## Current static readiness

The static implementation is ready when:

```sh
bash .csdlc/prepared/issues/494/validate-gcp-e-gpu-smoke.sh --lane=all
terraform fmt -check -recursive infra/gcp/workloads/modules/gpu-smoke-support infra/gcp/workloads/modules/gpu-smoke-instance infra/gcp/workloads/gpu-smoke-support infra/gcp/workloads/gpu-smoke-instance
TMPDIR="${PWD}/.t/" terraform -chdir=infra/gcp/workloads/gpu-smoke-support validate
TMPDIR="${PWD}/.t/" terraform -chdir=infra/gcp/workloads/gpu-smoke-instance validate
```

passes after backend-disabled provider initialization.

Default live target: `cs-poc-cha8mmii0xk0iaw5vpf8mxf`, `us-central1-a`,
default VPC/subnet, `g2-standard-4`, and one `nvidia-l4`. Override with
`GCP_E_*` environment variables when the company project topology changes.

The reusable support identity defaults to `adl-494-gpu-smoke`; override with
`GCP_E_SUPPORT_ID` only when intentionally rotating the service account,
firewall rule, or network tag. The SSH key and per-run SSH known-hosts file
default to Git-common private paths and may be rotated with
`GCP_E_SSH_KEY_FILE` and `GCP_E_SSH_KNOWN_HOSTS_FILE`. The script waits for the
startup readiness marker over IAP SSH before reading the GPU log, so transient
IAP/OS Login propagation does not turn a healthy VM into a false failure. SSH
keeps normal `gcloud compute ssh` OS Login/key propagation and supplies explicit
key plus run-scoped known-hosts paths so local proof does not depend on or write
to `~/.ssh`.
