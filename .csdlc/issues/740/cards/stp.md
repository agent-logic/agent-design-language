# Structured Task Prompt

Template: 1.0.0

Issue: 740

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Bounded #740 post-merge corrective for #730/#738 GCP-B1 proof only.

## Deliverables

- .csdlc/prepared/issues/740/validate_gcp_b1_corrective.sh
- .csdlc/prepared/issues/740/run-gcp-b1-corrective-proof.sh
- .csdlc/prepared/issues/740/validate_typed_issue.sh
- Corrected issue-owned GCP-B1 proof tooling that avoids retained OAuth tokens.
- Redacted legacy service-account key disposition evidence.
- Redacted Terraform-backend canary live proof with generation and digest.
- Typed C-SDLC v2 cards and review/publication records for #740.

## Acceptance

1. AC1: No generated backend file, recovery artifact, evidence artifact, lifecycle record, or repo-local diagnostic retains an OAuth access token, service-account key material, static credential path, or equivalent secret; automated validation fails on retained `access_token =`, bearer-token-shaped content, or provider credential material.
2. AC2: Terraform backend authentication is passed through non-retained environment or CLI/backend configuration; generated `.tf` files do not embed OAuth tokens, and failure cleanup deletes or sanitizes generated backend/auth artifacts before retention.
3. AC3: Mutation authorization is bound to authenticated operator approval and the exact reviewed plan/head, naming project, bucket, service account, plan digest, reviewed repository/head, expiry, rollback command, and authorized actor or approval receipt; predictable self-issued JSON is rejected.
4. AC4: The legacy user-managed service-account key is enumerated without exposing key material and is either revoked during the authorized window or retained only under an explicit, time-bounded operator exception naming owner, key id/create time, expiry, and reason.
5. AC5: A unique canary is written through a Terraform GCS backend state operation, not raw `gcloud storage cp`; retained redacted proof includes the backend state-object generation and digest showing Terraform wrote and recovered the canary state.
6. AC6: `CLOUDSDK_CONFIG` and Terraform data directories are issue/run-specific, disposable, and cleaned or sanitized after the run; no shared persistent GCloud auth/cache directory is used for the live proof.
7. AC7: Live readback re-proves the existing bucket remains private, versioned, seven-day soft-delete protected, uniform-bucket-level-access enabled, and public-access-prevention enforced.
8. AC8: Local proof records distinguish historical merged #730/#738 evidence from post-merge corrective proof; no retroactive pre-merge review claim is made.
9. AC9: Exact-head independent review and required CI pass before merge.

## Dependencies

- #730 historical source issue
- PR #738 merged main commit 44aa83bd168f4bf037e1d159aac70b966827db2b
- Authenticated GCP source identity with Token Creator on tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com
- Existing Terraform state bucket adl-tf-state-cs-host-377d41e71a824f92802120

## Inputs

- GitHub issue #740
- GitHub issue #730
- GitHub PR #738
- .csdlc/prepared/issues/730/run-gcp-b1-proof.sh
- infra/gcp/bootstrap/**
- Post-merge review findings at PR #738 head 305547e0519c3a92174bc33e20f11d948a99e016

## Non Goals

- Issue #446 work.
- Creating or using new static service-account keys.
- Compute, GPU, network, DNS, billing, folder, organization, or unrelated IAM deployment.
- GCP D1 or runtime deployment work.
- Deleting the adopted remote Terraform state bucket.
