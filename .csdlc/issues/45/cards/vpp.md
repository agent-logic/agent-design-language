# Validation Planning Prompt

Template: 1.0.0

Issue: 45

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/45/design.md

Diagram: .csdlc/prepared/issues/45/diagram.mmd

## Selected Lanes

[
  {
    "lane": "doctor-repository-identity",
    "proof_role": "Prove same-repository, declared split, absent origin, unparseable origin, mismatched identity, and rebind substitution behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "legacy-record-receipt-compatibility",
    "proof_role": "Prove missing code_repository preserves pre-field index and retained terminal-receipt digest bytes.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "absent_code_repository_preserves_pre_field_record_and_receipt_digests"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "typed-card-validation",
    "proof_role": "Run the source-built typed validator against issue 45 canonical cards and lifecycle state.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "run",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--bin",
      "csdlc-validate",
      "--",
      "--root",
      ".",
      "issue",
      "--issue",
      "45"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "public-schema-contract",
    "proof_role": "Prove the public publication schema remains available and terminal mutation schemas remain absent.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate6",
      "public_schema_keeps_publication_and_drops_merged_reconciliation"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "doctor-contract-and-lint",
    "proof_role": "Strict Clippy proves warning-free schema, lifecycle, guidance, and regression changes across all targets.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate2`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml absent_code_repository_preserves_pre_field_record_and_receipt_digests`
- `cargo run --locked --manifest-path csdlc-v2/Cargo.toml --bin csdlc-validate -- --root . issue --issue 45`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate6 public_schema_keeps_publication_and_drops_merged_reconciliation`
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed on missing or ambiguous repository identity, unexpected effective remote drift, stale typed state, failed three-case proof, or obsolete active guidance.

## Handoff

Retain typed evidence before convergence.
