# Validation Planning Prompt

Template: 1.0.0

Issue: 499

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/499/design.md

Diagram: .csdlc/prepared/issues/499/diagram.mmd

## Selected Lanes

[
  {
    "lane": "api-parity",
    "proof_role": "Prove api parity for RUST-01.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/499/validate-api-parity.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "resilience-positive-negative",
    "proof_role": "Prove resilience positive negative for RUST-01.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/499/validate-resilience-positive-negative.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "fault-and-trace",
    "proof_role": "Prove fault and trace for RUST-01.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/499/validate-fault-and-trace.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "retry-timeout-cancellation",
    "proof_role": "Prove retry timeout cancellation for RUST-01.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/499/validate-retry-timeout-cancellation.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "validation-impact",
    "proof_role": "Prove validation impact for RUST-01.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/499/validate-validation-impact.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "fmt-clippy",
    "proof_role": "Prove fmt clippy for RUST-01.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/499/validate-fmt-clippy.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/499/validate-api-parity.rb`
- `ruby .csdlc/prepared/issues/499/validate-resilience-positive-negative.rb`
- `ruby .csdlc/prepared/issues/499/validate-fault-and-trace.rb`
- `ruby .csdlc/prepared/issues/499/validate-retry-timeout-cancellation.rb`
- `ruby .csdlc/prepared/issues/499/validate-validation-impact.rb`
- `ruby .csdlc/prepared/issues/499/validate-fmt-clippy.rb`

## Failure Semantics

Fail closed on any stop condition, authority ambiguity, secret exposure, incomplete denominator, or non-proving validation.

## Handoff

Retain typed evidence before convergence.
