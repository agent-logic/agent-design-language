# Validation Planning Prompt

Template: 1.0.0

Issue: 431

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/431/design.md

Diagram: .csdlc/prepared/issues/431/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Verify exact preparation artifacts and authority boundaries before binding.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-5",
      "AC-7",
      "AC-9",
      "AC-10",
      "AC-11"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 600,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/431/validate_preparation_bundle.py"
    ],
    "parallel_group": "431-serial-01",
    "defer_reason": null
  },
  {
    "lane": "planning-package",
    "proof_role": "Validate per-surface six-lane parity, #432 ordering, path-bearing .adl dependency absence, WP-28 immutability, CodeFriend handoff, Runtime v4 non-scope, Observatory grounding, YAML structure, links, placeholders, exact scope, and live v0.92.1 routing.",
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
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/431/validate_planning_package.py"
    ],
    "parallel_group": "431-serial-02",
    "defer_reason": "Runs after the refreshed tracked planning package exists."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed or unintended planning-package changes.",
    "acceptance_ids": [
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 200,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "431-serial-03",
    "defer_reason": "Runs after the implementation diff exists."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 .csdlc/prepared/issues/431/validate_preparation_bundle.py`
- `python3 .csdlc/prepared/issues/431/validate_planning_package.py`
- `git diff --check`

## Failure Semantics

Fail closed on package denominator drift, tracked/backlog ambiguity, unsupported terminal claims, WP-28 authority overlap, invalid YAML/links/templates, or unreviewed issue migration.

## Handoff

Retain typed evidence before convergence.
