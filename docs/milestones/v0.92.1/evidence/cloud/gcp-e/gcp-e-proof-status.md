# #494 GCP-E proof status

Status as of 2026-08-31T03:20:00Z: static implementation proof passes and
the accepted live GCP-E L4 proof completed in `us-central1-a`. Runs r8 and r9
are the accepted live readback/cleanup evidence. Runs r10 and r11 are retained
as rejected hardening attempts that proved `gcloud compute ssh --plain` is not
the correct route for this proof.

## Static proof

Commands run from the #494 bound worktree:

```sh
bash .csdlc/prepared/issues/494/validate-gcp-e-gpu-smoke.sh --lane=all
terraform fmt -check -recursive infra/gcp/workloads/modules/gpu-smoke-support infra/gcp/workloads/modules/gpu-smoke-instance infra/gcp/workloads/gpu-smoke-support infra/gcp/workloads/gpu-smoke-instance
TMPDIR="${PWD}/.t/" terraform -chdir=infra/gcp/workloads/gpu-smoke-support validate
TMPDIR="${PWD}/.t/" terraform -chdir=infra/gcp/workloads/gpu-smoke-instance validate
git diff --check origin/main...HEAD
```

Result: pass.

## Live target preflight

Project and target:

- project: `cs-poc-cha8mmii0xk0iaw5vpf8mxf`
- region: `us-central1`
- zone: `us-central1-a`
- network/subnet: `default` / `default`
- machine: `g2-standard-4`
- accelerator: one `nvidia-l4`
- image family: `common-cu129-ubuntu-2204-nvidia-580`

Read-only GCP checks proved:

- Compute API enabled.
- `g2-standard-4` exists in `us-central1-a`.
- `nvidia-l4` exists in `us-central1-a`.
- `default` subnet exists in `us-central1`.
- No #494 instances, disks, snapshots, forwarding rules, static addresses, or
  #494 firewall residue remained after failed attempts.

## Live execution attempts

Two bounded live attempts were made with
`GOOGLE_APPLICATION_CREDENTIALS=/Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json`
and `CLOUDSDK_CONFIG` under Git-common. The credential file contents were not
read, copied, printed, or committed.

Attempt 1 failed before VM creation because the configured image family
`common-cu121` did not exist. Terraform destroyed the transient firewall and
service account.

Attempt 2 used `common-cu129-ubuntu-2204-nvidia-580` and failed before VM
creation on the GCP quota gate:

```text
Quota 'GPUS_ALL_REGIONS' exceeded. Limit: 0.0 globally.
metric name = compute.googleapis.com/gpus_all_regions
limit name = GPUS-ALL-REGIONS-per-project
```

Terraform destroyed the transient firewall and service account.

The run script now checks `GPUS_ALL_REGIONS` before Terraform apply and refuses
the proof without creating VM resources when the global GPU quota is below the
requested accelerator count.

Attempt 3 ran after the quota preference was approved to `1`. Terraform created
the service account, IAP firewall, and L4 VM, then SSH readback failed because
`gcloud compute ssh` tried to create its default key under `/Users/daniel/.ssh`.
The destroy trap removed the VM, firewall, and service account. The script now
routes `gcloud compute ssh` through `GCP_E_SSH_KEY_FILE`, defaulting to a
Git-common private path, and probes the startup marker before log readback so
IAP/OS Login propagation has a bounded retry window.

The operator corrected the desired repeated-run behavior: #494 should not
recreate stable support resources every time. The Terraform layout now mirrors
the AWS split with separate support and instance modules/roots. The stable
support root owns the service account and IAP firewall (`support_id`, default
`adl-494-gpu-smoke`), while the instance root creates and destroys only its
run-id VM. The post-run cleanup proof checks for no remaining per-run VM/disk
resources and retains readback evidence for the stable service account/firewall.
The runner imports existing stable service account/firewall resources before
support apply, so support reuse does not depend on a retained untracked local
Terraform state file.

`us-west1-a`, `us-west1-b`, and `us-west1-c` were checked after quota approval;
`a` and `b` returned stockout for `g2-standard-4 + 1x nvidia-l4`, and `c`
reported the shape unsupported. The default proof target was therefore moved to
the first proven quota-valid zone, `us-central1-a`.

Attempt 4 in `us-central1-a` completed the live GPU proof: stable support was a
no-op, the disposable VM was created, IAP SSH read back `NVIDIA L4,
580.173.02`, and Terraform destroyed exactly one VM.

Follow-up script hardening tried `gcloud compute ssh --plain` with explicit
Git-common key and known-hosts paths; that was rejected as the final route
because `--plain` disables the normal OS Login/key propagation that made the
successful proof work and produced `Permission denied (publickey)` during the
readiness probe. The final script keeps normal `gcloud compute ssh` behavior
while supplying explicit Git-common SSH key and run-scoped known-hosts paths.

## Quota request

The minimal global GPU quota preference was submitted:

- preference: `projects/cs-poc-cha8mmii0xk0iaw5vpf8mxf/locations/global/quotaPreferences/adl-494-gpus-all-regions-1`
- quota id: `GPUS-ALL-REGIONS-per-project`
- service: `compute.googleapis.com`
- preferred value: `1`
- granted value at creation: `0`
- reconciling: `true`
- trace id: `c7ea9756-7398-4e82-974a-cdc98c7a5b85`

No additional live GPU proof is required for the current #494 publication
packet. Future reruns can reuse the same stable support resources and create
only a new run-specific VM.
