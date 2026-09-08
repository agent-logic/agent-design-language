# GCP-B Terraform bootstrap runbook

This runbook starts the recoverable Terraform backend for ADL in the company GCP host project.

## Identity

- Project: `cs-host-377d41e71a824f92802120`
- Region/location: `us-west2`
- Bootstrap service account: `tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com`
- Default Terraform auth mode: short-lived impersonation from an approved
  company human or federated source identity.

Do not create, select, require, paste, print, commit, or retain a
service-account key. Historical static-key bootstrap evidence is retired
break-glass provenance only, not a runnable default.

`US-WEST2` is a supported Cloud Storage bucket region in the public GCS
location catalog. If the company organization policy, project policy, quota, or
service-account permissions reject `us-west2` during the authorized plan/apply,
stop and amend the issue plus authorization packet before using a different
location.

## Local checks

```sh
bash .csdlc/prepared/issues/491/validate-gcp-b-bootstrap.sh .
bash .csdlc/prepared/issues/491/run-gcp-b-readbacks.sh --lane=static
terraform -chdir=infra/gcp/bootstrap fmt -check
terraform -chdir=infra/gcp/bootstrap init -backend=false
terraform -chdir=infra/gcp/bootstrap validate
bash .csdlc/prepared/issues/730/validate-gcp-b1.sh --lane=static
```

## Read-only impersonation proof

This reads metadata only and must stay scoped to the accepted project and service account:

```sh
CLOUDSDK_CONFIG="$(git rev-parse --path-format=absolute --git-common-dir)/csdlc-v2/gcloud-config"
export CLOUDSDK_CONFIG
gcloud auth print-access-token \
  --impersonate-service-account=tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com \
  --project=cs-host-377d41e71a824f92802120 >/dev/null
```

The repo-local Cloud SDK config mirrors the #491/#608 proof pattern: credential
cache/log writes stay under Git common storage and are not committed. The
selected identity still must be an approved company human or federated source
identity with Token Creator on the bootstrap service account.

## Bootstrap apply under #730 authorization

Create a request under `.git/csdlc-v2/authorizations/730.json` that names the
exact project, bucket, service account, reviewed saved-plan digest, rollback
command, 90-minute expiry, USD 5 first-month spend cap, and 30-minute apply
timeout. Prepare the no-mutation saved plan first:

```sh
bash .csdlc/prepared/issues/730/prepare-gcp-b1-plan.sh
```

The binary plan is stored under `.git/csdlc-v2/gcp-b1/730.tfplan`; the
worktree retains only the redacted plan text and digest evidence. After the
operator authorizes that exact digest, copy
`.csdlc/prepared/issues/730/authorization-template.json` to
`.git/csdlc-v2/authorizations/730.json`, replace `expires_at` with an
ISO-8601 UTC timestamp no more than 90 minutes in the future, and run:

```sh
bash .csdlc/prepared/issues/730/run-gcp-b1-proof.sh \
  --authorization .git/csdlc-v2/authorizations/730.json
```

The script fails closed before mutation if the authorization, source identity,
plan digest, plan denominator, rollback command, expiry, budget, or local
residue checks do not match #730.
