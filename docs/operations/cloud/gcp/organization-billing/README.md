# GCP organization and billing baseline runbook

Issue #492 turns the #490/#491 GCP decisions into an operational baseline for organization and billing control.

## What this starts

- corporate group ownership on the accepted host project.
- billing export and budget observability for the accepted billing account and host project.
- Cost-attribution labels for #492-managed resources.
- A redacted readback path for project, billing, optional folder, and optional organization-policy checks.

## What this does not start

- Runtime compute.
- GPU resources.
- Private networking.
- Production traffic.
- Static service-account keys.
- #493 platform foundation resources.

## Configuration

Defaults are intentionally pinned to the accepted #490 denominator:

- Organization: `organizations/321515087273`
- Foundation folder: `folders/929563862525`
- Host project: `cs-host-377d41e71a824f92802120`
- Billing account: `billingAccounts/01FA88-CC4968-ADF817`
- Region: `us-west2`
- Corporate owner group: `gcp-admins@agent-logic.ai`

Change these only by updating issue truth and reviewing the plan.

## Validate locally

```sh
bash .csdlc/prepared/issues/492/validate-gcp-c-organization-billing.sh --phase=postbind
terraform -chdir=infra/gcp/organization fmt -check
terraform -chdir=infra/gcp/organization init -backend=false
terraform -chdir=infra/gcp/organization validate
```

## Read-only GCP proof

The static lane requires no credentials:

```sh
bash .csdlc/prepared/issues/492/run-gcp-c-readbacks.sh --lane=static
```

The live read-only lane requires an already-approved GCP context and must not print credential material:

```sh
CLOUDSDK_CORE_PROJECT=cs-host-377d41e71a824f92802120 \
GCP_C_FOLDER_ID=929563862525 \
GCP_C_ORGANIZATION_ID=321515087273 \
bash .csdlc/prepared/issues/492/run-gcp-c-readbacks.sh --lane=inventory-readonly
```

## Apply

Do not apply from this runbook until the exact Terraform plan is reviewed and accepted:

```sh
terraform -chdir=infra/gcp/organization plan -out=tfplan
terraform -chdir=infra/gcp/organization show -no-color tfplan
```

Apply only with command-scoped credentials outside the repository. Do not print, copy, commit, or retain credential material.

## Unchanged POC boundary

Existing POC resources remain unchanged unless a future issue records an explicit reviewed exception. The unchanged POC boundary is part of this issue: #492 only codifies the accepted foundation denominator and host-project billing/ownership guardrails.
