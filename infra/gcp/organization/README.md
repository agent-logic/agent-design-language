# GCP-C organization and billing baseline

Issue #492 owns this Terraform root. It codifies the accepted GCP organization, foundation folder, host project, billing account, corporate ownership group, labels, budget guardrail, and billing-export dataset target from #490/#491.

This root does not create runtime compute, GPU resources, private networking, production traffic, static service-account keys, or #493 platform resources.

## Accepted denominator

- Organization: `organizations/321515087273`
- Foundation folder: `folders/929563862525`
- Host project: `cs-host-377d41e71a824f92802120`
- Billing account: `billingAccounts/01FA88-CC4968-ADF817`
- Primary region: `us-west2`
- Corporate owner group: `gcp-admins@agent-logic.ai`
- Billing export dataset: `adl_gcp_c_billing_export`

## Ownership and billing controls

- `group:gcp-admins@agent-logic.ai` receives `roles/owner` on the host project, plus viewer/security-reviewer roles for explicit auditability.
- The host-project budget guardrail is scoped to the accepted billing account and host-project number.
- The billing-export BigQuery dataset is labeled with the required #492 cost-attribution labels.

## Local validation

```sh
terraform -chdir=infra/gcp/organization fmt -check
terraform -chdir=infra/gcp/organization init -backend=false
terraform -chdir=infra/gcp/organization validate
bash .csdlc/prepared/issues/492/validate-gcp-c-organization-billing.sh --phase=postbind
```

## Apply boundary

Only run `terraform apply` after the operator confirms the company GCP account context and the exact plan. Keep credentials outside the repository and pass them as command-scoped environment only.
