# Validation Planning Prompt

Template: 1.0.0

Issue: 365

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/365/design.md

Diagram: .csdlc/prepared/issues/365/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove exact base canonical terminal ancestry four-path ownership and no-policy boundary.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/365/validate_preparation.py"
    ],
    "parallel_group": "365-serial-01",
    "defer_reason": null
  },
  {
    "lane": "shepherd-private-unit",
    "proof_role": "Run the exact named nonzero in-source Shepherd private provenance verifier matrix for A/B wrong-kind corrupt stale generation/index and reopen truth.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4500,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--features",
      "internal-test-fixtures",
      "distributed::shepherd_serving_eligibility::tests::sealed_committed_projection_private_provenance",
      "--",
      "--exact",
      "--test-threads=1"
    ],
    "parallel_group": "365-serial-02",
    "defer_reason": "Deferred until bound source-unit test exists; zero tests fail closed."
  },
  {
    "lane": "observatory-private-unit",
    "proof_role": "Run the exact named nonzero in-source Observatory private provenance verifier matrix for A/B wrong-kind corrupt stale generation/index and reopen truth.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4500,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--features",
      "internal-test-fixtures",
      "distributed::observatory_serving_eligibility::tests::sealed_committed_projection_private_provenance",
      "--",
      "--exact",
      "--test-threads=1"
    ],
    "parallel_group": "365-serial-03",
    "defer_reason": "Deferred until bound source-unit test exists; zero tests fail closed."
  },
  {
    "lane": "opaque-api-compile-fail",
    "proof_role": "Execute normal-build rustdoc compile-fail examples proving both opaque public types deny struct-literal and private-constructor creation without exposing construction details.",
    "acceptance_ids": [
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--doc",
      "--features",
      "internal-test-fixtures"
    ],
    "parallel_group": "365-serial-04",
    "defer_reason": "Deferred until opaque type API docs exist; missing compile-fail examples fail review."
  },
  {
    "lane": "shepherd-focused",
    "proof_role": "Run the exact authentic feature-bearing Shepherd store lifecycle target including sealed construction restart corruption and redaction while retaining the full prior matrix.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 6500,
    "argv": [
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
    "parallel_group": "365-serial-05",
    "defer_reason": "Deferred until bound implementation; missing or zero tests fail closed."
  },
  {
    "lane": "observatory-focused",
    "proof_role": "Run the exact authentic feature-bearing Observatory store lifecycle target including sealed construction restart corruption and redaction while retaining the full prior matrix.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 6500,
    "argv": [
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
    "parallel_group": "365-serial-06",
    "defer_reason": "Deferred until bound implementation; missing or zero tests fail closed."
  },
  {
    "lane": "library-clippy",
    "proof_role": "Reject warnings and API misuse across both changed library and source-unit surfaces under the feature-bearing build.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
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
    "parallel_group": "365-serial-07",
    "defer_reason": "Deferred until implementation."
  },
  {
    "lane": "shepherd-clippy",
    "proof_role": "Reject warnings in the exact changed Shepherd integration target.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
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
    "parallel_group": "365-serial-08",
    "defer_reason": "Deferred until implementation."
  },
  {
    "lane": "observatory-clippy",
    "proof_role": "Reject warnings in the exact changed Observatory integration target.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
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
    "parallel_group": "365-serial-09",
    "defer_reason": "Deferred until implementation."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject patch hygiene outside the immutable #274 terminal base.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check",
      "26de2a048cea436e5140a8ab5afa7524324b3b39...HEAD"
    ],
    "parallel_group": "365-serial-10",
    "defer_reason": "Deferred until implementation revision."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `python3 .csdlc/prepared/issues/365/validate_preparation.py`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib --features internal-test-fixtures distributed::shepherd_serving_eligibility::tests::sealed_committed_projection_private_provenance -- --exact --test-threads=1`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib --features internal-test-fixtures distributed::observatory_serving_eligibility::tests::sealed_committed_projection_private_provenance -- --exact --test-threads=1`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --doc --features internal-test-fixtures`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_shepherd_serving_eligibility --features internal-test-fixtures -- --test-threads=1`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_observatory_serving_eligibility --features internal-test-fixtures -- --test-threads=1`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --lib --features internal-test-fixtures -- -D warnings`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_shepherd_serving_eligibility --features internal-test-fixtures -- -D warnings`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_observatory_serving_eligibility --features internal-test-fixtures -- -D warnings`
- `git diff --check 26de2a048cea436e5140a8ab5afa7524324b3b39...HEAD`

## Failure Semantics

Fail closed on caller construction raw authority exposure unverified digest stale/corrupt checkpoint A/B substitution restart drift policy change scope drift review finding CI failure or terminal mismatch.

## Handoff

Retain typed evidence before convergence.
