# Validation Planning Prompt

Template: 1.0.0

Issue: 679

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/679/design.md

Diagram: .csdlc/prepared/issues/679/diagram.mmd

## Selected Lanes

[
  {
    "lane": "679-init-package",
    "proof_role": "Prove #679 initialized package carries issue identity, S3 deployable Observatory boundaries, agent-logic-admin profile requirement, no-live-mutation default, and an executable preparation lane.",
    "acceptance_ids": [
      "AC-2",
      "AC-6",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/679/validate_init_package.py"
    ],
    "parallel_group": "679-prep",
    "defer_reason": null
  },
  {
    "lane": "679-deployability",
    "proof_role": "Prove static bundle relativity, CSP/header policy, redaction, profile-gated readbacks, and no-live-mutation defaults after implementation.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/679/validate_s3_deployable_observatory.py"
    ],
    "parallel_group": "679-impl",
    "defer_reason": "Added after implementation creates the deployability proof surface."
  },
  {
    "lane": "679-diff",
    "proof_role": "Reject whitespace and conflict-marker drift across the exact issue diff.",
    "acceptance_ids": [
      "AC-9"
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
    "parallel_group": "679-impl",
    "defer_reason": "Runs after implementation."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 .csdlc/prepared/issues/679/validate_init_package.py`
- `python3 .csdlc/prepared/issues/679/validate_s3_deployable_observatory.py`
- `git diff --check`

## Failure Semantics

Fail closed on live AWS mutation without explicit authority, missing business-profile guard, embedded secrets, absolute dev-only bundle paths, weakened TLS/CORS/CSP/WSS controls, or stale/missing exact-head review.

## Handoff

Retain typed evidence before convergence.
