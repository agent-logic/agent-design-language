# Validation Planning Prompt

Template: 1.0.0

Issue: 5765

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5765/design.md

Diagram: .csdlc/prepared/issues/5765/diagram.mmd

## Selected Lanes

[
  {
    "lane": "docs-planning-focused",
    "proof_role": "Validate YAML structure, reference text, and diff scope",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `git diff --check`

## Failure Semantics

Fail closed if the reference is missing, the scope widens, or the edit implies migration authorization.

## Handoff

Retain typed evidence before convergence.
