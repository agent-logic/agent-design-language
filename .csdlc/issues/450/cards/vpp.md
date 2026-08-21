# Validation Planning Prompt

Template: 1.0.0

Issue: 450

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/450/design.md

Diagram: .csdlc/prepared/issues/450/diagram.mmd

## Selected Lanes

[
  {
    "lane": "memory_palace_kernel_tests",
    "proof_role": "Proves deterministic authority construction, digest validation, and fail-closed rejection semantics.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "memory_palace"
    ],
    "parallel_group": "local-focused",
    "defer_reason": null
  },
  {
    "lane": "memory_palace_runtime_resident_tests",
    "proof_role": "Runs the existing adl-runtime library target including issue-owned resident_memory tests for raw production authority provisioning, serialized generation commits, rollback-safe restart, cache consumption, and negative cases.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib"
    ],
    "parallel_group": "local-focused",
    "defer_reason": null
  },
  {
    "lane": "memory_palace_long_lived_consumer_tests",
    "proof_role": "Executes the unchanged public compatibility path with a Runtime-produced packet, proves exact legacy projection and no-config/rejection behavior, and assertion-checks single-authority feature/evidence truth and old-surface dispositions.",
    "acceptance_ids": [
      "AC-2",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "--test",
      "memory_palace_tests"
    ],
    "parallel_group": "local-focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test memory_palace`
- `cargo test --manifest-path adl-runtime/Cargo.toml --lib`
- `cargo test --manifest-path adl/Cargo.toml --test memory_palace_tests`

## Failure Semantics

Fail closed. Do not publish or claim Memory Palace production convergence unless the actual resident path consumes the kernel authority and restart/divergence proof passes at the reviewed head.

## Handoff

Retain typed evidence before convergence.
