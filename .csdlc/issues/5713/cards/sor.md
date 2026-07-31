# Structured Output Record

Template: 1.0.0

Issue: 5713

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented a repo-native Runtime v3 local TLS bootstrap command using rcgen, explicit managed_external/local_self_signed configuration, durable state-root-scoped certificate reuse, explicit replacement, rustls validation, and local trust documentation.

## Artifacts

- .csdlc/evidence/5713/local-tls-validation.md

## Execution

- adl-runtime/src/lib.rs
- adl-runtime/src/bin/adl-runtime-local-tls-bootstrap.rs
- adl-runtime/src/local_tls.rs
- adl-runtime/tests/local_tls.rs
- docs/architecture/RUNTIME_V3_ENTRYPOINT_SWITCH.md

## Validation

[
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove the touched adl-runtime crate has no Rust or Clippy warnings across targets.",
    "outcome": "passed",
    "evidence_ref": "runtime-v3-local-tls-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "local_tls"
    ],
    "purpose": "Prove rcgen local self-signed TLS bootstrap, rustls acceptance, stable restart reuse, restrictive key persistence, concurrent exclusion, explicit replacement, failed replacement preservation, externally managed preservation, and missing-mode fail-closed behavior.",
    "outcome": "passed",
    "evidence_ref": "runtime-v3-local-tls-focused.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Prove rustfmt and textual diff hygiene for the touched Runtime v3 local TLS files and docs.",
    "outcome": "passed",
    "evidence_ref": "runtime-v3-local-tls-format-diff.log"
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
