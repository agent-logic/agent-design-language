# Validation Planning Prompt

Template: 1.0.0

Issue: 570

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/570/design.md

Diagram: .csdlc/prepared/issues/570/diagram.mmd

## Selected Lanes

[
  {
    "lane": "docs-stale-route-scan",
    "proof_role": "Prove updated docs do not present adl_pr_cycle, pr.sh, pr ready, pr preflight, raw GitHub fallback, or premature v3 authority as current workflow routes.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/570/validate-docs-routes.sh"
    ],
    "parallel_group": "v3-g-docs",
    "defer_reason": "Runs after documentation changes are made."
  },
  {
    "lane": "skill-guidance-scan",
    "proof_role": "Prove v2 operator and installed PR skill guidance records until-cutover v2 authority and v3 construction non-authority.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/570/validate-skill-guidance.sh"
    ],
    "parallel_group": "v3-g-docs",
    "defer_reason": "Runs after skill guidance checks and updates are made."
  },
  {
    "lane": "authority-boundary-scan",
    "proof_role": "Prove changed docs and skill guidance preserve the v2-live/v3-construction boundary and clean replacement truth before V3-F.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/570/validate-authority-boundary.sh"
    ],
    "parallel_group": "v3-g-docs",
    "defer_reason": "Runs after documentation and skill guidance changes are made."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace and malformed diff defects in the bounded #570 diff.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 300,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "v3-g-final",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/570/validate-docs-routes.sh`
- `bash .csdlc/prepared/issues/570/validate-skill-guidance.sh`
- `bash .csdlc/prepared/issues/570/validate-authority-boundary.sh`
- `git diff --check`

## Failure Semantics

Fail closed on any stale route presented as current workflow, premature v3 authority claim, untracked installed-skill divergence, hidden V3-F decision, or attempt to delete/retire v2 inside #570.

## Handoff

Retain typed evidence before convergence.
