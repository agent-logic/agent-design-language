# Validation Planning Prompt

Template: 1.0.0

Issue: 225

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/225/design.md

Diagram: .csdlc/prepared/issues/225/diagram.mmd

## Selected Lanes

[
  {
    "lane": "prebind-sip-correction",
    "proof_role": "The complete Gate 2 binary proves initialized/ready unbound SIP correction, including migration and authored-drift rejection plus empty actor/reason zero-mutation proof.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 9000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "parallel_group": "225-serial-01",
    "defer_reason": null
  },
  {
    "lane": "recovered-spp-correction",
    "proof_role": "The complete Gate 5 binary proves reviewed, published, and merge-ready recovery; exact recover_review audit plus qualifying transition correspondence; rejection of stale, transition-only, and audit-only provenance; and empty actor/reason zero-mutation behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 9000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5"
    ],
    "parallel_group": "225-serial-02",
    "defer_reason": null
  },
  {
    "lane": "focused-clippy",
    "proof_role": "Reject type, exhaustiveness, and editor owner regressions.",
    "acceptance_ids": [
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "--bin",
      "csdlc-edit",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "225-serial-03",
    "defer_reason": null
  },
  {
    "lane": "format",
    "proof_role": "Require canonical Rust formatting.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--",
      "--check"
    ],
    "parallel_group": "225-serial-04",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace and patch defects across the committed branch range.",
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
      "origin/main...HEAD"
    ],
    "parallel_group": "225-serial-05",
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
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate5`
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --lib --bin csdlc-edit -- -D warnings`
- `cargo fmt --manifest-path csdlc-v2/Cargo.toml -- --check`
- `git diff --check origin/main...HEAD`

## Failure Semantics

Fail closed on wrong phase/card/topology/recovery state, retained lifecycle truth, stale CAS, empty values, incomplete audit, projection drift, or validation/review failure.

## Handoff

Retain typed evidence before convergence.
