# GCP Terraform bootstrap

Issue #491 owns this root. It creates the private, versioned GCS bucket used for ADL Terraform remote state in the approved company host project.

Target project: `cs-host-377d41e71a824f92802120`

Bootstrap service account: `tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com`

Issue #730 replaces the historical static-key bootstrap with short-lived
service-account impersonation. Terraform must authenticate from an approved
company human or federated source identity and the Google provider must
impersonate the bootstrap service account directly. Do not configure a
service-account key file as the runnable default.

```sh
terraform -chdir=infra/gcp/bootstrap init -backend=false -input=false
terraform -chdir=infra/gcp/bootstrap plan -out=.csdlc/evidence/730/gcp-b1.tfplan
```

Review the saved plan digest before apply and remove the binary plan after the
authorized run records redacted evidence.

Do not commit `terraform.tfstate`, `tfplan`, `.terraform/`, credentials,
generated `backend.tf`, or provider-generated local state. After the bucket
exists, initialize a clean non-repo backend working directory from
`backend.tf.example`, prove remote readback and immutable-generation recovery,
then remove local Terraform residue from the repository worktree.
