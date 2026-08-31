# Validation Planning Prompt

Template: 1.0.0

Issue: 512

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/512/design.md

Diagram: .csdlc/prepared/issues/512/diagram.mmd

## Selected Lanes

[
  {
    "lane": "authentic-runtime-route",
    "proof_role": "Prove the HTML Observatory consumes the required authentic Runtime route, not a mock.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "bash",
      "adl/tools/validate_layer8_authority_observatory_ui.sh"
    ],
    "parallel_group": "runtime-route",
    "defer_reason": "Execution is blocked until #511 and #536 are terminal."
  },
  {
    "lane": "exact-browser-cases",
    "proof_role": "Run exact browser-facing redesign cases against the implemented OBS-A contract.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/512/validate-obs-b-browser.sh"
    ],
    "parallel_group": "browser",
    "defer_reason": null
  },
  {
    "lane": "accessibility",
    "proof_role": "Verify keyboard and screen-reader behavior for implemented views and states.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1800,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/512/validate-obs-b-accessibility.sh"
    ],
    "parallel_group": "browser",
    "defer_reason": null
  },
  {
    "lane": "redaction",
    "proof_role": "Verify projected Runtime data remains redacted in UI and evidence.",
    "acceptance_ids": [
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1800,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/512/validate-obs-b-redaction.sh"
    ],
    "parallel_group": "privacy",
    "defer_reason": null
  },
  {
    "lane": "recovery",
    "proof_role": "Verify empty degraded recovery and revoked UI states match OBS-A.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1800,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/512/validate-obs-b-recovery.sh"
    ],
    "parallel_group": "browser",
    "defer_reason": null
  },
  {
    "lane": "v3-local-canary",
    "proof_role": "Run the single csdlc binary local preparation path as non-authoritative cutover evidence.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "run",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--bin",
      "csdlc",
      "--",
      "local",
      "--request",
      ".csdlc/prepared/issues/512/v3-local-request.json",
      "--registry",
      "docs/templates/prompts/current.json",
      "--registrations",
      ".csdlc/prepared/issues/512/v3-local-registrations.json"
    ],
    "parallel_group": "cutover-canary",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/validate_layer8_authority_observatory_ui.sh`
- `bash .csdlc/prepared/issues/512/validate-obs-b-browser.sh`
- `bash .csdlc/prepared/issues/512/validate-obs-b-accessibility.sh`
- `bash .csdlc/prepared/issues/512/validate-obs-b-redaction.sh`
- `bash .csdlc/prepared/issues/512/validate-obs-b-recovery.sh`
- `cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- local --request .csdlc/prepared/issues/512/v3-local-request.json --registry docs/templates/prompts/current.json --registrations .csdlc/prepared/issues/512/v3-local-registrations.json`

## Failure Semantics

Fail closed on unmet #511 or #536 gates, mock Runtime substitution, redaction leakage, or v3 authority overclaim.

## Handoff

Retain typed evidence before convergence.
