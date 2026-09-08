# Structured Output Record

Template: 1.0.0

Issue: 730

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Added a tracked #730 authorization template for the exact saved-plan apply gate.

## Artifacts

- infra/gcp/bootstrap/provider.tf
- infra/gcp/bootstrap/variables.tf
- infra/gcp/bootstrap/terraform.tfvars.example
- infra/gcp/bootstrap/README.md
- docs/operations/cloud/gcp/terraform-bootstrap/README.md
- .csdlc/prepared/issues/730/validate-gcp-b1.sh
- .csdlc/prepared/issues/730/prepare-gcp-b1-plan.sh
- .csdlc/prepared/issues/730/run-gcp-b1-proof.sh
- .csdlc/evidence/730/static-validation.log
- .csdlc/evidence/730/live-proof-fail-closed.stderr.log
- .csdlc/evidence/730/live-proof-fail-closed.exitcode
- .csdlc/evidence/730/diff-check.log
- .csdlc/evidence/730/residue-scan.log
- .csdlc/prepared/issues/730/authorization-template.json

## Execution

- Added Google provider service-account impersonation for the existing tf-bootstrap identity instead of static key execution.
- Updated bootstrap region defaults and examples from the stale us-central1 value to the issue-declared us-west2 location, with an explicit stop if live org or project policy rejects that location.
- Rewrote the GCP Terraform bootstrap runbook to use impersonation, reviewed saved-plan digest authorization, and a no-key command path.
- Strengthened issue-owned static validation to detect static credential-file paths, verify bounded Terraform identity/location labels, run Terraform fmt/init/validate without backend mutation, and scan for local Terraform residue.
- Added a prepare-gcp-b1-plan script that creates the reviewed saved plan under Git common storage and retains only redacted/digest evidence in the worktree.
- Replaced the live proof placeholder with fail-closed authorization, impersonation, reviewed-plan digest, bounded plan, apply-timeout, readback, canary recovery, and residue checks.
- Added an authorization template carrying the exact project, region, bucket, service account, plan path, current saved-plan digest, spend cap, apply timeout, and rollback command required by the live proof script.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/730/validate-gcp-b1.sh",
      "--lane=static"
    ],
    "purpose": "No-mutation GCP-B1 static validation for impersonation-only Terraform contract, bounded region/project labels, Terraform fmt/init/validate, and residue scan.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/730/static-validation.log; gcp-b1 static validation passed"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/730/run-gcp-b1-proof.sh"
    ],
    "purpose": "Live GCP-B1 proof precondition guard without authorization artifact.",
    "outcome": "blocked",
    "evidence_ref": ".csdlc/evidence/730/live-proof-fail-closed.stderr.log; exact authorization artifact required; exit code 1 retained in .csdlc/evidence/730/live-proof-fail-closed.exitcode"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/730/prepare-gcp-b1-plan.sh"
    ],
    "purpose": "No-mutation saved-plan preparation using short-lived service-account impersonation before operator-reviewed apply authorization.",
    "outcome": "blocked",
    "evidence_ref": ".csdlc/evidence/730/prepare-plan.stderr.log; gcloud auth reauthentication failed because non-interactive execution cannot prompt"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/730/prepare-gcp-b1-plan.sh"
    ],
    "purpose": "No-mutation saved-plan preparation using short-lived service-account impersonation before operator-reviewed apply authorization.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/730/gcp-b1-plan-digest.json; plan_sha256 e7e8e604071cc9de01ddf27c07379ef5c53036e98cd51a370749b5f97362e99b; redacted plan .csdlc/evidence/730/gcp-b1-plan.redacted.txt"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/730/prepare-gcp-b1-plan.sh"
    ],
    "purpose": "No-mutation saved-plan preparation using short-lived service-account impersonation with Terraform provider data isolated outside the module tree.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/730/gcp-b1-plan-digest.json; plan_sha256 f34d0bc7d17110d8ca969eaf7a866b639ea20cf973cba20669465eea5457abf9; redacted plan .csdlc/evidence/730/gcp-b1-plan.redacted.txt"
  },
  {
    "command": [
      "gcloud",
      "storage",
      "buckets",
      "describe",
      "gs://adl-tf-state-cs-host-377d41e71a824f92802120",
      "--project=cs-host-377d41e71a824f92802120",
      "--format=json"
    ],
    "purpose": "Read-only pre-apply check for existing Terraform state bucket.",
    "outcome": "blocked",
    "evidence_ref": ".csdlc/evidence/730/preapply-bucket-describe.stderr.log; bucket not found 404; exit code 1"
  },
  {
    "command": [
      "GOOGLE_APPLICATION_CREDENTIALS=/nope",
      "bash",
      ".csdlc/prepared/issues/730/prepare-gcp-b1-plan.sh"
    ],
    "purpose": "Negative proof that saved-plan preparation rejects static service-account credential-file environment before any Terraform or cloud operation.",
    "outcome": "blocked",
    "evidence_ref": ".csdlc/evidence/730/static-key-rejection.stderr.log; static credential file environment is not allowed; exit code 1"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/730/validate-gcp-b1.sh",
      "--lane=static"
    ],
    "purpose": "Static proof after extending the live runner to migrate bootstrap state into the impersonated GCS backend, pull backend state from a clean repo-local probe, and reject backend impersonation drift.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/730/static-validation.log; gcp-b1 static validation passed; residue-scan.log has 0 lines; diff-check.log has 0 lines"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/730/validate-gcp-b1.sh",
      "--lane=static"
    ],
    "purpose": "Static proof after making post-apply cleanup failure-safe and requiring the clean GCS backend probe to contain the expected Terraform bucket and IAM-member resources.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/730/static-validation.log; gcp-b1 static validation passed; live-proof-fail-closed.stderr.log still reports exact authorization artifact required; residue-scan.log has 0 lines; diff-check.log has 0 lines"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
