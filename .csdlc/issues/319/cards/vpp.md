# Validation Planning Prompt

Template: 1.0.0

Issue: 319

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/319/design.md

Diagram: .csdlc/prepared/issues/319/diagram.mmd

## Selected Lanes

[
  {
    "lane": "ceremony-unit",
    "proof_role": "Prove safe split-step ceremony behavior and negative paths.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_release_ceremony.sh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "release-evidence",
    "proof_role": "Validate exact predecessor, candidate, document, receipt, and non-claim bindings.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/319/validate-release-evidence.rb",
      "all"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "ceremony-preflight",
    "proof_role": "Run and validate the real ceremony script in non-mutating check-only mode.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/319/validate-release-evidence.rb",
      "ceremony"
    ],
    "parallel_group": "preflight",
    "defer_reason": null
  },
  {
    "lane": "typed-exact-head-review",
    "proof_role": "Record and verify the independent exact-head review identity and result before publication.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "csdlc-review",
      "--root",
      ".",
      "--request",
      ".csdlc/prepared/issues/319/record-review.json"
    ],
    "parallel_group": "review",
    "defer_reason": "Executed after implementation commits establish the exact review revision."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/test_release_ceremony.sh`
- `ruby .csdlc/prepared/issues/319/validate-release-evidence.rb all`
- `ruby .csdlc/prepared/issues/319/validate-release-evidence.rb ceremony`
- `csdlc-review --root . --request .csdlc/prepared/issues/319/record-review.json`

## Failure Semantics

Fail closed before tag or release mutation and preserve exact failure evidence for bounded repair.

## Handoff

Retain typed evidence before convergence.
