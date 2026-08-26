# Structured Output Record

Template: 1.0.0

Issue: 356

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Expose minimal read-only redacted accessors on terminal #350's sealed Observatory authority projection.

## Artifacts

- adl-runtime/src/distributed/serving_authority.rs
- adl-runtime/tests/distributed_observatory_authority_projection.rs

## Execution

- Borrowed accessors for four opaque redacted references
- Copied accessors for committed index, generations, digest refs, signer count, deadline and finalization time
- Focused A/A getter, A/B denial, and redaction proof

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_observatory_authority_projection",
      "--features",
      "internal-test-fixtures",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Prove all sealed projection accessors, A/B mismatch denial, canonical binding, durable quorum restore rejection, integer bounds, and redaction.",
    "outcome": "passed",
    "evidence_ref": "8 tests passed; 0 failed; 0 ignored at source commit 8ab811777"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_observatory_authority_projection",
      "--features",
      "internal-test-fixtures",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings in exact accessor source and focused target.",
    "outcome": "passed",
    "evidence_ref": "strict Clippy completed successfully at source commit 8ab811777"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "5bff0099858f005bcc045b0aa7548be4892a2acb...cdc242321"
    ],
    "purpose": "Prove clean patch syntax and exact two-product-path scope against terminal #350 base.",
    "outcome": "passed",
    "evidence_ref": "exit 0 with no output; product diff limited to serving_authority.rs and distributed_observatory_authority_projection.rs"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_observatory_authority_projection",
      "--features",
      "internal-test-fixtures",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Run the focused projection target.",
    "outcome": "passed",
    "evidence_ref": "focused-accessors.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_observatory_authority_projection",
      "--features",
      "internal-test-fixtures",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict Clippy.",
    "outcome": "passed",
    "evidence_ref": "strict-clippy.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
