# GCP-C organization and billing proof

Issue: #492

## Local proof

- `bash .csdlc/prepared/issues/492/validate-gcp-c-organization-billing.sh --phase=postbind`
- `bash .csdlc/prepared/issues/492/run-gcp-c-readbacks.sh --lane=static`
- `terraform -chdir=infra/gcp/organization fmt -check`
- `terraform -chdir=infra/gcp/organization init -backend=false`
- `terraform -chdir=infra/gcp/organization validate`

## Static readback posture

- `cloud_mutation=false`
- `credential_material_retained=false`
- `redaction=credential_material_not_printed`

## Live readback posture

Live readback is read-only and uses the operator-approved GCP context. It must not print names, ids, token values, service-account keys, private keys, client secrets, refresh tokens, or credential file contents.

Recorded read-only result:

- `gcp_c_readback_lane=inventory-readonly`
- `project_describe_readable=true`
- `billing_project_readable=true`
- `corporate_owner_project_role_readable=not_applied_or_not_authorized` before apply; the Terraform root now grants `roles/owner` to `group:gcp-admins@agent-logic.ai`.
- `budget_readable=not_applied_or_not_authorized` before apply; the Terraform root defines the host-project budget guardrail.
- `billing_export_dataset_readable=not_applied_or_not_authorized` before apply; the Terraform root defines the labeled billing-export dataset target.
- `folder_describe_readable=not_configured`
- `organization_policy_readable=not_configured`
- `cloud_mutation=false`
- `credential_material_retained=false`
- `redaction=names_ids_and_credential_material_not_printed`

The command-scoped service-account key proved host-project and billing-project read access. Optional folder and organization-policy readbacks require additional IAM if they are selected by setting `GCP_C_FOLDER_ID` or `GCP_C_ORGANIZATION_ID`; this issue does not grant new organization or folder roles.

## Scope proof

- Organization denominator: `321515087273`
- Foundation folder denominator: `929563862525`
- Host project denominator: `cs-host-377d41e71a824f92802120`
- Billing account denominator: `01FA88-CC4968-ADF817`
- Corporate group ownership: `group:gcp-admins@agent-logic.ai` receives `roles/owner` on the accepted host project in Terraform.
- Budget/export observability: host-project billing budget plus labeled BigQuery billing-export dataset target are declared in Terraform; the readback script can verify them after apply/authorization.
- Cost attribution: #492 labels are declared in Terraform variables and attached to the billing-export dataset.
- Existing POC resources: unchanged POC boundary is explicit; this issue does not mutate POC resources

## Non-claims

This proof does not claim runtime deployment, GPU launch, production activation, private platform foundation, static service-account-key creation, or credential custody.
