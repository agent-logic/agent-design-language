# Validation Planning Prompt

Template: 1.0.0

Issue: 554

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/554/authored/design.md

Diagram: .csdlc/issues/554/authored/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-memory-palace-docs",
    "proof_role": "AC-1 docs invariant",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "--test",
      "memory_palace_tests",
      "v092_docs_name_memory_palace_production_authority_without_broad_completion_claim"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "focused-runtime-v2-kernel",
    "proof_role": "AC-2 timeout reliability",
    "acceptance_ids": [
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "--lib",
      "runtime_v2::tests::unified_runtime_kernel"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "typed-issue-validation",
    "proof_role": "AC-3 lifecycle/card validation before review and publication; hosted required checks remain fail-closed after publication.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-554-v0-92-1-shared-gate-coverage-baseline",
      "issue",
      "--issue",
      "554"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path adl/Cargo.toml --test memory_palace_tests v092_docs_name_memory_palace_production_authority_without_broad_completion_claim`
- `cargo test --manifest-path adl/Cargo.toml --lib runtime_v2::tests::unified_runtime_kernel`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate --root /Volumes/FastWork/adl-worktrees/adl-issue-554-v0-92-1-shared-gate-coverage-baseline issue --issue 554`

## Failure Semantics

Fail closed and preserve logs; do not rerun #549 until #554 is green and merged.

## Handoff

Retain typed evidence before convergence.
