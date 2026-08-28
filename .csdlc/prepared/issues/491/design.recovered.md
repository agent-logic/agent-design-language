# Issue #491 Design — GCP-B Terraform bootstrap

## Intent

Issue #491 produces the recoverable GCP Terraform bootstrap for the approved company host project. It must create or normalize the Terraform remote-state backend and deployment identity while proving versioning, privacy, auditability, provider pins, impersonation, saved-plan review, break-glass posture, and recoverable removal of local bootstrap state.

## Company identity decision

#490 accepted the long-term host project and recommended a company-controlled Terraform bootstrap identity:

- Project: `cs-host-377d41e71a824f92802120`
- Service account: `tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com`
- Preferred execution: short-lived service-account impersonation from `daniel@agent-logic.ai` and later CI Workload Identity Federation.

The operator explicitly authorized creating a static key to avoid command-by-command prompts. Initial key creation was blocked by organization policy `constraints/iam.managed.disableServiceAccountKeyCreation`; after operator confirmation that they own the org policy, a narrow bootstrap window was opened, one JSON key was created at `/Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json`, a read-only project-describe smoke check passed with that key, and service-account key creation policy was re-closed. The retained evidence records only path and metadata, never key contents.

Preferred execution remains keyless impersonation through short-lived credentials; the static key is a local operator-approved break-glass/bootstrap credential for this sprint.

## Scope

Owned surfaces:

- `infra/gcp/bootstrap/**`
- `docs/operations/cloud/gcp/terraform-bootstrap/**`
- `docs/milestones/v0.92.1/evidence/cloud/gcp-b/**`
- `.csdlc/prepared/issues/491/**`

## Bootstrap shape

The implementation should provide:

1. A pinned Terraform root/module for a private, versioned, auditable GCS backend.
2. A keyless provider configuration using `impersonate_service_account`.
3. Local-state bootstrap instructions that remove or quarantine local state after remote backend creation.
4. Readback scripts proving backend bucket versioning, IAM, audit/log posture, service-account existence, impersonation readiness, and local key metadata without exposing key contents.
5. A break-glass section that records how the local key was created, where it is expected, and how to rotate/revoke it or return to impersonation-only execution.

## Dependency gates

- #490 must be terminal and ancestral before #491 bind/implementation.
- GCP mutation must use the company account and the approved host project.
- Static service-account keys require explicit operator approval, must stay outside the repository under `$HOME/keys`, and must never be printed or retained in evidence.
- Credentials, ADC tokens, service-account private keys, or raw token files must never enter retained evidence.

## Non-goals

- Runtime deployment.
- Production hierarchy rollout beyond the host project.
- AWS changes.
- GPU launch.
- Persistent organization-policy weakening.

## Validation model

Pre-bind validation proves the design packet, keyless identity decision, operator-approved key path, owned paths, and issue-local validator/readback entrypoints.

Post-bind validation proves Terraform formatting/validation, provider pins, backend privacy/versioning, impersonation, saved-plan review, local-state cleanup, and redacted retained evidence.

Live GCP readbacks are allowed only when they do not print credentials and are scoped to the accepted host project and service account.

## Failure policy

Fail closed if state recovery fails, the local static key is missing when a key-backed command is requested, reviewed plan identity drifts, credential material would be retained, or mutation would target a project other than `cs-host-377d41e71a824f92802120`.
