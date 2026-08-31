# #494 GCP-E proof status

Status as of 2026-08-31T01:48:28Z: static implementation proof passes; live
GPU execution is blocked by GCP project-level GPU quota approval.

## Static proof

Commands run from the #494 bound worktree:

```sh
bash .csdlc/prepared/issues/494/validate-gcp-e-gpu-smoke.sh --lane=all
terraform fmt -check infra/gcp/workloads/gpu-smoke
terraform -chdir=infra/gcp/workloads/gpu-smoke validate
git diff --check origin/main...HEAD
```

Result: pass.

## Live target preflight

Project and target:

- project: `cs-poc-cha8mmii0xk0iaw5vpf8mxf`
- region: `us-west1`
- zone: `us-west1-a`
- network/subnet: `default` / `default`
- machine: `g2-standard-4`
- accelerator: one `nvidia-l4`
- image family: `common-cu129-ubuntu-2204-nvidia-580`

Read-only GCP checks proved:

- Compute API enabled.
- `g2-standard-4` exists in `us-west1-a`.
- `nvidia-l4` exists in `us-west1-a`.
- `default` subnet exists in `us-west1`.
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
the proof without creating resources when the global GPU quota is below the
requested accelerator count.

## Quota request

The minimal global GPU quota preference was submitted:

- preference: `projects/cs-poc-cha8mmii0xk0iaw5vpf8mxf/locations/global/quotaPreferences/adl-494-gpus-all-regions-1`
- quota id: `GPUS-ALL-REGIONS-per-project`
- service: `compute.googleapis.com`
- preferred value: `1`
- granted value at creation: `0`
- reconciling: `true`
- trace id: `c7ea9756-7398-4e82-974a-cdc98c7a5b85`

Live #494 GPU proof can resume when `GPUS_ALL_REGIONS` grants at least `1`.
