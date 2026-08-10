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
    "proof_role": "Verify required v0.92.1 planning and feature surfaces, issue identifiers, source routing, accepted v3 source revision, terminal Runtime source authority, and planning posture.",
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
    "proof_role": "Verify issue-wave YAML, local links, placeholders, and every intended tracked or untracked publication path before commit.",
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
  },
  {
    "lane": "bounded-independent-review",
    "proof_role": "Obtain an independent source-grounded review of milestone completeness, dependency truth, source pins, and proof quality before publication.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 8000,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-review",
      "--root",
      ".",
      "--request",
      ".csdlc/prepared/issues/146/review-request.json"
    ],
    "parallel_group": "pre-publication-review",
    "defer_reason": "Runs after the milestone package and focused deterministic validation are complete at an exact review revision."
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
- `.adl/bin/csdlc-v2/csdlc-review --root . --request .csdlc/prepared/issues/146/review-request.json`

## Failure Semantics

Fail closed on missing scope, contradictory dependencies, unsupported proof claims, invalid YAML or links, stale source routing, or unresolved independent-review findings.

## Handoff

Retain typed evidence before convergence.
