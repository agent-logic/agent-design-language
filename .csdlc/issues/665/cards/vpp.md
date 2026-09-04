# Validation Planning Prompt

Template: 1.0.0

Issue: 665

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/665/design.md

Diagram: .csdlc/prepared/issues/665/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-v2-emergency-adoption-regression",
    "proof_role": "Prove the #660-shaped ready-phase emergency branch/worktree adoption path, fail-closed topology rejects, preservation, evidence recording, and downstream lifecycle eligibility.",
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
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "emergency_branch_adoption"
    ],
    "parallel_group": "csdlc-v2-bind",
    "defer_reason": null
  },
  {
    "lane": "issue-665-scope-denominator",
    "proof_role": "Prove the #665 readiness packet names issue-owned bind/topology design and repository targets before implementation begins.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/665/validate-emergency-adoption-scope.sh"
    ],
    "parallel_group": "readiness",
    "defer_reason": null
  },
  {
    "lane": "issue-665-operator-docs",
    "proof_role": "Prove the operator documentation exists and names the fail-closed ready-to-bound adoption sequence.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/665/validate-emergency-adoption-docs.sh"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Prove whitespace and conflict-marker hygiene for the exact branch diff.",
    "acceptance_ids": [
      "AC-6",
      "AC-8"
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
    "parallel_group": "hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5 emergency_branch_adoption`
- `bash .csdlc/prepared/issues/665/validate-emergency-adoption-scope.sh`
- `bash .csdlc/prepared/issues/665/validate-emergency-adoption-docs.sh`
- `git diff --check`

## Failure Semantics

Fail closed on stale generation/digest, mismatched repository, unsafe branch or worktree, dirty or ambiguous topology, missing base ancestry, collision state, weakened review/publication gates, or zero-test validation.

## Handoff

Retain typed evidence before convergence.
