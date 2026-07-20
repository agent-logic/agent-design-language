# Validation Planning Prompt

Template: 1.0.0

Issue: 5358

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5358/design.md

Diagram: .csdlc/prepared/issues/5358/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-doctor-5358",
    "proof_role": "Check canonical typed state, all six generated cards, digests, retained design, and bound issue integrity",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5358"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "issue-local-scope-inventory",
    "proof_role": "Enumerate every untracked #5358 issue-local artifact for bounded review",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "ls-files",
      "--others",
      "--exclude-standard",
      "--",
      ".csdlc/issues/5358",
      ".csdlc/prepared/issues/5358",
      ".csdlc/evidence/5358"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `csdlc-doctor --repo . --issue 5358`
- `git ls-files --others --exclude-standard -- .csdlc/issues/5358 .csdlc/prepared/issues/5358 .csdlc/evidence/5358`

## Failure Semantics

Fail closed on card corruption, claim collision, stale authority, unresolved acceptance blockers, missing exact-revision evidence, or any preparation-to-acceptance overclaim.

## Handoff

Retain typed evidence before convergence.
