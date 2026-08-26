# Validation Planning Prompt

Template: 1.0.0

Issue: 343

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/343/design.md

Diagram: .csdlc/prepared/issues/343/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Validate the exact coordination-only scope, canonical child graph, exclusions, handoff boundary, and authored bundle.",
    "acceptance_ids": [
      "AC-3",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/343/validate_preparation_bundle.py"
    ],
    "parallel_group": "343-serial-01-preparation",
    "defer_reason": null
  },
  {
    "lane": "terminal-child-census",
    "proof_role": "Fail closed unless retained typed observations prove #256 and #341 terminal/canonical/ancestral and historical WP-17/WP-19 validation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/343/validate_sprint_readiness.py",
      "--terminal"
    ],
    "parallel_group": "343-serial-02-terminal",
    "defer_reason": "Deferred while #256 and #341 remain open."
  },
  {
    "lane": "sprint-packet-validation",
    "proof_role": "Validate exact issue/PR/revision/cache/ancestry/review/demo/artifact/release-truth fields and reject stale, fixture-only, or unsupported claims.",
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
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/343/validate_sprint_readiness.py",
      "--packet"
    ],
    "parallel_group": "343-serial-03-packet",
    "defer_reason": "Deferred until terminal child evidence and the sprint packet exist."
  },
  {
    "lane": "exact-scope-and-diff-hygiene",
    "proof_role": "Reject paths outside the exact #343 lifecycle, evidence, and sprint packet boundary and inspect committed, staged, unstaged, and untracked content for diff hygiene.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 200,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/343/validate_exact_scope.py"
    ],
    "parallel_group": "343-serial-04-scope",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 .csdlc/prepared/issues/343/validate_preparation_bundle.py`
- `python3 .csdlc/prepared/issues/343/validate_sprint_readiness.py --terminal`
- `python3 .csdlc/prepared/issues/343/validate_sprint_readiness.py --packet`
- `python3 .csdlc/prepared/issues/343/validate_exact_scope.py`

## Failure Semantics

Fail closed on dependency, authority, ancestry, evidence, redaction, scope, review, publication, or terminal mismatch; do not repair child work from #343.

## Handoff

Retain typed evidence before convergence.
