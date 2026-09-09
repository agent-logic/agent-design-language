# Issue #765 XCL Terraform Provider Major Validation

## Scope

- `infra/aws/runtime/xcl-01/versions.tf`
- `infra/aws/runtime/xcl-01/.terraform.lock.hcl`
- `infra/gcp/workloads/xcl-01/versions.tf`
- `infra/gcp/workloads/xcl-01/.terraform.lock.hcl`
- `adl/tools/test_xcl_terraform_provider_major_bounds.sh`

## Provider constraints

- AWS XCL root: `hashicorp/aws` is bounded to `>= 6.0.0, < 7.0.0`; committed lock selection remains `6.62.0`.
- GCP XCL root: `hashicorp/google` is bounded to `>= 8.0.0, < 9.0.0`; committed lock selection remains `8.0.0`.

## Validation performed

- `./adl/tools/test_xcl_terraform_provider_major_bounds.sh --self-test`: passed, including negative fixture rejection for an open-ended provider-major floor.
- `terraform -chdir=infra/aws/runtime/xcl-01 init -backend=false -lockfile=readonly`: passed.
- `terraform -chdir=infra/gcp/workloads/xcl-01 init -backend=false -lockfile=readonly`: passed.
- `terraform -chdir=infra/aws/runtime/xcl-01 fmt -check`: passed.
- `terraform -chdir=infra/gcp/workloads/xcl-01 fmt -check`: passed.
- `terraform -chdir=infra/aws/runtime/xcl-01 validate`: passed.
- `terraform -chdir=infra/gcp/workloads/xcl-01 validate`: passed.
- `terraform -chdir=infra/aws/runtime/xcl-01 plan -refresh=false -input=false ...`: passed with placeholder inputs and approved ADL business AWS profile; plan result was `23 to add, 0 to change, 0 to destroy`; no apply was run.
- `terraform -chdir=infra/gcp/workloads/xcl-01 plan -refresh=false -input=false ...`: passed with placeholder inputs; plan result was `6 to add, 0 to change, 0 to destroy`; no apply was run.

## Notes

- Terraform required normalized lockfile constraint syntax, so both source and lock metadata use the normalized `x.y.z` bound form.
- The AWS plan surfaced the pre-existing `managed_policy_arns` deprecation warning; that is outside issue #765 and was not changed.
- No unrelated Terraform root or provider major was changed.
