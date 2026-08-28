# Structured Output Record

Template: 1.0.0

Issue: 491

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the GCP-B Terraform bootstrap as an approved service-account key-backed sprint path with private/versioned remote state, pinned provider contract, redacted readbacks, ignored local Terraform state, and retained non-secret proof evidence.

## Artifacts

- infra/gcp/bootstrap/main.tf
- infra/gcp/bootstrap/provider.tf
- infra/gcp/bootstrap/variables.tf
- infra/gcp/bootstrap/outputs.tf
- infra/gcp/bootstrap/README.md
- infra/gcp/bootstrap/.terraform.lock.hcl
- docs/operations/cloud/gcp/terraform-bootstrap/README.md
- docs/milestones/v0.92.1/evidence/cloud/gcp-b/bootstrap-identity-readiness.md
- .csdlc/prepared/issues/491/validate-gcp-b-bootstrap.sh
- .csdlc/prepared/issues/491/run-gcp-b-readbacks.sh
- .csdlc/prepared/issues/491/design.recovered.md
- .csdlc/prepared/issues/491/diagram.recovered.mmd

## Execution

- Created infra/gcp/bootstrap with pinned hashicorp/google provider, private versioned GCS backend bucket, service-account IAM/member bindings, and ignored local Terraform cache/plan artifacts.
- Removed short-lived impersonation from the default sprint execution path; #491 uses the operator-approved service-account key only as command-scoped credential material outside the repository.
- Updated the GCP bootstrap runbook, milestone evidence, design packet, validator, and readback script to prove approved key-backed execution without printing or retaining key contents.
- Kept future non-key company identity work outside #491 scope while preserving the granted TokenCreator context as non-blocking future readiness evidence.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/491/validate-gcp-b-bootstrap.sh",
      "."
    ],
    "purpose": "Prove the #491 bootstrap packet, owned paths, key-backed identity contract, wrong-project/service-account rejection, provider pins, and local-state hygiene.",
    "outcome": "passed",
    "evidence_ref": "local run: gcp-b bootstrap packet validation passed"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/491/run-gcp-b-readbacks.sh",
      "--lane=static"
    ],
    "purpose": "Prove the readback entrypoint has a non-credentialed static lane and reports only approved project, service account, and key-file metadata.",
    "outcome": "passed",
    "evidence_ref": "local run: static lane performed no GCP API calls and reported key_file_present=true"
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
    "evidence_ref": "local run passed"
  },
  {
    "command": [
      "terraform",
      "-chdir=infra/gcp/bootstrap",
      "validate"
    ],
    "purpose": "Prove the Terraform configuration validates after backend-disabled initialization.",
    "outcome": "passed",
    "evidence_ref": "local run after terraform init -backend=false passed"
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
