# GCP Terraform bootstrap

Issue #491 owns this root. It creates the private, versioned GCS bucket used for ADL Terraform remote state in the approved company host project.

Target project: `cs-host-377d41e71a824f92802120`

Bootstrap service account: `tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com`

Preferred execution uses short-lived impersonation:

```sh
terraform init
terraform plan \
  -var='impersonate_service_account=tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com' \
  -out=tfplan
```

For the operator-approved sprint bootstrap key, run a command with:

```sh
GOOGLE_APPLICATION_CREDENTIALS=/Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json terraform plan -out=tfplan
```

Do not commit `terraform.tfstate`, `tfplan`, `.terraform/`, credentials, or provider-generated local state. After the bucket exists, copy `backend.tf.example` to `backend.tf`, run `terraform init -migrate-state`, and quarantine/remove any local state after verifying migration.
