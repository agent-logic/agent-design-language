# Structured Output Record

Template: 1.0.0

Issue: 495

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented XCL-01 cross-cloud Runtime Terraform conversion with repaired #194 optional voter denominator coverage, repaired GCP artifact-source/cleanup-deadline mapping, static validator proof, Terraform formatting, and non-apply provider schema validation.

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

- Added real AWS optional private voter instances behind launch_voters, preserving #194 AwsVoterA/AwsVoterB private subnet placement, private security group membership, encrypted gp3 root disks, no public IPs, cleanup tags, voter node identity, and endpoint dependencies.
- Added AWS optional_voter_ids output and validator checks so #194 optional voter claims cannot pass without Terraform resources.
- Added GCP artifact_bucket/artifact_prefix inputs, optional service-account storage.objectViewer binding, metadata/startup artifact-source record, cleanup deadline labels/metadata, and portable outputs for artifact_source and cleanup_deadline.
- Updated denominator inventory, AWS/GCP README truth, retained proof packet, and governed validator to reflect the repaired R2 findings without claiming plan/apply/destroy or paid live proof.

## Validation

[
  {
    "command": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/xcl-01/validate-xcl-01-cross-cloud-runtime-terraform.sh",
      "--lane=all"
    ],
    "purpose": "Run the issue-owned governed XCL-01 validator covering denominator inventory, portable contract, AWS/GCP Terraform surfaces, #194 optional voters, GCP artifact IAM/deadline mapping, lockfiles, CloudFormation rollback retention, credential redaction, and live-proof gating.",
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
    "evidence_ref": "terminal: terraform fmt -check completed with exit 0 after formatting infra/aws/runtime/xcl-01/outputs.tf"
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
    "evidence_ref": "terminal: git diff --check origin/main...HEAD and git diff --check completed with exit 0"
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
