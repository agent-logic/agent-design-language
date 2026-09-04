# Validation Planning Prompt

Template: 1.0.0

Issue: 674

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/674/design.md

Diagram: .csdlc/prepared/issues/674/diagram.mmd

## Selected Lanes

[
  {
    "lane": "welcome-package-docs",
    "proof_role": "Check that the versioned document exists; contains required orientation, governance, safety, help, welcome, and support markers; and excludes specified secret, host-path, lore, personhood, and capability-fantasy markers. Exact-head review confirms qualitative tone in context.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 800,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/674/validate-welcome-package-docs.sh"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed whitespace and patch artifacts.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
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

- `bash .csdlc/prepared/issues/674/validate-welcome-package-docs.sh`
- `git diff --check origin/main...HEAD`

## Failure Semantics

Fail closed on unsupported capability claims, missing governance conditions, secret or host-path content, missing required sections, or scope beyond documentation.

## Handoff

Retain typed evidence before convergence.
