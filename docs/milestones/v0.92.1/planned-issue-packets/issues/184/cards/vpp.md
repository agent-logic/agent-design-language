# Validation Planning Prompt

Template: 1.0.0

Issue: 184

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/184/design.md

Diagram: .csdlc/prepared/issues/184/diagram.mmd

## Selected Lanes

[
  {
    "lane": "drt-04-outcome-contract",
    "proof_role": "Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/184/validate-outcome.rb"
    ],
    "parallel_group": "drt-04-outcome-contract",
    "defer_reason": "The issue-delivered validator is authored with the implementation and must pass before review."
  },
  {
    "lane": "drt-04-production-proof",
    "proof_role": "Execute the exact production-path qualification or deterministic conformance command for this Runtime slice.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 6500,
    "budget_tokens": 6000,
    "argv": [
      "bash",
      "adl/tools/v0921/drt-04/validate.sh"
    ],
    "parallel_group": "drt-04-production-proof",
    "defer_reason": "The issue owns this runner; live phases remain gated by their declared dependencies and external authority."
  },
  {
    "lane": "drt-04-diff-hygiene",
    "proof_role": "Reject whitespace and malformed-diff defects before exact-head review.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
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
    "parallel_group": "drt-04-diff-hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/184/validate-outcome.rb`
- `bash adl/tools/v0921/drt-04/validate.sh`
- `git diff --check origin/main...HEAD`

## Failure Semantics

Fail closed on dependency drift, path collision, authority ambiguity, missing producer evidence, validation failure, or unresolved review finding.

## Handoff

Retain typed evidence before convergence.
