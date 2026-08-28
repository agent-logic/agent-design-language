# GCP-B Terraform bootstrap runbook

This runbook starts the recoverable Terraform backend for ADL in the company GCP host project.

## Identity

- Project: `cs-host-377d41e71a824f92802120`
- Bootstrap service account: `tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com`
- Default Terraform auth mode for this sprint: approved service-account key under `$HOME/keys`, passed only as command-scoped source credentials.
- Operator-approved local key path: `/Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json`
- Future preferred auth: company-controlled non-key identity such as Workload Identity Federation once the provider path is ready.

Never paste, print, commit, or retain the JSON key contents.

## Local checks

```sh
bash .csdlc/prepared/issues/491/validate-gcp-b-bootstrap.sh .
bash .csdlc/prepared/issues/491/run-gcp-b-readbacks.sh --lane=static
terraform -chdir=infra/gcp/bootstrap fmt -check
terraform -chdir=infra/gcp/bootstrap init -backend=false
terraform -chdir=infra/gcp/bootstrap validate
```

## Read-only identity proof

This reads metadata only and must stay scoped to the accepted project and service account:

```sh
CLOUDSDK_AUTH_CREDENTIAL_FILE_OVERRIDE=/Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json \
  bash .csdlc/prepared/issues/491/run-gcp-b-readbacks.sh --lane=identity-readonly
```

## Bootstrap apply

Create and review a saved plan before apply:

```sh
GOOGLE_APPLICATION_CREDENTIALS=/Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json \
  terraform -chdir=infra/gcp/bootstrap plan -out=tfplan
terraform -chdir=infra/gcp/bootstrap show -no-color tfplan > docs/milestones/v0.92.1/evidence/cloud/gcp-b/tfplan.redacted.txt
```

Apply only after the reviewed plan is accepted:

```sh
GOOGLE_APPLICATION_CREDENTIALS=/Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json \
  terraform -chdir=infra/gcp/bootstrap apply tfplan
```

Then migrate to the GCS backend:

```sh
cp infra/gcp/bootstrap/backend.tf.example infra/gcp/bootstrap/backend.tf
GOOGLE_APPLICATION_CREDENTIALS=/Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json \
  terraform -chdir=infra/gcp/bootstrap init -migrate-state
```

Local state files must not remain as normal working files after migration; move them into a private, non-repo recovery location or delete them only after independent readback proves the remote state bucket is versioned and accessible.
