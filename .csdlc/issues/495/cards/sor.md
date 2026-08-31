# Structured Output Record

Template: 1.0.0

Issue: 495

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented XCL-01 cross-cloud Runtime Terraform conversion on current origin/main with repaired #194 optional voter denominator coverage, repaired AWS private S3 network egress, repaired GCP artifact-source/cleanup-deadline mapping, static validator proof, Terraform formatting, and non-apply provider schema validation.

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

- Merged current origin/main 5692d95ee6e4ee632833be348fa5601ddccbca1a into the #495 worktree after read-only collision checks showed no overlap with #495 paths.
- Preserved real AWS optional private voter instances behind launch_voters, #194 private subnet/security-group/no-public-IP/encrypted-gp3 semantics, voter node identity, and endpoint dependencies.
- Preserved explicit AWS runtime-instance HTTPS egress to the regional S3 prefix list via s3_prefix_list_id so IAM and the S3 gateway endpoint are paired with a real private network path.
- Preserved GCP artifact_bucket/artifact_prefix inputs, optional service-account storage.objectViewer binding, metadata/startup artifact-source record, cleanup deadline labels/metadata, and portable outputs for artifact_source and cleanup_deadline.
- Reran the governed validator, Terraform fmt, and non-apply AWS/GCP provider validation after the current-base merge without claiming plan/apply/destroy or paid live proof.

## Validation

[
  {
    "command": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/xcl-01/validate-xcl-01-cross-cloud-runtime-terraform.sh",
      "--lane=all"
    ],
    "purpose": "Run the issue-owned governed XCL-01 validator after current-base merge.",
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
    "purpose": "Reject unformatted Terraform after current-base merge.",
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
    "purpose": "Initialize the AWS XCL-01 Terraform root without backend, plan, apply, destroy, or credentials to enable provider schema validation after current-base merge.",
    "outcome": "passed",
    "evidence_ref": "terminal: infra/aws/runtime/xcl-01 init reused lockfile provider selection and succeeded"
  },
  {
    "command": [
      "terraform",
      "validate"
    ],
    "purpose": "Validate the AWS XCL-01 Terraform root syntax and provider schema without cloud mutation after current-base merge.",
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
    "purpose": "Initialize the GCP XCL-01 Terraform root without backend, plan, apply, destroy, or credentials to enable provider schema validation after current-base merge.",
    "outcome": "passed",
    "evidence_ref": "terminal: infra/gcp/workloads/xcl-01 init reused lockfile provider selection and succeeded"
  },
  {
    "command": [
      "terraform",
      "validate"
    ],
    "purpose": "Validate the GCP XCL-01 Terraform root syntax and provider schema without cloud mutation after current-base merge.",
    "outcome": "passed",
    "evidence_ref": "terminal: infra/gcp/workloads/xcl-01 Success! The configuration is valid."
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
