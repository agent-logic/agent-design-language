# Validation Planning Prompt

Template: 1.0.0

Issue: 451

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/451/design.md

Diagram: .csdlc/prepared/issues/451/diagram.mmd

## Selected Lanes

[
  {
    "lane": "production_birthday_kernel",
    "proof_role": "Prove prerequisite validation, authenticated ACC binding, exactly-once durable state transitions, every deterministic failpoint, two-instance contention, restart, and duplicate refusal.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "production_birthday",
      "--",
      "--nocapture"
    ],
    "parallel_group": "451-focused-kernel",
    "defer_reason": "Created only after typed bind; the exact issue-owned test must exist and pass before implementation finalize."
  },
  {
    "lane": "production_birthday_resident_path",
    "proof_role": "Prove the actual long-lived resident denial, activation, restart, duplicate denial, governed continuation, and unchanged ordinary-resident path through merged authorities.",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--test",
      "production_birthday_runtime",
      "--",
      "--nocapture"
    ],
    "parallel_group": "451-focused-resident",
    "defer_reason": "Created only after typed bind with the long-lived Runtime adapter; missing or zero selected tests fail closed before finalize."
  },
  {
    "lane": "runtime_feature_wiring_audit",
    "proof_role": "Require all nine feature rows to bind construction, production consumption, behavioral and negative proof, exact source revision, and a live disposition.",
    "acceptance_ids": [
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/451/validate_runtime_feature_wiring.rb"
    ],
    "parallel_group": "451-audit",
    "defer_reason": "Created after typed bind from the exact candidate source inventory; it must reject every non-live audit disposition before finalize."
  },
  {
    "lane": "retained_evidence_contract",
    "proof_role": "Validate the retained audit and birthday evidence against exact schemas, repository-relative path policy, and private-state, prompt, provider, and tool-payload redaction rules.",
    "acceptance_ids": [
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/451/validate_retained_evidence.rb"
    ],
    "parallel_group": "451-evidence",
    "defer_reason": "Created after typed bind with retained evidence schemas and fixtures; absence or redaction failure blocks finalize."
  },
  {
    "lane": "rust_format_runtime_kernel",
    "proof_role": "Reject formatting drift in the changed Runtime-kernel crate.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--",
      "--check"
    ],
    "parallel_group": "451-quality-format-kernel",
    "defer_reason": null
  },
  {
    "lane": "rust_format_adl",
    "proof_role": "Reject formatting drift in the changed ADL Runtime integration crate.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl/Cargo.toml",
      "--",
      "--check"
    ],
    "parallel_group": "451-quality-format-adl",
    "defer_reason": null
  },
  {
    "lane": "strict_clippy_runtime_kernel",
    "proof_role": "Reject warnings across all targets in the changed Runtime-kernel crate.",
    "acceptance_ids": [
      "AC-1",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "451-quality-kernel",
    "defer_reason": null
  },
  {
    "lane": "strict_clippy_adl",
    "proof_role": "Reject warnings across all targets in the changed ADL Runtime integration crate.",
    "acceptance_ids": [
      "AC-1",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "451-quality-adl",
    "defer_reason": null
  },
  {
    "lane": "diff_hygiene",
    "proof_role": "Reject malformed or whitespace-damaged candidate changes.",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
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
    "parallel_group": "451-quality-diff",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test production_birthday -- --nocapture`
- `cargo test --locked --manifest-path adl/Cargo.toml --test production_birthday_runtime -- --nocapture`
- `ruby .csdlc/prepared/issues/451/validate_runtime_feature_wiring.rb`
- `ruby .csdlc/prepared/issues/451/validate_retained_evidence.rb`
- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml -- --check`
- `cargo fmt --manifest-path adl/Cargo.toml -- --check`
- `cargo clippy --locked --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`
- `cargo clippy --locked --manifest-path adl/Cargo.toml --all-targets -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed. Do not publish or claim a first production birthday when any dependency, cross-binding, exactly-once/restart case, renewed audit row, redaction check, or exact-head review is incomplete.

## Handoff

Retain typed evidence before convergence.
