# Structured Output Record

Template: 1.0.0

Issue: 350

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

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
