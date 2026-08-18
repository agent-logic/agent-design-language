# Structured Output Record

Template: 1.0.0

Issue: 350

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Seal replicated Observatory authority with durable quorum, deadline, exact serving-cut cross-binding, and redacted projection.

## Artifacts

- adl-runtime/src/distributed/authority_protocol.rs
- adl-runtime/src/distributed/authority_protocol_contract_tests.rs
- adl-runtime/src/distributed/serving_authority.rs
- adl-runtime/tests/distributed_observatory_authority_projection.rs
- adl-runtime/Cargo.toml

## Execution

- Persisted private bounded old/joint quorum eligibility snapshots and committed inclusive deadlines in durable published authority truth
- Denied legacy direct verification from constructing or publishing sealed durable quorum authority
- Added exact RFC 8785 cross-binding between replicated authority artifacts and the verified serving-authority cut
- Added redacted unconstructible Observatory authority projection and focused fail-closed proof
- Kept the existing 52-case authority-protocol compatibility denominator green on replicated sealed publication

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--features",
      "internal-test-fixtures",
      "--lib",
      "authority_protocol",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Run exact authority protocol unit denominator.",
    "outcome": "passed",
    "evidence_ref": "authority-protocol-compatibility.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "HEAD^",
      "HEAD"
    ],
    "purpose": "Run diff check.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--features",
      "internal-test-fixtures",
      "--test",
      "distributed_observatory_authority_projection",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict focused Clippy.",
    "outcome": "passed",
    "evidence_ref": "projection-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--features",
      "internal-test-fixtures",
      "--test",
      "distributed_observatory_authority_projection",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Run focused Observatory authority projection proof.",
    "outcome": "passed",
    "evidence_ref": "projection-focused.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--features",
      "internal-test-fixtures",
      "--test",
      "distributed_observatory_authority_projection",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Prove expanded explicit cross-binding, encoding, restore mutation, integer-bound, and redaction matrix.",
    "outcome": "passed",
    "evidence_ref": "exact remediation run at a2024a186: 7 passed, 0 failed, 0 ignored, 0 filtered"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--features",
      "internal-test-fixtures",
      "--lib",
      "authority_protocol",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Prove real replicated compatibility path and explicit legacy direct publication denial.",
    "outcome": "passed",
    "evidence_ref": "exact remediation run at a2024a186: 53 passed, 0 failed, 0 ignored, 267 filtered"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--features",
      "internal-test-fixtures",
      "--test",
      "distributed_observatory_authority_projection",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warning and API regressions after R1 remediation.",
    "outcome": "passed",
    "evidence_ref": "exact remediation run at a2024a186: finished dev profile with no warnings/errors"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Reject patch whitespace and conflict-marker hygiene defects across the complete candidate.",
    "outcome": "passed",
    "evidence_ref": "exact remediation candidate 7ba526f2 against origin/main: exit 0, no output"
  },
  {
    "command": [
      "cargo",
      "test/clippy/fmt",
      "exact #350 declared lanes at source commit 76215e811783d3b753434fc5300d1ea2bfe55f4d"
    ],
    "purpose": "Close 350-R2B-P1-DURABLE-RESTORE, 350-R2B-P1-EXTRA-SIGNER, 350-R2B-P1-EVIDENCE-TRUTH, and 350-R2B-P2-MATRIX-GAPS with exact source-bound tracked evidence.",
    "outcome": "passed",
    "evidence_ref": "projection-focused=f3d1ba96d376686917db5bf5fe18b0d90c7a3bc6ee26f2e264f4a4ecdd02bb29; authority-protocol=25765175d7fcdaa488f9d3334dd7141b76c997212b5c49b439d1874868f32357; clippy=43cd963d94709fa4b174fa3dbd314977f9480e1abc71e279109909d7a1ed4bd3; diff=af064573279fbea66256552e6e2bce10b4fda7670ac404b7dbe578a9cc97f455"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
