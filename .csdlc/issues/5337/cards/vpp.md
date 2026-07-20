# Validation Planning Prompt

Template: 1.0.0

Issue: 5337

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5337/design.md

Diagram: .csdlc/prepared/issues/5337/diagram.mmd

## Selected Lanes

[
  {
    "lane": "characterization-unit-and-integration",
    "proof_role": "Prove manifest/schema, runner, normalization, comparison, coverage, and negative safety contracts",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-characterization/Cargo.toml",
      "--all-targets"
    ],
    "parallel_group": "local-rust",
    "defer_reason": null
  },
  {
    "lane": "pinned-v1-corpus-verification",
    "proof_role": "Verify all retained repeated observations and the complete coverage map against the versioned corpus",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "run",
      "--manifest-path",
      "adl-characterization/Cargo.toml",
      "--bin",
      "adl-characterize",
      "--",
      "verify",
      "--corpus",
      "adl-characterization/corpus/v1/corpus.yaml",
      "--observations",
      "adl-characterization/observations/v1"
    ],
    "parallel_group": "local-proof",
    "defer_reason": null
  },
  {
    "lane": "format-and-lint",
    "proof_role": "Prove the standalone crate is formatted and warning-free under strict Clippy",
    "acceptance_ids": [
      "AC-1",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-characterization/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "local-rust",
    "defer_reason": null
  },
  {
    "lane": "typed-review-and-publication",
    "proof_role": "Prove exact-revision review, resolved findings, lifecycle integrity, and publication readiness",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 2000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5337"
    ],
    "parallel_group": "local-review",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path adl-characterization/Cargo.toml --all-targets`
- `cargo run --manifest-path adl-characterization/Cargo.toml --bin adl-characterize -- verify --corpus adl-characterization/corpus/v1/corpus.yaml --observations adl-characterization/observations/v1`
- `cargo clippy --manifest-path adl-characterization/Cargo.toml --all-targets -- -D warnings`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5337`

## Failure Semantics

Fail closed on non-current template provenance, invalid cards, generic or implementation-claiming prose, shared-path overlap, missing #5336 dependency, hidden normalization, skipped review, AWS/raw-gh use, or any product-scope execution.

## Handoff

Retain typed evidence before convergence.
