# Validation Planning Prompt

Template: 1.0.0

Issue: 350

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/350/design.md

Diagram: .csdlc/prepared/issues/350/diagram.mmd

## Selected Lanes

[
  {
    "lane": "projection-focused",
    "proof_role": "Prove exact sealed projection; canonical and noncanonical encoding vectors; A/B by A/B cross-pair matrix; signer, configuration, threshold, and joint-quorum restore mutation; deadline/restart/legacy-state handling; and redaction.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 16000,
    "argv": [
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
    "parallel_group": "350-serial-01",
    "defer_reason": null
  },
  {
    "lane": "authority-protocol-compatibility",
    "proof_role": "Keep the existing 52-case authority-protocol denominator green while fixtures exercise replicated sealed publication; legacy direct verification may not synthesize durable quorum authority.",
    "acceptance_ids": [
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 16000,
    "argv": [
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
    "parallel_group": "350-serial-02",
    "defer_reason": null
  },
  {
    "lane": "projection-clippy",
    "proof_role": "Reject warnings and API misuse for the exact focused target.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
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
    "parallel_group": "350-serial-03",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject patch hygiene defects before exact-head review.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "350-serial-04",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --features internal-test-fixtures --test distributed_observatory_authority_projection -- --test-threads=1`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --features internal-test-fixtures --lib authority_protocol -- --test-threads=1`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --features internal-test-fixtures --test distributed_observatory_authority_projection -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed on caller authority, provenance mismatch, quorum/deadline ambiguity, stale/corrupt/legacy restore authority, redaction leak, scope drift, failed proof, stale review, red CI, or nonterminal finish.

## Handoff

Retain typed evidence before convergence.
