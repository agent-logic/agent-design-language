# AWS-F Runtime platform proof packet

Issue: #489, corrected by #579

## Proof class

Static, non-mutating, local validation.

## Result

- `cloud_mutation=false`
- `production_traffic=false`
- `credential_material_retained=false`
- `runtime_hosts_direct_public_ingress=false` for the committed defaults
- `public_edge_owner=#122`
- `aws_f_route53_or_acm_resources=false`
- `live_disposable_deployment_proof=false`
- `adoption_register_owner=#488`

## Checked surfaces

- `infra/aws/runtime/README.md`
- `infra/aws/runtime/alb-origin/**`
- `infra/aws/runtime/private-node/**`
- `infra/aws/runtime/modules/private-runtime-node/**`
- `infra/aws/modules/csm-runtime-alb/**` as consumed ALB module
- `docs/operations/cloud/aws/runtime-platform/README.md`
- `.csdlc/prepared/issues/579/validate-aws-f-corrective.sh`

## Local commands

```bash
bash .csdlc/prepared/issues/579/validate-aws-f-corrective.sh --lane=all
git diff --check
```

The corrective validator checks Terraform-owned paths, rejects AWS-F Route53 or
ACM resource ownership, rejects direct public Runtime ingress in committed
ingress rules/examples, requires S3 backend declarations plus committed backend
config examples with distinct state keys, locking, and encryption, checks
Terraform account/workspace guardrails, and rejects the old public-node shortcut by requiring
`associate_public_ip_address = false` inside the issue-owned private Runtime
node module.

## Live proof gate

The live disposable deployment/path/cleanup proof is intentionally separate
from this static packet. It requires explicit operator authorization for AWS
resource mutation and must record the saved plan, target health, external path
receipt, and zero residue readbacks described in the runbook.
