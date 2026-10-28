# AWS-C Terraform state-isolation register

Issue: #486

Purpose: record the pre-publication inventory boundary for the AWS-C Terraform
bootstrap so it does not import, copy, or dual-own existing ADL AWS state.

## Source inventory evidence

- #484 / AWS-A retained inventory: `docs/operations/cloud/aws/inventory/AWS_RESOURCE_OWNERSHIP_INVENTORY.md`
- #122 / public edge Terraform: `infra/aws/csm-public-edge/**`
- Existing DDNS Terraform: `infra/ddns/**`
- #486 bootstrap Terraform: `infra/aws/bootstrap/**`

## Existing surfaces not owned by #486

| Surface | Existing owner/evidence | #486 disposition |
| --- | --- | --- |
| Website and static-origin buckets | #484 inventory lists existing S3 buckets including website/static-origin assets. | Not imported, copied, referenced, or managed by `infra/aws/bootstrap`. |
| DDNS Lambda, Route53 updater, token state | Existing Terraform lives under `infra/ddns/**`. | Not imported, copied, referenced, or managed by `infra/aws/bootstrap`. |
| Public edge / CloudFront / API Gateway / WAF / Route53 aliases | #122 owns `infra/aws/csm-public-edge/**`. | Not imported, copied, referenced, or managed by `infra/aws/bootstrap`. |
| Runtime workload compute, ALB, GPU, or CSM hosts | Owned by later AWS-D/AWS-E/AWS-F/#122-derived workload issues, not AWS-C. | Not imported, copied, referenced, or managed by `infra/aws/bootstrap`. |

## #486-owned Terraform state

`infra/aws/bootstrap` owns only the persistent Terraform backend foundation:

- one encrypted/versioned S3 state bucket and bucket controls;
- one DynamoDB lock table;
- one scoped deployment role;
- one backend-access policy and attachment.

## Isolation checks

- No `terraform import` is part of #486.
- No existing state file is copied into #486.
- No website, DDNS, public-edge, Runtime, or workload resource is declared under
  `infra/aws/bootstrap`.
- Future Terraform roots consume the backend through the exported
  `backend_hcl` values after operator review; they do not become owned by #486.

