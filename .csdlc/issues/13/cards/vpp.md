# Validation Planning Prompt

Template: 1.0.0

Issue: 13

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/13/design.md

Diagram: .csdlc/prepared/issues/13/diagram.mmd

## Selected Lanes

[
  {
    "lane": "ci-path-policy-contract",
    "proof_role": "Prove Runtime-only, full, fast-workspace, and skipped coverage selection.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 6000,
    "argv": [
      "bash",
      "adl/tools/test_ci_path_policy.sh"
    ],
    "parallel_group": "focused-contracts",
    "defer_reason": null
  },
  {
    "lane": "ci-runtime-contract",
    "proof_role": "Prove job-level guards and aggregate producer result semantics.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "parallel_group": "focused-contracts",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed tracked changes before review.",
    "acceptance_ids": [
      "AC-6"
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
    "parallel_group": "focused-contracts",
    "defer_reason": null
  },
  {
    "lane": "github-actions-runtime-canary",
    "proof_role": "Retain live timing evidence that Runtime coverage and required aggregates succeed while both workspace shards remain skipped without runner allocation.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 4000,
    "argv": [
      "gh",
      "run",
      "view",
      "--json",
      "jobs,status,conclusion"
    ],
    "parallel_group": "post-publication",
    "defer_reason": "Requires the published issue #13 PR head and its live GitHub Actions run."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/test_ci_path_policy.sh`
- `bash adl/tools/test_ci_runtime_contracts.sh`
- `git diff --check`
- `gh run view --json jobs,status,conclusion`

## Failure Semantics

Fail closed when selector values are malformed or producer results differ from the explicit selected route.

## Handoff

Retain typed evidence before convergence.
