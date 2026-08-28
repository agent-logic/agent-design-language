# AWS-F Runtime platform proof packet

Issue: #489

## Proof class

Static, non-mutating, local validation.

## Result

- `cloud_mutation=false`
- `production_traffic=false`
- `credential_material_retained=false`
- `runtime_hosts_direct_public_ingress=false` for the committed defaults
- `public_edge_owner=#122`
- `adoption_register_owner=#488`

## Checked surfaces

- `infra/aws/runtime/README.md`
- `infra/aws/runtime/alb-origin/**`
- `infra/aws/runtime/private-node/**`
- `infra/aws/runtime/modules/private-runtime-node/**`
- `infra/aws/modules/csm-runtime-alb/**` as consumed ALB module
- `docs/operations/cloud/aws/runtime-platform/README.md`
- `.csdlc/prepared/issues/489/validate-aws-f-runtime-platform.sh`
- `.csdlc/prepared/issues/489/run-aws-f-readbacks.sh`

## Local commands

```bash
bash .csdlc/prepared/issues/489/validate-aws-f-runtime-platform.sh --phase=postbind
bash .csdlc/prepared/issues/489/run-aws-f-readbacks.sh --lane=static
git diff --check
```

The validator checks Terraform-owned paths and rejects the old public-node
shortcut by requiring `associate_public_ip_address = false` inside the
issue-owned private Runtime node module.

## Live proof gate

The live disposable deployment/path/cleanup proof is intentionally separate
from this static packet. It requires explicit operator authorization for AWS
resource mutation and must record the saved plan, target health, external path
receipt, and zero residue readbacks described in the runbook.
