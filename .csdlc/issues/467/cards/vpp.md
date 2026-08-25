# Validation Planning Prompt

Template: 1.0.0

Issue: 467

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/467/design.md

Diagram: .csdlc/prepared/issues/467/diagram.mmd

## Selected Lanes

[
  {
    "lane": "467-preparation-bundle",
    "proof_role": "Validate #467 issue identity, unbound topology, design/diagram presence, and concrete owned validator targets before bind.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/467/validate-preparation.rb"
    ],
    "parallel_group": "467-prep",
    "defer_reason": null
  },
  {
    "lane": "467-quality-gate-matrix",
    "proof_role": "Regenerate and validate the exact 33-row corrective v0.92 quality-gate matrix with canonical hydration and concrete blocker taxonomy.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/467/validate-quality-gate.rb",
      "matrix"
    ],
    "parallel_group": "467-local",
    "defer_reason": "Runs after implementation."
  },
  {
    "lane": "467-adversarial-suite",
    "proof_role": "Run positive controls and adversarial cases for fabricated accepted rows, suppressed evidence, stale/non-ancestral authority, malformed terminal evidence, substitutions, duplicates, and vacuous publication.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/467/test-validate-quality-gate.rb"
    ],
    "parallel_group": "467-local",
    "defer_reason": "Runs after implementation."
  },
  {
    "lane": "467-diff-hygiene",
    "proof_role": "Reject whitespace and conflict artifacts.",
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
    "parallel_group": "467-local",
    "defer_reason": "Runs after implementation."
  },
  {
    "lane": "467-exact-head-review",
    "proof_role": "Record one fresh exact-head review covering every row disposition and discovery-completeness denominator before publication.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-review",
      "guard",
      "--request",
      "/Users/daniel/git/agent-design-language/.git/csdlc-v2/requests/467-review-guard.json"
    ],
    "parallel_group": "467-review",
    "defer_reason": "Runs after implementation and local validation."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/467/validate-preparation.rb`
- `ruby .csdlc/prepared/issues/467/validate-quality-gate.rb matrix`
- `ruby .csdlc/prepared/issues/467/test-validate-quality-gate.rb`
- `git diff --check`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-review guard --request /Users/daniel/git/agent-design-language/.git/csdlc-v2/requests/467-review-guard.json`

## Failure Semantics

Fail closed on stale/non-ancestral evidence, unclassified rows, duplicate or extra mappings, self-attested substitutions, packet-missing-only product blockers, uninvestigated all-blocked publication, review finding, or lifecycle topology drift.

## Handoff

Retain typed evidence before convergence.
