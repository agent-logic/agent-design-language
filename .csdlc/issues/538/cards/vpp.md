# Validation Planning Prompt

Template: 1.0.0

Issue: 538

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/538/design.md

Diagram: .csdlc/prepared/issues/538/diagram.mmd

## Selected Lanes

[
  {
    "lane": "sprint-10-membership",
    "proof_role": "Prove canonical full Sprint 10 membership and order while preserving coordination-only ownership.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/538/validate-sprint10-readiness.sh",
      "membership"
    ],
    "parallel_group": "readiness",
    "defer_reason": null
  },
  {
    "lane": "sprint-10-scoped-readiness",
    "proof_role": "Prove typed readiness for the #523-#526 preparation tranche and report, without synthesizing, absent full-sprint sibling records.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/538/validate-sprint10-readiness.sh",
      "all"
    ],
    "parallel_group": "readiness",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Verify bounded whitespace-clean preparation changes.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 300,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "readiness",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/538/validate-sprint10-readiness.sh membership`
- `bash .csdlc/prepared/issues/538/validate-sprint10-readiness.sh all`
- `git diff --check`

## Failure Semantics

Fail closed on membership drift, dependency drift, missing or generic child prompt truth, omitted open prerequisites, stale authority, or unresolved readiness findings.

## Handoff

Retain typed evidence before convergence.
