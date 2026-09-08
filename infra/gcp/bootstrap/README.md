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
bash .csdlc/prepared/issues/730/prepare-gcp-b1-plan.sh
```

The binary plan is stored under `.git/csdlc-v2/gcp-b1/730.tfplan`. Review the
saved plan digest in `.csdlc/evidence/730/gcp-b1-plan-digest.json` before
apply. The authorized live proof removes the binary plan after it records
redacted readback and recovery evidence.

Do not commit `terraform.tfstate`, `tfplan`, `.terraform/`, credentials,
generated `backend.tf`, or provider-generated local state. After the bucket
exists, initialize a clean non-repo backend working directory from
`backend.tf.example`, prove remote readback and immutable-generation recovery,
then remove local Terraform residue from the repository worktree.
