# XCL-01 cross-cloud Runtime Terraform proof

Issue: #495

Status: static/local proof packet. Live AWS/GCP plan/apply/destroy proof is not claimed and remains gated on explicit operator authorization.

## Implemented surfaces

- `infra/runtime-portable/README.md`
- `infra/runtime-portable/runtime-workload-contract.v1.json`
- `infra/aws/runtime/xcl-01/`
- `infra/gcp/workloads/xcl-01/`
- `docs/milestones/v0.92.1/evidence/cloud/xcl-01/validate-xcl-01-cross-cloud-runtime-terraform.sh`
- `.csdlc/prepared/issues/495/denominator-inventory.md`

## Denominator preservation

- #194 CloudFormation source remains
  `adl/tools/issue194_private_network.cloudformation.json`.
- #268 CloudFormation source remains
  `adl/tools/issue268_runtime_qualification.cloudformation.yaml`.
- CloudFormation rollback authority remains available until #496 accepts
  retirement.
- AWS and GCP implementations expose provider-specific differences rather than
  claiming a single shared resource graph.

## Live proof boundary

No credential material is required for the static validator. This issue runs
only non-apply Terraform checks:

- `terraform init -backend=false -input=false`
- `terraform validate`

No Terraform `plan`, `apply`, or `destroy` is executed by this proof packet.

The future live parity lane must record exact inputs, plan identity, deployment
identity, cleanup selectors, and zero-residue readback for both providers before
claiming live convergence.

## Local validation performed

- `bash docs/milestones/v0.92.1/evidence/cloud/xcl-01/validate-xcl-01-cross-cloud-runtime-terraform.sh --lane=all`
- `terraform fmt -check infra/aws/runtime/xcl-01 infra/gcp/workloads/xcl-01`
- `terraform init -backend=false -input=false` in `infra/aws/runtime/xcl-01`
- `terraform validate` in `infra/aws/runtime/xcl-01`
- `terraform init -backend=false -input=false` in `infra/gcp/workloads/xcl-01`
- `terraform validate` in `infra/gcp/workloads/xcl-01`
