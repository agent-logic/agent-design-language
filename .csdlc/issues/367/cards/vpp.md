# Validation Planning Prompt

Template: 1.0.0

Issue: 367

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/367/design.md

Diagram: .csdlc/prepared/issues/367/diagram.mmd

## Selected Lanes

[
  {
    "lane": "shepherd-private-unit",
    "proof_role": "Run the exact source-unit filter proving missing lineage denial, sealed preimage tamper rejection, and private pair-adapter construction/accessor invariants.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
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
    "parallel_group": "367-serial-01",
    "defer_reason": "Deferred until adapter implementation; zero tests fail closed."
  },
  {
    "lane": "normal-build-api-denial",
    "proof_role": "Run normal-build rustdoc compile-fail examples proving sealed DTO/raw lineage and pair-adapter struct construction are unavailable.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--doc"
    ],
    "parallel_group": "367-serial-02",
    "defer_reason": "Deferred until adapter compile-fail docs exist; zero tests fail closed."
  },
  {
    "lane": "shepherd-focused",
    "proof_role": "Run the exact feature-bearing Shepherd target proving verifier-derived lineage persistence, replacement/end preservation, legacy denial, redaction, and unchanged lifecycle behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
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
    "parallel_group": "367-serial-04",
    "defer_reason": "Deferred until implementation; missing or zero tests fail closed."
  },
  {
    "lane": "observatory-focused",
    "proof_role": "Run the exact feature-bearing Observatory target proving opaque adapter A/A success, authentic A/B first-use/restart denial, exact-child borrows, and redaction.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
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
    "parallel_group": "367-serial-05",
    "defer_reason": "Deferred until implementation; missing or zero tests fail closed."
  },
  {
    "lane": "library-clippy",
    "proof_role": "Reject warnings and hidden API drift across the feature-bearing library and private verifier/adapter surfaces.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 5000,
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
    "parallel_group": "367-serial-06",
    "defer_reason": "Deferred until implementation."
  },
  {
    "lane": "shepherd-clippy",
    "proof_role": "Reject warnings in the exact feature-bearing Shepherd target.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 5000,
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
    "parallel_group": "367-serial-07",
    "defer_reason": "Deferred until implementation."
  },
  {
    "lane": "observatory-clippy",
    "proof_role": "Reject warnings in the exact feature-bearing Observatory target.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 5000,
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
    "parallel_group": "367-serial-08",
    "defer_reason": "Deferred until implementation."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject patch hygiene outside the immutable #365 terminal base.",
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
      "a4801fbb3a58bed27ba53367cbda8b31a1f56083...HEAD"
    ],
    "parallel_group": "367-serial-09",
    "defer_reason": "Deferred until implementation revision."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib shepherd_serving_eligibility::tests::sealed_committed_projection_private_provenance --features internal-test-fixtures`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --doc`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_shepherd_serving_eligibility --features internal-test-fixtures -- --test-threads=1`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_observatory_serving_eligibility --features internal-test-fixtures -- --test-threads=1`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --lib --features internal-test-fixtures -- -D warnings`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_shepherd_serving_eligibility --features internal-test-fixtures -- -D warnings`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_observatory_serving_eligibility --features internal-test-fixtures -- -D warnings`
- `git diff --check a4801fbb3a58bed27ba53367cbda8b31a1f56083...HEAD`

## Failure Semantics

Fail closed on raw or caller pairing authority missing lineage unverified/corrupt provenance authentic A/B substitution restart drift policy change scope drift review finding CI failure or terminal mismatch.

## Handoff

Retain typed evidence before convergence.
