# Structured Output Record

Template: 1.0.0

Issue: 495

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented XCL-01 cross-cloud Runtime Terraform conversion with repaired #268 AWS Runtime-host denominator coverage, denominator inventory authority, static validator proof, Terraform formatting, and non-apply provider schema validation.

## Artifacts

- .csdlc/prepared/issues/495/denominator-inventory.md
- infra/runtime-portable/README.md
- infra/runtime-portable/runtime-workload-contract.v1.json
- infra/aws/runtime/xcl-01/README.md
- infra/aws/runtime/xcl-01/main.tf
- infra/aws/runtime/xcl-01/variables.tf
- infra/aws/runtime/xcl-01/outputs.tf
- infra/aws/runtime/xcl-01/versions.tf
- infra/aws/runtime/xcl-01/.terraform.lock.hcl
- infra/gcp/workloads/xcl-01/README.md
- infra/gcp/workloads/xcl-01/main.tf
- infra/gcp/workloads/xcl-01/variables.tf
- infra/gcp/workloads/xcl-01/outputs.tf
- infra/gcp/workloads/xcl-01/versions.tf
- infra/gcp/workloads/xcl-01/.terraform.lock.hcl
- docs/milestones/v0.92.1/evidence/cloud/xcl-01/xcl-01-cross-cloud-runtime-terraform-proof.md
- docs/milestones/v0.92.1/evidence/cloud/xcl-01/validate-xcl-01-cross-cloud-runtime-terraform.sh
- .csdlc/evidence/495

## Execution

- Added the provider-neutral Runtime workload contract for the admitted #194/#268 denominator.
- Added the missing .csdlc/prepared/issues/495/denominator-inventory.md authority referenced by the approved design.
- Added AWS Terraform conversion surfaces that preserve #194 private network semantics and #268 Runtime host semantics, including EC2 Runtime host, IAM role, instance profile, SSM policy, pinned S3 object-version read permission, IMDSv2, encrypted gp3 root disk, optional retained EBS attachment, optional operator break-glass SSH, bootstrap log, readiness marker, and readiness outputs.
- Added GCP Terraform conversion surfaces that expose service-account, firewall, private-access, disk, startup-script, label, and readiness-marker differences explicitly.
- Added retained Terraform provider lockfiles for AWS and GCP after backend-disabled non-apply init.
- Updated the retained XCL-01 proof packet and governed static validator with no credential material and no plan/apply/destroy claim.

## Validation

[
  {
    "command": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/xcl-01/validate-xcl-01-cross-cloud-runtime-terraform.sh",
      "--lane=all"
    ],
    "purpose": "Run the issue-owned governed XCL-01 validator covering denominator inventory, portable contract, AWS/GCP Terraform surfaces, lockfiles, CloudFormation rollback retention, credential redaction, and live-proof gating.",
    "outcome": "passed",
    "evidence_ref": "terminal: xcl-01 governed validation passed: --lane=all"
  },
  {
    "command": [
      "terraform",
      "fmt",
      "-check",
      "infra/aws/runtime/xcl-01",
      "infra/gcp/workloads/xcl-01"
    ],
    "purpose": "Reject unformatted Terraform in the AWS and GCP XCL-01 roots.",
    "outcome": "passed",
    "evidence_ref": "terminal: terraform fmt -check completed with exit 0"
  },
  {
    "command": [
      "terraform",
      "init",
      "-backend=false",
      "-input=false"
    ],
    "purpose": "Initialize the AWS XCL-01 Terraform root without backend, plan, apply, destroy, or credentials to enable provider schema validation.",
    "outcome": "passed",
    "evidence_ref": "terminal: infra/aws/runtime/xcl-01 init reused lockfile provider selection and succeeded"
  },
  {
    "command": [
      "terraform",
      "validate"
    ],
    "purpose": "Validate the AWS XCL-01 Terraform root syntax and provider schema without cloud mutation.",
    "outcome": "passed",
    "evidence_ref": "terminal: infra/aws/runtime/xcl-01 Success! The configuration is valid."
  },
  {
    "command": [
      "terraform",
      "init",
      "-backend=false",
      "-input=false"
    ],
    "purpose": "Initialize the GCP XCL-01 Terraform root without backend, plan, apply, destroy, or credentials to enable provider schema validation.",
    "outcome": "passed",
    "evidence_ref": "terminal: infra/gcp/workloads/xcl-01 init reused lockfile provider selection and succeeded"
  },
  {
    "command": [
      "terraform",
      "validate"
    ],
    "purpose": "Validate the GCP XCL-01 Terraform root syntax and provider schema without cloud mutation.",
    "outcome": "passed",
    "evidence_ref": "terminal: infra/gcp/workloads/xcl-01 Success! The configuration is valid."
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Reject whitespace and patch hygiene problems before review.",
    "outcome": "passed",
    "evidence_ref": "terminal: git diff --check origin/main...HEAD completed with exit 0"
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
