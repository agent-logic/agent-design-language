# Validation Planning Prompt

Template: 1.0.0

Issue: 418

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/418/design.md

Diagram: .csdlc/prepared/issues/418/diagram.mmd

## Selected Lanes

[
  {
    "lane": "gh-breakglass-policy-contract",
    "proof_role": "Prove exact policy prerequisites, canonical argv allowlist, comprehensive denial fixtures, three create-only receipt events, redaction, no-early-use, and typed reconciliation freeze.",
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
    "budget_seconds": 60,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/418/validate_gh_breakglass_policy.sh"
    ],
    "parallel_group": "418-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash .csdlc/prepared/issues/418/validate_gh_breakglass_policy.sh`

## Failure Semantics

Fail closed on any missing regression evidence, authorization, exact identity, allowlist match, immutable receipt, redaction, or typed reconciliation route; never broaden raw GitHub authority.

## Handoff

Retain typed evidence before convergence.
