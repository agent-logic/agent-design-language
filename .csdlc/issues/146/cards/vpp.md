# Validation Planning Prompt

Template: 1.0.0

Issue: 146

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/146/design.md

Diagram: .csdlc/prepared/issues/146/diagram.mmd

## Selected Lanes

[
  {
    "lane": "milestone-package-contract",
    "proof_role": "Verify planning-only posture, the preserved work-package denominator, WP-01 creation authority, retirement truth, lifecycle sequence, dependency graph, and all eleven C-SDLC v3 decisions.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/146/validate-v0921-package.rb"
    ],
    "parallel_group": "focused-docs",
    "defer_reason": null
  },
  {
    "lane": "yaml-link-and-complete-delta-contract",
    "proof_role": "Verify milestone YAML, repository links, and placeholder hygiene across the corrected publication delta.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/146/validate-v0921-links.rb"
    ],
    "parallel_group": "focused-docs",
    "defer_reason": null
  },
  {
    "lane": "committed-diff-hygiene",
    "proof_role": "Reject malformed changes across the complete committed publication delta against the exact base revision.",
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
    "parallel_group": "focused-docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/146/validate-v0921-package.rb`
- `ruby .csdlc/prepared/issues/146/validate-v0921-links.rb`
- `git diff --check origin/main...HEAD`

## Failure Semantics

Fail closed on missing scope, contradictory dependencies, unsupported proof claims, invalid YAML or links, stale source routing, or unresolved independent-review findings.

## Handoff

Retain typed evidence before convergence.
