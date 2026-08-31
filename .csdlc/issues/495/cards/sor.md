# Structured Output Record

Template: 1.0.0

Issue: 495

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented XCL-01 cross-cloud Runtime Terraform conversion with repaired AWS and GCP #268 retained Runtime volume/disk mount/readiness semantics, preserved #194 optional voter/private-network coverage, preserved AWS private S3 network egress, preserved GCP artifact-source/cleanup-deadline mapping, static validator proof, Terraform formatting, and non-apply provider schema validation.

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

- Recovered the failed R6 exact-head review through typed review recovery before source mutation.
- Required retained_runtime_disk for the GCP #268 Runtime qualification host so #495 cannot claim the retained Runtime disk denominator without a persistent disk identity.
- Added retained_runtime_disk_device_name so the GCP disk attachment has a stable by-id path for startup-script mounting.
- Updated the GCP startup script to install and run adl-issue268-mount-runtime, wait for the retained persistent disk, verify an existing filesystem, mount /opt/adl-runtime, require /opt/adl-runtime/runtime/install, and write /var/lib/adl/issue268-bootstrap-ready only after retained-disk readiness is proven.
- Updated the GCP portable_contract readiness_command to require marker presence, mountpoint -q /opt/adl-runtime, and /opt/adl-runtime/runtime/install.
- Updated the denominator inventory and GCP README so the GCP mapping truthfully requires retained disk mount/install proof without claiming byte-for-byte EBS equivalence.
- Strengthened the issue-owned governed validator so future #495 proof fails if the GCP retained-disk variable, stable device name, mount helper, install-directory readiness, or readiness_command checks disappear.

## Validation

[
  {
    "command": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/xcl-01/validate-xcl-01-cross-cloud-runtime-terraform.sh",
      "--lane=all"
    ],
    "purpose": "Run the issue-owned governed XCL-01 validator after GCP retained-disk readiness remediation.",
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
    "purpose": "Reject unformatted Terraform after retained-disk readiness remediation.",
    "outcome": "passed",
    "evidence_ref": "terminal: terraform fmt -check completed with exit 0"
  },
  {
    "command": [
      "terraform",
      "-chdir=infra/aws/runtime/xcl-01",
      "init",
      "-backend=false"
    ],
    "purpose": "Initialize the AWS XCL-01 Terraform root without backend, plan, apply, destroy, or credentials to enable provider schema validation.",
    "outcome": "passed",
    "evidence_ref": "terminal: hashicorp/aws v6.62.0 installed; Terraform initialized successfully"
  },
  {
    "command": [
      "terraform",
      "-chdir=infra/aws/runtime/xcl-01",
      "validate"
    ],
    "purpose": "Validate the AWS XCL-01 Terraform root syntax and provider schema without cloud mutation.",
    "outcome": "passed",
    "evidence_ref": "terminal: infra/aws/runtime/xcl-01 Success! The configuration is valid."
  },
  {
    "command": [
      "terraform",
      "-chdir=infra/gcp/workloads/xcl-01",
      "init",
      "-backend=false"
    ],
    "purpose": "Initialize the GCP XCL-01 Terraform root without backend, plan, apply, destroy, or credentials to enable provider schema validation.",
    "outcome": "passed",
    "evidence_ref": "terminal: hashicorp/google v8.0.0 installed; Terraform initialized successfully"
  },
  {
    "command": [
      "terraform",
      "-chdir=infra/gcp/workloads/xcl-01",
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
  },
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "issue",
      "--issue",
      "495"
    ],
    "purpose": "Validate typed lifecycle/card truth after R6 recovery before replacing SOR execution truth.",
    "outcome": "passed",
    "evidence_ref": "terminal: generation 18, phase implemented, status pass"
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
