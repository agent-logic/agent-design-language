# GCP execution identity plan for #491+

Issue: #490

Status: recommendation only. This file does not authorize #490 to mutate GCP.

## Current identity classification

The refreshed user login `daniel@agent-logic.ai` is sufficient for #490 read-only discovery. Retained IAM readbacks show broad human roles, including:

- Organization: `roles/resourcemanager.organizationAdmin`
- Folders `726824330959` and `929563862525`: `roles/resourcemanager.folderAdmin` and `roles/resourcemanager.folderEditor`
- Several projects, including the accepted host project: `roles/owner`

This proves the user can observe and may have broad power, but it is not the recommended repeatable Terraform/apply identity. #491+ should avoid relying on ambient human ADC as the execution principal.

## Recommended auth model

Use a company-controlled Terraform service account in the accepted long-term host project:

- Project: `cs-host-377d41e71a824f92802120`
- Suggested identity: `tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com`
- Local operator path: service-account impersonation from an approved human account with `roles/iam.serviceAccountTokenCreator` on that service account.
- CI/non-human path: Workload Identity Federation into the same service account.
- Key path: not recommended by default. Use only as break glass with explicit authorization, storage outside the repo, mode 0600, no stdout printing, short rotation/revocation window, and deletion after the bounded operation.

## Least-privilege phases

### Phase 0: #490 read-only baseline

Purpose: discovery and decision only.

Minimum read roles, scoped as narrowly as practical:

- Organization/folder/project discovery: `roles/browser` or equivalent custom read-only Resource Manager permissions.
- IAM census: `roles/iam.securityReviewer` on the relevant organization/folders/projects.
- Billing readback: `roles/billing.viewer` on `billingAccounts/01FA88-CC4968-ADF817`.
- Service/API and quota readback: `roles/serviceusage.serviceUsageViewer` and `roles/compute.viewer` on target projects.

No key is needed for this phase; refreshed user login already proved #490 readback viability.

### Phase 1: #491 Terraform bootstrap

Purpose: create or normalize the Terraform backend, deployment identity, provider pins, and saved-plan workflow when #491 authorizes mutation.

Minimum mutation roles for the Terraform service account, scoped to the accepted host project and billing account unless #491 explicitly widens scope:

- Attach/verify billing for bootstrap project: `roles/billing.user` on `billingAccounts/01FA88-CC4968-ADF817`.
- Enable only #491-approved APIs: `roles/serviceusage.serviceUsageAdmin` on `cs-host-377d41e71a824f92802120`.
- Manage Terraform state bucket and object versioning: `roles/storage.admin` on the host project or a narrower custom role for the exact backend bucket.
- Create/manage deployment service accounts: `roles/iam.serviceAccountAdmin` on the host project.
- Bind only #491-approved IAM edges: prefer a custom role with `resourcemanager.projects.getIamPolicy` and `resourcemanager.projects.setIamPolicy` on the host project; if using predefined roles, keep `roles/resourcemanager.projectIamAdmin` scoped only to the host project.
- If WIF is implemented in #491: `roles/iam.workloadIdentityPoolAdmin` scoped to the host project.

Do not grant broad Organization Admin or project Owner to the Terraform service account unless a later issue records a narrower alternative is insufficient.

### Phase 2: #492+ organization/platform mutations

Purpose: only when future issues authorize folder/project/billing/API/IAM or platform-network changes beyond the host project.

Add roles incrementally and issue-locally:

- Folder/project creation: `roles/resourcemanager.projectCreator` on the approved parent folder, plus folder admin only if the issue mutates folders.
- Shared VPC/network work: `roles/compute.networkAdmin` scoped to the approved host/service projects.
- KMS/logging/monitoring/storage: grant the narrow product-admin roles only on the target project and only for the issue that needs them.

## First non-secret proof command

Before any #491 apply, prove the execution identity has the intended permissions without exposing credentials and without mutating GCP:

```bash
gcloud projects test-iam-permissions cs-host-377d41e71a824f92802120 \
  --impersonate-service-account=tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com \
  --permissions=resourcemanager.projects.get,serviceusage.services.enable,storage.buckets.create,iam.serviceAccounts.create,resourcemanager.projects.getIamPolicy,resourcemanager.projects.setIamPolicy
```

Expected proof: the returned permission list contains only the permissions #491 approved for the bootstrap operation. A missing permission blocks apply; an unexpectedly broad permission set should be treated as a least-privilege finding.

For billing attachment proof, use the billing-account permissions test/readback available to the operator account or the Terraform service account. If the specific billing permissions test route is unavailable, #491 should retain a read-only billing project describe before and after the authorized apply rather than printing any credential material.

## First mutation proof after #491 authorization

The first create/update proof should be a saved Terraform plan followed by a bounded apply that creates only the #491 backend/deployment-identity resources, then records:

- exact impersonated service account,
- exact Terraform plan digest,
- exact resources created or updated,
- billing project binding,
- backend versioning/locking/readback,
- zero credential material in stdout/stderr/artifacts,
- rollback/delete plan where applicable.
