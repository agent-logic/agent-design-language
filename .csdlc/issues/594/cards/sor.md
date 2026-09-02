# Structured Output Record

Template: 1.0.0

Issue: 594

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented bounded Runtime redacted-log archival to S3 with optional Runtime init configuration, rendered Vector S3 delivery, focused tests including S3 archive outage-survival proof, issue-owned validation wrappers, and an isolated Terraform archive module.

## Artifacts

- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/src/observability/vector.rs
- adl-runtime-kernel/tests/configuration.rs
- adl-runtime-kernel/tests/observability.rs
- .csdlc/prepared/issues/594/validate-runtime-log-archive.sh
- .csdlc/prepared/issues/594/validate-diff-hygiene.sh
- .csdlc/prepared/issues/594/validate-terraform-log-archive.sh
- .csdlc/prepared/issues/594/validate-live-aws.sh
- .csdlc/evidence/594/runtime-log-archive.log
- .csdlc/evidence/594/runtime-log-archive-config.log
- .csdlc/evidence/594/terraform-log-archive.log
- .csdlc/evidence/594/diff-hygiene.log
- infra/aws/runtime/log-archive/.gitignore
- infra/aws/runtime/log-archive/.terraform.lock.hcl
- infra/aws/runtime/log-archive/versions.tf
- infra/aws/runtime/log-archive/variables.tf
- infra/aws/runtime/log-archive/locals.tf
- infra/aws/runtime/log-archive/main.tf
- infra/aws/runtime/log-archive/outputs.tf
- infra/aws/runtime/log-archive/archive_contract.tftest.hcl

## Execution

- Added optional observability_pipeline.s3_archive runtime init configuration with lowercase AWS region, DNS-compatible bucket, and DNS-safe environment, Polis, and Runtime identity validation.
- Rendered runtime_v3_s3_archive from runtime_v3_redacted through a bounded delivery transform with identity-partitioned keys, SSE-S3, gzip JSON, disabled S3 health checks, 5 MiB or 60 second batching, 512 MiB drop-newest disk buffering, bounded retry settings, and explicit failure/drop telemetry annotations.
- Added focused Runtime configuration and observability tests, including a pinned Vector validate check for the generated archive config and an archive-enabled runtime startup/master-log survival test.
- Tightened the issue-owned Runtime validation wrapper so it runs S3 archive configuration tests, Vector rendering/validation tests, and the archive outage-survival test.
- Added an issue-owned diff hygiene wrapper that emits a non-empty success receipt after git diff --check passes.
- Added infra/aws/runtime/log-archive Terraform for private S3 bucket controls, versioning, lifecycle retention, SSE-S3, bucket-owner-enforced ownership, and exact-prefix publisher IAM policy with terraform test assertions.
- Added a module-local Terraform ignore rule so generated provider cache files are not tracked.

## Validation

[
  {
    "command": [
      "/bin/bash",
      ".csdlc/prepared/issues/594/validate-diff-hygiene.sh"
    ],
    "purpose": "Issue 594 diff hygiene validation with non-empty retained success receipt",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  },
  {
    "command": [
      "/bin/bash",
      "/Volumes/FastWork/adl-worktrees/adl-issue-594-runtime-logs-s3-archive/.csdlc/prepared/issues/594/validate-runtime-log-archive.sh"
    ],
    "purpose": "Issue 594 Runtime S3 archive validation covering configuration parsing, unsafe identity rejection, Vector S3 sink rendering, pinned Vector config validation, and archive-enabled startup/master-log survival with disabled S3 health checks and bounded drop-newest buffering.",
    "outcome": "passed",
    "evidence_ref": "runtime-log-archive.log"
  },
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "configuration",
      "--no-tests=fail",
      "-E",
      "test(s3_archive)"
    ],
    "purpose": "Issue 594 Runtime init S3 archive configuration validation",
    "outcome": "passed",
    "evidence_ref": "runtime-log-archive-config.log"
  },
  {
    "command": [
      "/bin/bash",
      "/Volumes/FastWork/adl-worktrees/adl-issue-594-runtime-logs-s3-archive/.csdlc/prepared/issues/594/validate-terraform-log-archive.sh"
    ],
    "purpose": "Issue 594 Terraform S3 archive validation",
    "outcome": "passed",
    "evidence_ref": "terraform-log-archive.log"
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
