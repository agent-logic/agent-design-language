# Structured Output Record

Template: 1.0.0

Issue: 367

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Bind sealed Shepherd and Observatory committed projections to one verifier-derived authority lineage and return an opaque borrowed verified pair.

## Artifacts

- adl-runtime/src/distributed/serving_authority.rs
- adl-runtime/src/distributed/shepherd_serving_eligibility.rs
- adl-runtime/tests/distributed_shepherd_serving_eligibility.rs
- adl-runtime/tests/distributed_observatory_serving_eligibility.rs

## Execution

- Derive the exact existing redacted lineage reference from VerifiedServingAuthorityCut and bind it through Shepherd durable state receipts and sealed provenance
- Return a privately constructed borrowed VerifiedCommittedChildLineagePair over the exact sealed children
- Fail closed on legacy missing lineage malformed provenance and two genuine different-lineage stores before first use and after reopen

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check",
      "a4801fbb3a58bed27ba53367cbda8b31a1f56083...HEAD"
    ],
    "purpose": "Diff check immutable #365 base.",
    "outcome": "passed",
    "evidence_ref": "diff.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--features",
      "internal-test-fixtures",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict feature-bearing library Clippy.",
    "outcome": "passed",
    "evidence_ref": "lib-clippy.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_observatory_serving_eligibility",
      "--features",
      "internal-test-fixtures",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict exact target Clippy.",
    "outcome": "passed",
    "evidence_ref": "observatory-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_observatory_serving_eligibility",
      "--features",
      "internal-test-fixtures",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Full Observatory target, 7/7.",
    "outcome": "passed",
    "evidence_ref": "observatory-integration.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--doc"
    ],
    "purpose": "Normal-build compile-fail docs, 3/3.",
    "outcome": "passed",
    "evidence_ref": "rustdoc.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_shepherd_serving_eligibility",
      "--features",
      "internal-test-fixtures",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict exact target Clippy.",
    "outcome": "passed",
    "evidence_ref": "shepherd-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_shepherd_serving_eligibility",
      "--features",
      "internal-test-fixtures",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Full Shepherd target, 9/9.",
    "outcome": "passed",
    "evidence_ref": "shepherd-integration.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "shepherd_serving_eligibility::tests::sealed_committed_projection_private_provenance",
      "--features",
      "internal-test-fixtures"
    ],
    "purpose": "Exact private unit filter, 1/1.",
    "outcome": "passed",
    "evidence_ref": "shepherd-private-unit.log"
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
