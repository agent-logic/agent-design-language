# Structured Output Record

Template: 1.0.0

Issue: 579

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repaired AWS-F runtime platform boundary after late review: removed executable ACM/Route53 public-edge ownership from AWS-F ALB surfaces, kept Runtime ingress default-closed, made remote-state isolation machine-visible, and tightened corrective validation/proof truth without running live AWS mutation.

## Artifacts

- infra/aws/modules/csm-runtime-alb/main.tf
- infra/aws/modules/csm-runtime-alb/variables.tf
- infra/aws/runtime/alb-origin/main.tf
- infra/aws/runtime/alb-origin/variables.tf
- infra/aws/runtime/alb-origin/terraform.tfvars.example
- infra/aws/runtime/alb-origin/versions.tf
- infra/aws/runtime/private-node/versions.tf
- infra/aws/runtime/README.md
- docs/operations/cloud/aws/runtime-platform/README.md
- docs/milestones/v0.92.1/evidence/cloud/aws-f/aws-f-runtime-platform-proof.md
- .csdlc/prepared/issues/579/validate-aws-f-corrective.sh

## Execution

- Removed Route53 alias creation and ACM certificate creation/validation resources from infra/aws/modules/csm-runtime-alb and stopped passing those controls from the AWS-F alb-origin root.
- Kept ALB certificate consumption to explicit certificate_arn or existing ISSUED ACM lookup, preserving #122 ownership of public DNS/certificate/public-edge exposure.
- Added S3 backend declarations to AWS-F runtime root stacks so state isolation is not only advisory.
- Updated AWS-F runtime docs and retained proof packet to record static/non-mutating proof, #122 public-edge ownership, and the still-deferred live disposable deployment/zero-residue proof gate.
- Strengthened the #579 corrective validator to reject AWS-F ACM/Route53 resources and world-open committed Runtime ingress without relying on stale #489 validator paths.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/579/validate-aws-f-corrective.sh",
      "--lane=terraform-static"
    ],
    "purpose": "Prove AWS-F Terraform surfaces are formatted, reject ACM/Route53 public-edge ownership, reject world-open committed Runtime ingress, require remote-state backend declarations, and preserve private-node ingress posture.",
    "outcome": "passed",
    "evidence_ref": "local stdout: aws-f corrective validation passed: terraform-static"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/579/validate-aws-f-corrective.sh",
      "--lane=security-validator-regression"
    ],
    "purpose": "Prove the corrective validator no longer relies on stale #489 path logic or overescaped world-open ingress matching.",
    "outcome": "passed",
    "evidence_ref": "local stdout: aws-f corrective validation passed: security-validator-regression"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/579/validate-aws-f-corrective.sh",
      "--lane=proof-truth"
    ],
    "purpose": "Prove AWS-F docs/evidence bound static proof, #122 public-edge ownership, state isolation wording, and live-proof deferral truthfully.",
    "outcome": "passed",
    "evidence_ref": "local stdout: aws-f corrective validation passed: proof-truth"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and conflict-marker errors in the #579 corrective diff.",
    "outcome": "passed",
    "evidence_ref": "local command exited 0 with no output"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
