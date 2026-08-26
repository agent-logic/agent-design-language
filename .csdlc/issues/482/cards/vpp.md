# Validation Planning Prompt

Template: 1.0.0

Issue: 482

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/482/design.md

Diagram: .csdlc/prepared/issues/482/diagram.mmd

## Selected Lanes

[
  {
    "lane": "asset-denominator",
    "proof_role": "Prove asset denominator for CORP-A.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/482/validate-asset-denominator.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "provenance-and-license",
    "proof_role": "Prove provenance and license for CORP-A.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/482/validate-provenance-and-license.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "redaction-and-custody",
    "proof_role": "Prove redaction and custody for CORP-A.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1800,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/482/validate-redaction-and-custody.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Prove full branch diff hygiene for CORP-A.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1800,
    "argv": [
      "git",
      "diff",
      "main...HEAD",
      "--check"
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

- `ruby .csdlc/prepared/issues/482/validate-asset-denominator.rb`
- `ruby .csdlc/prepared/issues/482/validate-provenance-and-license.rb`
- `ruby .csdlc/prepared/issues/482/validate-redaction-and-custody.rb`
- `git diff main...HEAD --check`

## Failure Semantics

Fail closed on any stop condition, authority ambiguity, secret exposure, incomplete denominator, or non-proving validation.

## Handoff

Retain typed evidence before convergence.
