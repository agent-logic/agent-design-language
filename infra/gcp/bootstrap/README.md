# GCP Terraform bootstrap

Issue #491 owns this root. It creates the private, versioned GCS bucket used for ADL Terraform remote state in the approved company host project.

Target project: `cs-host-377d41e71a824f92802120`

Bootstrap service account: `tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com`

Sprint execution uses the approved service-account key as command-scoped source credentials:

```sh
GOOGLE_APPLICATION_CREDENTIALS=/Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json terraform init -backend=false
GOOGLE_APPLICATION_CREDENTIALS=/Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json terraform plan -out=tfplan
```

Keep the key file outside the repository and never print or commit its contents.

Do not commit `terraform.tfstate`, `tfplan`, `.terraform/`, credentials, or provider-generated local state. After the bucket exists, copy `backend.tf.example` to `backend.tf`, run `terraform init -migrate-state`, and quarantine/remove any local state after verifying migration.
