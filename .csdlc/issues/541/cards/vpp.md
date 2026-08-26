# Validation Planning Prompt

Template: 1.0.0

Issue: 541

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/541/design.md

Diagram: .csdlc/prepared/issues/541/diagram.mmd

## Selected Lanes

[
  {
    "lane": "retired-route-search",
    "proof_role": "Check edited current guidance no longer presents retired lifecycle routes as current authority.",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "rg",
      "-n",
      "adl_pr_cycle|pr ready|pr run|pr finish|pr\\.sh",
      "docs/onboarding.md",
      "docs/tooling",
      "adl/tools/README.md"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "current-authority-search",
    "proof_role": "Check edited current guidance names typed v2 authority, installed binaries, typed skills, and canonical repository identity.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "rg",
      "-n",
      "Gate 10D2|csdlc-v2/operator/skills|\\.adl/bin/csdlc-v2|agent-logic/agent-design-language|legacy-origin",
      "docs/onboarding.md",
      "docs/tooling",
      "adl/tools/README.md"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Check edited docs have no whitespace or conflict-marker issues.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 250,
    "argv": [
      "git",
      "diff",
      "--check"
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

- `rg -n adl_pr_cycle|pr ready|pr run|pr finish|pr\.sh docs/onboarding.md docs/tooling adl/tools/README.md`
- `rg -n Gate 10D2|csdlc-v2/operator/skills|\.adl/bin/csdlc-v2|agent-logic/agent-design-language|legacy-origin docs/onboarding.md docs/tooling adl/tools/README.md`
- `git diff --check`

## Failure Semantics

Fail closed on stale current-route guidance, repository identity confusion, runtime/tooling mutations outside docs scope, direct card edits, or terminal-state overclaiming.

## Handoff

Retain typed evidence before convergence.
