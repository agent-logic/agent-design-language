# Validation Planning Prompt

Template: 1.0.0

Issue: 514

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/514/design.md

Diagram: .csdlc/prepared/issues/514/diagram.mmd

## Selected Lanes

[
  {
    "lane": "profile-schema",
    "proof_role": "Prove profile schema for PROV-A.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/514/validate-profile-schema.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "ollama-materialization",
    "proof_role": "Prove ollama materialization for PROV-A.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/514/validate-ollama-materialization.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "invalid-profile",
    "proof_role": "Prove invalid profile for PROV-A.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/514/validate-invalid-profile.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "last-known-good",
    "proof_role": "Prove last known good for PROV-A.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/514/validate-last-known-good.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "redaction",
    "proof_role": "Prove redaction for PROV-A.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/514/validate-redaction.rb"
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

- `ruby .csdlc/prepared/issues/514/validate-profile-schema.rb`
- `ruby .csdlc/prepared/issues/514/validate-ollama-materialization.rb`
- `ruby .csdlc/prepared/issues/514/validate-invalid-profile.rb`
- `ruby .csdlc/prepared/issues/514/validate-last-known-good.rb`
- `ruby .csdlc/prepared/issues/514/validate-redaction.rb`

## Failure Semantics

Fail closed on any stop condition, authority ambiguity, secret exposure, incomplete denominator, or non-proving validation.

## Handoff

Retain typed evidence before convergence.
