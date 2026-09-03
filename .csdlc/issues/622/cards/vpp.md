# Validation Planning Prompt

Template: 1.0.0

Issue: 622

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/622/design.md

Diagram: .csdlc/prepared/issues/622/diagram.mmd

## Selected Lanes

[
  {
    "lane": "provider-reload-production",
    "proof_role": "Prove valid activation last-known-good retention and the production execution call path with a nonzero exact target.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/622/validate-provider-profile-hotload.sh",
      "production"
    ],
    "parallel_group": "provider-runtime",
    "defer_reason": null
  },
  {
    "lane": "provider-reload-safety",
    "proof_role": "Prove atomic concurrency parameter capability redaction debounce and shutdown behavior with nonzero focused tests.",
    "acceptance_ids": [
      "AC-4",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/622/validate-provider-profile-hotload.sh",
      "safety"
    ],
    "parallel_group": "provider-safety",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/622/validate-provider-profile-hotload.sh production`
- `bash .csdlc/prepared/issues/622/validate-provider-profile-hotload.sh safety`

## Failure Semantics

Fail closed on missing production ownership, partial snapshots, invalid-candidate promotion, credential disclosure, authority mutation, restart-based substitution, zero-test selectors, or absent exact-head review.

## Handoff

Retain typed evidence before convergence.
