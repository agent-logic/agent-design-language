# Validation Planning Prompt

Template: 1.0.0

Issue: 689

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/689/design.md

Diagram: .csdlc/prepared/issues/689/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-control-routing-docs",
    "proof_role": "Prove shell syntax, canonical route guidance, and refusal of all legacy Runtime routes, including open and empty invocation, while retaining Observatory commands.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 600,
    "argv": [
      "bash",
      "adl/tools/test_csmctl_linux_backend.sh"
    ],
    "parallel_group": "689-focused",
    "defer_reason": null
  },
  {
    "lane": "canonical-runtime-ownership",
    "proof_role": "Run the actual focused Rust ownership, convergence, status, and transactional reload tests for the unchanged canonical controller.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1200,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl",
      "csm_runtime_v3_cmd::tests::"
    ],
    "parallel_group": "689-focused",
    "defer_reason": null
  },
  {
    "lane": "exact-range-diff",
    "proof_role": "Reject whitespace and conflict-marker defects across the exact issue range.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 200,
    "argv": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "parallel_group": "689-quality",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_csmctl_linux_backend.sh`
- `cargo test --locked --manifest-path adl/Cargo.toml --bin adl csm_runtime_v3_cmd::tests::`
- `git diff --check origin/main...HEAD`

## Failure Semantics

Fail closed when a legacy Runtime lifecycle verb is used and print the exact canonical replacement without touching a service.

## Handoff

Retain typed evidence before convergence.
