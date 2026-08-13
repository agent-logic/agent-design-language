# Validation Planning Prompt

Template: 1.0.0

Issue: 274

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/274/design.md

Diagram: .csdlc/prepared/issues/274/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Pre-bind packet identity and terminal ancestry through #358; #360 terminal ancestry is separately proven by the immutable implementation base.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 2000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/274/validate_preparation_bundle.py"
    ],
    "parallel_group": "274-serial-01",
    "defer_reason": "Pre-bind-only evidence; not rerun after bind."
  },
  {
    "lane": "observatory-transition-unit",
    "proof_role": "Prove the private state guard and named normalized final-state digest recomputation/tamper denial.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "observatory_serving_eligibility::tests",
      "--features",
      "internal-test-fixtures",
      "--",
      "--test-threads=1"
    ],
    "parallel_group": "274-serial-02a",
    "defer_reason": null
  },
  {
    "lane": "observatory-focused",
    "proof_role": "Through terminal #360 fixtures, prove authentic sealed Acquire/Renew/Transfer/Revoke, restart, overlap, stale/superseded predecessor, revoked revival, nanos expiry, replay, A/B mismatch, and redaction.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 18000,
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
    "parallel_group": "274-serial-02",
    "defer_reason": null
  },
  {
    "lane": "observatory-clippy",
    "proof_role": "Reject warnings in the exact feature-bearing target.",
    "acceptance_ids": [
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
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
    "parallel_group": "274-serial-03",
    "defer_reason": null
  },
  {
    "lane": "exact-scope",
    "proof_role": "Prove exact two new product paths and one additive mod.rs declaration against immutable #360 merge base.",
    "acceptance_ids": [
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 2000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/274/validate_scope.py"
    ],
    "parallel_group": "274-serial-04",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace errors across the immutable #360 implementation-base diff.",
    "acceptance_ids": [
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check",
      "dae957c435b73d87af1f36d4e15fb088f6fd055b...HEAD"
    ],
    "parallel_group": "274-serial-05",
    "defer_reason": null
  },
  {
    "lane": "terminal-ancestry",
    "proof_role": "After typed finish, prove canonical merged terminal cache and merge ancestry to origin/main.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/274/validate_terminal.py"
    ],
    "parallel_group": "274-serial-06",
    "defer_reason": "Deferred until typed finish creates terminal authority."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `python3 .csdlc/prepared/issues/274/validate_preparation_bundle.py`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib observatory_serving_eligibility::tests --features internal-test-fixtures -- --test-threads=1`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_observatory_serving_eligibility --features internal-test-fixtures -- --test-threads=1`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_observatory_serving_eligibility --features internal-test-fixtures -- -D warnings`
- `python3 .csdlc/prepared/issues/274/validate_scope.py`
- `git diff --check dae957c435b73d87af1f36d4e15fb088f6fd055b...HEAD`
- `python3 .csdlc/prepared/issues/274/validate_terminal.py`

## Failure Semantics

Fail closed on stale or nonancestral authority, wrong quorum/lease/OwnerCommit/fence/generation/receipt, overlapping transfer, replay conflict, restart ambiguity, revoked or expired revival, redaction leak, scope drift, premature bind, review finding, CI failure, or noncanonical terminal ancestry.

## Handoff

Retain typed evidence before convergence.
