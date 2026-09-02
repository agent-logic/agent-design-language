# Structured Review Prompt

Template: 1.0.0

Issue: 594

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/594/diff-hygiene.log
.csdlc/evidence/594/runtime-log-archive.log
.csdlc/evidence/594/runtime-log-archive-config.log
.csdlc/evidence/594/terraform-log-archive.log
.csdlc/prepared/issues/594/validate-diff-hygiene.sh
.csdlc/prepared/issues/594/validate-runtime-log-archive.sh
.csdlc/prepared/issues/594/validate-terraform-log-archive.sh
adl-runtime-kernel/src/config.rs
adl-runtime-kernel/src/observability/vector.rs
adl-runtime-kernel/tests/configuration.rs
adl-runtime-kernel/tests/observability.rs
infra/aws/runtime/log-archive

## Prompts

- Can any S3 or Vector failure block Runtime readiness, operation, or recovery?
- Are buffer, retry, disk, and telemetry bounds explicit and tested?
- Are object identity and payloads redacted and portable?
- Are bucket and IAM controls least privilege and Terraform-owned?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Live AWS delivery/readback remains separately authorization-gated and was not executed by this local implementation review.

## Review Result

Revision: Some("git-blake3:061f7f740b2cbb8dfeb702193601e212b0e6035f:71e5db8b438e6b3a5e80bd68cb0b9359706b282354290b460956f8651edf51f7")

Reviewer: Some("codex:/root/review_594_prepr_r2")

Result: pass
