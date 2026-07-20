# Structured Output Record

Template: 1.0.0

Issue: 4647

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Retained the exact reviewer-authored WP-19 verification artifact, remediated both P1 findings, dispositioned all 22 WP-19 findings, fixed accepted P2/P3 code and documentation defects, and added a WP-20 remediation matrix plus pre-PR review record.

## Artifacts

- docs/milestones/v0.91.7/review/V0917_EXTERNAL_REVIEW_VERIFICATION_2026-07-19.md
- docs/milestones/v0.91.7/review/wp20_remediation_4647/WP19_FINDING_REMEDIATION_MATRIX_4647.md
- docs/milestones/v0.91.7/review/wp20_remediation_4647/PRE_PR_REVIEW_4647.md

## Execution

- Isolated authoritative coverage LLVM profile output by run id and preserved sibling profile data during cleanup
- Pinned AWS Bedrock invocation to an operator-approved expected STS account hash before InvokeModel and removed false sts_verified status
- Hardened CSM runtime API audit-write failure handling, redaction, OPTIONS admission disclosure, GET projection, PID liveness, client-error sanitization, and serialization fallback status
- Fixed local Ollama streaming UTF-8 chunk handling and malformed-byte recovery
- Fixed provider IPv6 loopback endpoint classification and provider coverage-impact mapping
- Updated current v0.91.7/v0.91.8 planning and review surfaces for #5406, #5489, #5383, Fable lane terminology, publication path claims, and digest identity
- Recorded every WP19-01 through WP19-22 finding as fixed or fixed by current code in a WP-20 remediation matrix

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "-p",
      "adl",
      "runtime_api_redaction_is_key_aware_and_preserves_benign_ids",
      "--",
      "--nocapture"
    ],
    "purpose": "Prove exact account keys redact string and numeric account identifiers while benign counters remain allowed.",
    "outcome": "passed",
    "evidence_ref": "local stdout, CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/4647/adl-target, 1 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "-p",
      "adl",
      "provider_setup_writes_bedrock_account_pin_material",
      "--",
      "--nocapture"
    ],
    "purpose": "Prove generated Bedrock setup includes profile, region, account hash environment pin, and clear fail-closed README guidance.",
    "outcome": "passed",
    "evidence_ref": "local stdout, CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/4647/adl-target, 1 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "-p",
      "adl",
      "invocation_artifact_and_http_constructor_error_paths_are_exercised",
      "--",
      "--nocapture"
    ],
    "purpose": "Prove held invocation artifact locks classify native and Bedrock post-success artifact failures as non-retryable partial-success-unknown.",
    "outcome": "passed",
    "evidence_ref": "local stdout, CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/4647/adl-target, 1 passed"
  },
  {
    "command": [
      "csdlc-validate",
      "--request",
      ".csdlc/prepared/issues/4647/validate-current.json"
    ],
    "purpose": "Validate current diff hygiene and C-SDLC v2 doctor pass through the typed PVF request.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/prepared/issues/4647/validation/current, disposition local_pass with diff-check and csdlc-doctor passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "-p",
      "adl",
      "bedrock_error_sanitizer",
      "--",
      "--nocapture"
    ],
    "purpose": "Prove Bedrock diagnostics redact signed AWS values, AWS ARNs, and raw account IDs while retaining useful AccessDenied context.",
    "outcome": "passed",
    "evidence_ref": "local stdout, CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/4647/adl-target, 3 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "-p",
      "adl",
      "bedrock_error_sanitizer",
      "--",
      "--nocapture"
    ],
    "purpose": "Prove Bedrock diagnostics redact standard and partitioned AWS ARNs plus raw account IDs while retaining useful AccessDenied context.",
    "outcome": "passed",
    "evidence_ref": "local stdout, CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/4647/adl-target, 4 passed"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl/Cargo.toml"
    ],
    "purpose": "Verify the post-clippy repair formats cleanly before republishing PR #5588.",
    "outcome": "passed",
    "evidence_ref": "local stdout, CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/4647/adl-target, exit 0"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Verify the post-publication clippy repair satisfies the same all-target warnings-as-errors gate that failed in GitHub.",
    "outcome": "passed",
    "evidence_ref": "local stdout, CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/4647/adl-target, Finished dev profile in 36.00s"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "provider::http_family::tests::bedrock_invocation_artifact",
      "--",
      "--nocapture"
    ],
    "purpose": "Verify Bedrock invocation artifact record/read/write/create-dir behavior still passes after the helper was refactored to satisfy clippy.",
    "outcome": "passed",
    "evidence_ref": "local stdout, CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/4647/adl-target, 5 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "provider::local::tests::ollama_streaming_buffers_split_multibyte_utf8",
      "--",
      "--nocapture"
    ],
    "purpose": "Verify the Ollama streaming UTF-8 buffering regression still passes after moving the test module to satisfy clippy.",
    "outcome": "passed",
    "evidence_ref": "local stdout, CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/4647/adl-target, 1 passed"
  }
]

## Integration

pr_open

## Publication

Publication: draft

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
