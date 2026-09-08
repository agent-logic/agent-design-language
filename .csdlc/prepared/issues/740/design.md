# Issue 740 design

## Purpose

Repair the post-merge #730/#738 GCP-B1 proof so the accepted remote-state bootstrap evidence is credential-safe, authorization-bound, legacy-key-disposition complete, and genuinely proves Terraform GCS backend write/recovery behavior.

## Context

PR #738 merged at `44aa83bd168f4bf037e1d159aac70b966827db2b` after head `305547e0519c3a92174bc33e20f11d948a99e016`. A later exact-head review found four P1 gaps:

1. generated backend files can retain short-lived OAuth access tokens, including in failure recovery;
2. the mutation authorization guard accepts self-issued predictable JSON;
3. the legacy user-managed service-account key disposition is absent;
4. the canary used raw `gcloud storage cp` instead of a Terraform backend write.

This issue is a post-merge corrective. It must not rewrite history or claim the merged #738 review was sufficient.

## Design

The corrective proof has two layers.

First, local deterministic validation checks the script and retained evidence contract:

- no retained `access_token =` backend configuration;
- no bearer-token-shaped or service-account-key-shaped material in issue-owned retained artifacts;
- authorization evidence requires an external authenticated approval binding instead of a self-issued JSON file;
- Terraform-backend canary proof must name a backend state object generation and digest;
- evidence records must identify this as post-merge corrective proof for #740.

Second, the live proof runs within the declared GCP-B1 denominator:

- project `cs-host-377d41e71a824f92802120`;
- location `us-west2`;
- bucket `adl-tf-state-cs-host-377d41e71a824f92802120`;
- impersonated service account `tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com`;
- no static service-account key creation or selection.

The proof script must use an issue/run-specific `CLOUDSDK_CONFIG` and Terraform data directories. Credentials may be minted through `gcloud auth print-access-token --impersonate-service-account`, but the token may only live in process environment or command arguments for the bounded command that needs it. It must not be written to generated `.tf`, retained recovery files, lifecycle records, or evidence JSON.

For the Terraform backend canary, use a small issue-owned Terraform configuration with a GCS backend prefix dedicated to #740. Apply a state-only canary resource or output whose value includes a unique nonce. Then recover the corresponding GCS backend state object by immutable generation and verify the redacted digest/nonce evidence. Do not substitute raw object upload for this proof.

For the legacy key disposition, list user-managed keys for the bootstrap service account without exposing key material. Either revoke the legacy key when clearly authorized within the issue scope, or record an explicit time-bounded exception with owner, key id/create time, expiry, and reason.

## Review boundary

Review #740 for the corrected script, validation, redacted evidence, and lifecycle truth only. Do not review or mutate #446, GCP-D1, runtime deployment, GPU provisioning, or unrelated cloud resources.
