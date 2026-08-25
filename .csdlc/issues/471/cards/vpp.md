# Validation Planning Prompt

Template: 1.0.0

Issue: 471

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/471/design.md

Diagram: .csdlc/prepared/issues/471/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-kernel-471",
    "proof_role": "Prove all authoritative wiring, determinism, lifecycle, supervision, health, metrics, quality, and diff contracts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 2520,
    "budget_tokens": 12500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/471/validate-runtime-kernel.sh"
    ],
    "parallel_group": "471-runtime-kernel",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash .csdlc/prepared/issues/471/validate-runtime-kernel.sh`

## Failure Semantics

Fail closed on invalid wiring, undeclared nondeterminism, unbounded queues or waits, restart-policy bypass, silent degradation, stale review, or any failing proof.

## Handoff

Retain typed evidence before convergence.
