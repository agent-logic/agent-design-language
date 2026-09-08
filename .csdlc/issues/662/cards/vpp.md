# Validation Planning Prompt

Template: 1.0.0

Issue: 662

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/662/design.md

Diagram: .csdlc/prepared/issues/662/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-agent-to-agent-initiation",
    "proof_role": "Prove successful governed Beacon-to-Ember initiation, distinct identity/correlation, replay behavior, cancellation, missing or stale recipient, unauthorized initiation, provider failure, and authoritative activity projection through deterministic Runtime integration tests.",
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
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      ".csdlc/prepared/issues/662/validate-focused.sh"
    ],
    "parallel_group": "runtime-agent-initiation",
    "defer_reason": "The exact focused test implementation is an issue #662 deliverable."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Prove exact branch diff whitespace hygiene.",
    "acceptance_ids": [
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

- `.csdlc/prepared/issues/662/validate-focused.sh`
- `git diff --check`

## Failure Semantics

Fail explicitly for unauthorized initiation, missing or stale recipient, cancellation, replay conflict, recipient failure, or provider failure; never synthesize delivery or model output success.

## Handoff

Retain typed evidence before convergence.
