# Validation Planning Prompt

Template: 1.0.0

Issue: 5384

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5384/design.md

Diagram: .csdlc/prepared/issues/5384/diagram.mmd

## Selected Lanes

[
  {
    "lane": "typed-card-contracts",
    "proof_role": "Required preparation gate for six-card current-native identity, structure, schema, digest, and canonical lifecycle consistency",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5384"
    ],
    "parallel_group": "local-preparation",
    "defer_reason": null
  },
  {
    "lane": "dependency-terminal-gate",
    "proof_role": "Required promotion gate for complete predecessor typed closed_out receipts, merged disposition, and observed-SHA ancestry; live issue and PR state is separately refreshed through the approved connector",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5384/validate_dependency_gate.rb"
    ],
    "parallel_group": "local-preparation",
    "defer_reason": null
  },
  {
    "lane": "preparation-scope",
    "proof_role": "Required preparation gate confirming that tracked and untracked changes remain inside the three authorized #5384 lifecycle paths",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5384/validate_preparation_scope.rb"
    ],
    "parallel_group": "local-preparation",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Required preparation gate for whitespace and patch hygiene",
    "acceptance_ids": [
      "AC-1",
      "AC-7"
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
    "parallel_group": "local-preparation",
    "defer_reason": null
  },
  {
    "lane": "exact-preparation-review",
    "proof_role": "Required preparation gate for bounded independent review of cards, design, diagram, gates, and protected-path authority before typed approval and bind",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 6000,
    "argv": [
      "codex",
      "exec",
      "--sandbox",
      "read-only"
    ],
    "parallel_group": "review",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5384`
- `ruby .csdlc/prepared/issues/5384/validate_dependency_gate.rb`
- `ruby .csdlc/prepared/issues/5384/validate_preparation_scope.rb`
- `git diff --check`
- `codex exec --sandbox read-only`

## Failure Semantics

Fail closed on missing current-template proof, incomplete dependency topology, absent merged/closed_out/receipt/ancestry evidence, protected-path widening, stale live truth, unsupported claims, unresolved review findings, or any request to implement, publish, merge, use AWS, use Runtime v2, or invoke raw gh.

## Handoff

Retain typed evidence before convergence.
