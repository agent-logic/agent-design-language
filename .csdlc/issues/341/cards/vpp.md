# Validation Planning Prompt

Template: 1.0.0

Issue: 341

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/341/design.md

Diagram: .csdlc/prepared/issues/341/diagram.mmd

## Selected Lanes

[
  {
    "lane": "provider-positive-matrix",
    "proof_role": "Run at least two approved real-provider positive columns through the identical scenario and retain redacted receipts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/demo_v092_provider_neutral_birthday.sh",
      "--mode",
      "positive"
    ],
    "parallel_group": "provider-proof",
    "defer_reason": null
  },
  {
    "lane": "proof-validator-tests",
    "proof_role": "Run focused validator and negative-case tests without provider credentials; exact-head review remains SRP/review lifecycle truth.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/test_v092_provider_neutral_proof.sh"
    ],
    "parallel_group": "local-proof",
    "defer_reason": null
  },
  {
    "lane": "proof-matrix-validator",
    "proof_role": "Validate redaction, parity, outcome classifications, digests, and no fixture/cached substitution.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      "adl/tools/validate_v092_provider_neutral_proof.py",
      "demos/v0.92/provider-neutral-birthday/proof-matrix-positive.json",
      "--require-live"
    ],
    "parallel_group": "local-proof",
    "defer_reason": null
  },
  {
    "lane": "private-observatory-demo",
    "proof_role": "Show several private agents running in the Observatory without public exposure.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/demo_v092_provider_neutral_birthday.sh",
      "--mode",
      "observatory"
    ],
    "parallel_group": "demo",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash adl/tools/demo_v092_provider_neutral_birthday.sh --mode positive`
- `bash adl/tools/test_v092_provider_neutral_proof.sh`
- `python3 adl/tools/validate_v092_provider_neutral_proof.py demos/v0.92/provider-neutral-birthday/proof-matrix-positive.json --require-live`
- `bash adl/tools/demo_v092_provider_neutral_birthday.sh --mode observatory`

## Failure Semantics

Fail closed on missing provider credentials, stale prerequisite truth, unredacted payloads, cached/fixture substitutions, non-equivalent scenarios, hidden provider failures, unavailable Runtime, or missing exact-head review.

## Handoff

Retain typed evidence before convergence.
