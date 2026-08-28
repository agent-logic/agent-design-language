# GCP-C organization and billing baseline

Issue #492 owns this Terraform root. It codifies the accepted GCP organization, foundation folder, host project, billing account, corporate ownership group, labels, and budget/export observability contract from #490/#491.

This root does not create runtime compute, GPU resources, private networking, production traffic, static service-account keys, or #493 platform resources.

## Accepted denominator

- Organization: `organizations/321515087273`
- Foundation folder: `folders/929563862525`
- Host project: `cs-host-377d41e71a824f92802120`
- Billing account: `billingAccounts/01FA88-CC4968-ADF817`
- Primary region: `us-west2`

## Local validation

```sh
terraform -chdir=infra/gcp/organization fmt -check
terraform -chdir=infra/gcp/organization init -backend=false
terraform -chdir=infra/gcp/organization validate
bash .csdlc/prepared/issues/492/validate-gcp-c-organization-billing.sh --phase=postbind
```

## Apply boundary

Only run `terraform apply` after the operator confirms the company GCP account context and the exact plan. Keep credentials outside the repository and pass them as command-scoped environment only.
