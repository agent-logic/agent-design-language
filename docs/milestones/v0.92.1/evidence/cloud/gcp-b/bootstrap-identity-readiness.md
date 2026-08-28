# GCP-B bootstrap identity readiness

Issue: #491

Accepted project from #490: `cs-host-377d41e71a824f92802120`

Bootstrap service account: `tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com`

Recorded setup facts:

- The service account exists in the accepted company host project.
- Project-scoped bootstrap roles were granted for service usage, storage backend management, service-account management, and project IAM binding.
- Billing user was granted on the accepted billing account for bootstrap billing attachment.
- The active company user `daniel@agent-logic.ai` was granted `roles/iam.serviceAccountTokenCreator` on the bootstrap service account as a future impersonation prerequisite, but #491 sprint execution does not depend on short-lived impersonation.
- The operator explicitly authorized one local static key at `/Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json`.
- Organization policy initially blocked key creation, so project/org policy was temporarily set to allow service-account key creation, the key was created, and the policy was re-closed immediately afterward.
- The resulting key file exists locally at mode `0600`; read-only smoke checks using that key returned the active host project without printing token or key contents.

No key file contents, token contents, ADC database contents, or credential values are retained here.
