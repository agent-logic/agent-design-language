# Validation Planning Prompt

Template: 1.0.0

Issue: 506

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/506/design.md

Diagram: .csdlc/prepared/issues/506/diagram.mmd

## Selected Lanes

[
  {
    "lane": "qualification-contract",
    "proof_role": "Prove requirements 181 and 182 are mapped into one deterministic distributed qualification contract.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl-runtime/tests/distributed_contract/validate_drt_a.sh",
      "qualification-contract"
    ],
    "parallel_group": "drt-a",
    "defer_reason": "Deferred only until #506 implementation creates the issue-owned test harness under adl-runtime/tests/distributed_contract/**."
  },
  {
    "lane": "acip-authority",
    "proof_role": "Prove ACIP identity, authority, permit, causation, correlation, sequence, term, Polis, payload, and credential-binding behavior.",
    "acceptance_ids": [
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl-runtime/tests/distributed_contract/validate_drt_a.sh",
      "acip-authority"
    ],
    "parallel_group": "drt-a",
    "defer_reason": "Deferred only until #506 implementation creates the issue-owned test harness under adl-runtime/tests/distributed_contract/**."
  },
  {
    "lane": "replay-conformance",
    "proof_role": "Prove byte-stable encode, decode, re-encode, duplicate-denial, and replay receipts from committed fixtures.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl-runtime/tests/distributed_contract/validate_drt_a.sh",
      "replay-conformance"
    ],
    "parallel_group": "drt-a",
    "defer_reason": "Deferred only until #506 implementation creates the issue-owned test harness under adl-runtime/tests/distributed_contract/**."
  },
  {
    "lane": "negative-matrix",
    "proof_role": "Prove stale, duplicate, reordered, malformed, unsigned, wrong-domain, cross-Polis, and authority-mutation attempts fail closed.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl-runtime/tests/distributed_contract/validate_drt_a.sh",
      "negative-matrix"
    ],
    "parallel_group": "drt-a",
    "defer_reason": "Deferred only until #506 implementation creates the issue-owned test harness under adl-runtime/tests/distributed_contract/**."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl-runtime/tests/distributed_contract/validate_drt_a.sh qualification-contract`
- `bash adl-runtime/tests/distributed_contract/validate_drt_a.sh acip-authority`
- `bash adl-runtime/tests/distributed_contract/validate_drt_a.sh replay-conformance`
- `bash adl-runtime/tests/distributed_contract/validate_drt_a.sh negative-matrix`

## Failure Semantics

Fail closed if identity provenance is synthetic, replay can change authority, duplicate-denial receipts are not exact, or negative authority inputs pass.

## Handoff

Retain typed evidence before convergence.
