# Validation Planning Prompt

Template: 1.0.0

Issue: 513

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/513/design.md

Diagram: .csdlc/prepared/issues/513/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-authority-topology",
    "proof_role": "Validate DEC-01 source ownership, reverse-reference dispositions, compatibility command, migration dry-run, rollback dry-run, and Runtime v4 exclusion.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "docs/milestones/v0.92.1/evidence/runtime-decoupling/validate-runtime-authority-topology.sh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Validate whitespace hygiene for tracked DEC-01 changes.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
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
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash docs/milestones/v0.92.1/evidence/runtime-decoupling/validate-runtime-authority-topology.sh`
- `git diff --check`

## Failure Semantics

Fail closed on unowned source roots, unclassified reverse references, missing compatibility proof, non-executable migration or rollback dry-run, Runtime v4 authority expansion, stale review, red CI, or lifecycle topology drift.

## Handoff

Retain typed evidence before convergence.
