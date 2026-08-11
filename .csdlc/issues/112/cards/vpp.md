# Validation Planning Prompt

Template: 1.0.0

Issue: 112

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/112/design.md

Diagram: .csdlc/prepared/issues/112/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-112-preparation-hygiene",
    "proof_role": "Prove all six typed cards, approved design, sole #111 serial gate, fail-closed validation truth, and pre-execution SRP/SOR truth without claiming product behavior.",
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
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/112/validate-preparation.rb"
    ],
    "parallel_group": "112-preparation",
    "defer_reason": null
  },
  {
    "lane": "layer8-authority-contract-plan",
    "proof_role": "Validate the future product PVF contract cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --test layer8_authority --no-tests=fail after #111 merges; this preparation lane proves plan completeness only, not product behavior.",
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
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/112/validate-preparation.rb",
      "--lane",
      "authority-contract"
    ],
    "parallel_group": "112-preparation",
    "defer_reason": null
  },
  {
    "lane": "layer8-runtime-api-integration-plan",
    "proof_role": "Validate the future product PVF contract cargo nextest run --locked --manifest-path adl/Cargo.toml --test layer8_authority_runtime_api --no-tests=fail after #111 merges; this preparation lane proves plan completeness only, not product behavior.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/112/validate-preparation.rb",
      "--lane",
      "runtime-api-integration"
    ],
    "parallel_group": "112-preparation",
    "defer_reason": null
  },
  {
    "lane": "layer8-observatory-ui-plan",
    "proof_role": "Validate the future real-browser product PVF contract adl/tools/validate_layer8_authority_observatory_ui.sh for authorized and refused states after #111 merges; this preparation lane proves plan completeness only, not product behavior.",
    "acceptance_ids": [
      "AC-3",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/112/validate-preparation.rb",
      "--lane",
      "observatory-ui"
    ],
    "parallel_group": "112-preparation",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/112/validate-preparation.rb`
- `ruby .csdlc/prepared/issues/112/validate-preparation.rb --lane authority-contract`
- `ruby .csdlc/prepared/issues/112/validate-preparation.rb --lane runtime-api-integration`
- `ruby .csdlc/prepared/issues/112/validate-preparation.rb --lane observatory-ui`

## Failure Semantics

Fail closed on serial-gate or ownership drift, unauthenticated or stale identity, capability mismatch, recipient substitution or widening, cross-Polis action, replay, revocation, expiry, policy uncertainty, audit discontinuity or write failure, forbidden-field leakage, zero-test selection, preparation-as-product-proof substitution, or unresolved exact-head findings.

## Handoff

Retain typed evidence before convergence.
