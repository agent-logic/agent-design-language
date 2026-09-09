# Validation Planning Prompt

Template: 1.0.0

Issue: 761

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/761/design.md

Diagram: .csdlc/prepared/issues/761/diagram.mmd

## Selected Lanes

[
  {
    "lane": "packet-denominator-contract",
    "proof_role": "Deterministic Python routing and validation contract proof; not a release gate.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      "adl/tools/skills/repo-packet-builder/tests/test_denominators.py",
      "-v"
    ],
    "parallel_group": "local",
    "defer_reason": "Validator is implemented in bound issue worktree; unavailable in inspection checkout until merge."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `python3 adl/tools/skills/repo-packet-builder/tests/test_denominators.py -v`

## Failure Semantics

Fail closed on incomplete test proof, denominator corruption or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
