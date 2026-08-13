# Validation Planning Prompt

Template: 1.0.0

Issue: 260

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/260/design.md

Diagram: .csdlc/prepared/issues/260/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-distributed-caller-compile",
    "proof_role": "Compile proof for migrated distributed Runtime callers",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1200,
    "argv": [
      "cargo",
      "check",
      "--manifest-path",
      "adl-runtime/Cargo.toml"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-distributed-authority-adapter-callers-260",
    "proof_role": "Issue-owned focused harness for #260 distributed Runtime caller migration after #259 terminal",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 360,
    "budget_tokens": 1800,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_authority_adapter_callers_260",
      "--no-tests=fail"
    ],
    "parallel_group": "local",
    "defer_reason": "The issue-owned post-#259 harness adl-runtime/tests/distributed_authority_adapter_callers_260.rs must be created in the #260 bound worktree after #259 is terminal and ancestral; pre-bind preparation records this denominator without creating source/test files on main."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo check --manifest-path adl-runtime/Cargo.toml`
- `cargo test --manifest-path adl-runtime/Cargo.toml --test distributed_authority_adapter_callers_260 --no-tests=fail`

## Failure Semantics

Fail closed on #259 nonterminal gate, scope overlap with #258/#259/#203, stale main, validation failure, stale review, or typed publication mismatch.

## Handoff

Retain typed evidence before convergence.
