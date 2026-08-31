# #494 GCP-E GPU readiness smoke

This packet owns one bounded GCP L4 readiness decision. It creates a single
disposable On-Demand L4 VM, captures exact hardware/runtime evidence, destroys
the resources, and independently proves zero-resource cleanup.

It does not own six-resident qualification, production deployment, public
ingress, DNS, Observatory, Unity, AWS work, or persistent GPU infrastructure.

## One-command proof

From the repository root:

```sh
GOOGLE_APPLICATION_CREDENTIALS=/Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json \
  docs/milestones/v0.92.1/evidence/cloud/gcp-e/run-gcp-e-l4-smoke.sh
```

The credential file path is operator-owned and must not be copied, printed, or
committed. The script records only command results and redacted resource
identity.

## Required proof

- paid authorization: USD 20 maximum, On-Demand only;
- exact inputs: project, region, zone, machine type, L4 accelerator, image,
  service account, labels, deadline, and model label;
- GPU readback: `nvidia-smi`/driver/memory plus host memory/disk telemetry;
- inference decision: either a local model runtime proof if already available
  on the image, or a fail-closed reason retained in the proof packet;
- cleanup: `terraform destroy` plus independent zero-resource checks for the
  #494 resource selector.
- quota guard: read-only `GPUS_ALL_REGIONS` quota check before Terraform apply,
  so zero-GPU projects fail without creating resources.

## Current static readiness

The static implementation is ready when:

```sh
bash .csdlc/prepared/issues/494/validate-gcp-e-gpu-smoke.sh --lane=all
terraform fmt -check infra/gcp/workloads/gpu-smoke
terraform -chdir=infra/gcp/workloads/gpu-smoke validate
```

passes after backend-disabled provider initialization.

Default live target: `cs-poc-cha8mmii0xk0iaw5vpf8mxf`, `us-west1-a`,
default VPC/subnet, `g2-standard-4`, and one `nvidia-l4`. Override with
`GCP_E_*` environment variables when the company project topology changes.
