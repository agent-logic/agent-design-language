# Structured Output Record

Template: 1.0.0

Issue: 491

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Remediated #491 review findings by binding GCP readbacks to the approved service-account key path and expanding retained-secret scanning across all #491 product, lifecycle, prepared, and evidence surfaces while excluding ignored Terraform provider/cache artifacts.

## Artifacts

- .csdlc/prepared/issues/491/run-gcp-b-readbacks.sh
- .csdlc/prepared/issues/491/validate-gcp-b-bootstrap.sh
- infra/gcp/bootstrap
- docs/operations/cloud/gcp/terraform-bootstrap
- docs/milestones/v0.92.1/evidence/cloud/gcp-b

## Execution

- The identity-readonly lane now requires the approved GCP_B_KEY_FILE to exist, rejects conflicting CLOUDSDK_AUTH_CREDENTIAL_FILE_OVERRIDE and GOOGLE_APPLICATION_CREDENTIALS, exports both variables to the approved key path for the command-scoped readback, and reports credential_source=approved_key_file plus credential_binding_verified=true.
- The #491 validator now requires the credential-binding markers in the readback script and scans infra/gcp/bootstrap, .csdlc/issues/491, .csdlc/evidence/491, prepared artifacts, runbook docs, and milestone evidence for retained credential markers.
- The scan explicitly skips only the validator's own denylist and ignored Terraform provider/cache artifacts under .terraform so downloaded provider binaries do not create false retained-source failures.

## Validation

[
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "issue",
      "--issue",
      "491"
    ],
    "purpose": "Validate #491 typed issue/card truth after review recovery.",
    "outcome": "passed",
    "evidence_ref": "generation 25, status pass, findings []"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and patch-integrity defects.",
    "outcome": "passed",
    "evidence_ref": "exit 0 with no output"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/491/validate-gcp-b-bootstrap.sh",
      "."
    ],
    "purpose": "Prove the #491 bootstrap packet, owned paths, key-backed identity contract, wrong-project/service-account rejection, provider pins, local-state hygiene, and expanded retained-secret scan.",
    "outcome": "passed",
    "evidence_ref": "gcp-b bootstrap packet validation passed"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/491/run-gcp-b-readbacks.sh",
      "--lane=static"
    ],
    "purpose": "Prove the readback entrypoint has a non-credentialed static lane and reports only approved project, service account, and key-file metadata.",
    "outcome": "passed",
    "evidence_ref": "static lane performed no GCP API calls and reported key_file_present=true"
  },
  {
    "command": [
      "GCP_B_KEY_FILE=/Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json",
      "bash",
      ".csdlc/prepared/issues/491/run-gcp-b-readbacks.sh",
      "--lane=identity-readonly"
    ],
    "purpose": "Perform approved-key-backed read-only GCP proof for the accepted company project and bootstrap service account without printing key contents.",
    "outcome": "passed",
    "evidence_ref": "project_lifecycle_state=ACTIVE; service_account_readable=true; credential_source=approved_key_file; credential_binding_verified=true; approved_key_backed_readback=true; retained_output_redacted=true"
  },
  {
    "command": [
      "terraform",
      "-chdir=infra/gcp/bootstrap",
      "fmt",
      "-check"
    ],
    "purpose": "Prove Terraform formatting for the GCP bootstrap root.",
    "outcome": "passed",
    "evidence_ref": "exit 0 with no output"
  },
  {
    "command": [
      "terraform",
      "-chdir=infra/gcp/bootstrap",
      "validate"
    ],
    "purpose": "Prove the Terraform configuration validates.",
    "outcome": "passed",
    "evidence_ref": "Success! The configuration is valid."
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
