# Validation Planning Prompt

Template: 1.0.0

Issue: 5499

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5499/design.md

Diagram: .csdlc/prepared/issues/5499/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove six-card, design, diagram, dependency, scope, COTS, budget, and no-product-change preparation truth",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5499/validate-preparation.rb"
    ],
    "parallel_group": "local-control",
    "defer_reason": null
  },
  {
    "lane": "dependency-gate",
    "proof_role": "Fail closed until #5340, #5341, #5342, and final WP-09 gate #5349 have merged typed closeout receipts ancestral to HEAD",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5499/check-dependencies.rb"
    ],
    "parallel_group": "local-control",
    "defer_reason": null
  },
  {
    "lane": "conductor-contract",
    "proof_role": "Run focused all-target conductor tests and strict Clippy from FastWork after the dependency gate opens",
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
    "resource_profile": "medium",
    "budget_seconds": 180,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5499/validate-conductor.sh"
    ],
    "parallel_group": "conductor-local",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Verify exact issue-branch patch hygiene",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local-control",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/5499/validate-preparation.rb`
- `ruby .csdlc/prepared/issues/5499/check-dependencies.rb`
- `bash .csdlc/prepared/issues/5499/validate-conductor.sh`
- `git diff --check`

## Failure Semantics

Fail closed on missing or stale lifecycle input, unresolved or cyclic dependencies, unknown lanes, WIP overflow, path overlap, ambiguous authority, nondeterministic output, absent retained receipts, or budget breach; never convert blocked preparation into product authority.

## Handoff

Retain typed evidence before convergence.
