# Validation Planning Prompt

Template: 1.0.0

Issue: 446

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/446/design.md

Diagram: .csdlc/prepared/issues/446/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue446-contract",
    "proof_role": "Prove exact issue-owned Runtime and ACC surfaces.",
    "acceptance_ids": [
      "AC-1",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/446/validate_issue446.sh"
    ],
    "parallel_group": "contract",
    "defer_reason": null
  },
  {
    "lane": "resident-tool-governance",
    "proof_role": "Prove single-proposal extraction, authority binding, UTS-to-ACC compilation, gate and adapter denial, receipt redaction, and duplicate-proposal denial.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "resident_tool_execution",
      "--lib",
      "--",
      "--nocapture"
    ],
    "parallel_group": "runtime",
    "defer_reason": null
  },
  {
    "lane": "runtime-full-tick",
    "proof_role": "Prove an actual long-lived Runtime provider StepOutput crosses ACC, Freedom Gate, production adapter dispatch, and exact checkpoint receipt creation.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "long_lived_agent::tests::tick_routes_provider_output_through_runtime_acc_and_adapter",
      "--lib",
      "--",
      "--exact",
      "--nocapture"
    ],
    "parallel_group": "runtime",
    "defer_reason": null
  },
  {
    "lane": "runtime-compile-lint",
    "proof_role": "Prove integrated crate compilation and strict lint cleanliness.",
    "acceptance_ids": [
      "AC-1",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl/Cargo.toml",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "compile",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash .csdlc/prepared/issues/446/validate_issue446.sh`
- `cargo test --manifest-path adl/Cargo.toml resident_tool_execution --lib -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml long_lived_agent::tests::tick_routes_provider_output_through_runtime_acc_and_adapter --lib -- --exact --nocapture`
- `cargo clippy --manifest-path adl/Cargo.toml --lib -- -D warnings`

## Failure Semantics

Fail closed before actuation on parse, authority, compiler, policy, gate, adapter, identity, or receipt errors.

## Handoff

Retain typed evidence before convergence.
