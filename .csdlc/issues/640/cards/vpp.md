# Validation Planning Prompt

Template: 1.0.0

Issue: 640

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Run one issue-owned deterministic nonzero Runtime validator followed by a coordinated exact-candidate Wuji acceptance mode.

## Lane Inputs

Design: .csdlc/prepared/issues/640/design.md

Diagram: .csdlc/prepared/issues/640/diagram.mmd

## Selected Lanes

[
  {
    "lane": "model-backed-shepherd",
    "proof_role": "Prove provider-neutral non-empty unique configuration, governed provider execution, preload state, degraded isolation, restart recovery, canonical identity, /v1/ready and Observatory agreement, API health, formatting, and diff hygiene with nonzero focused tests.",
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
    "resource_profile": "large",
    "budget_seconds": 3900,
    "budget_tokens": 20000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/640/validate-model-backed-shepherd.sh"
    ],
    "parallel_group": "runtime",
    "defer_reason": "The issue-owned wrapper exists; its named resident_shepherd, shepherd_provider, shepherd_model_health, and shepherd_readiness_consistency cases are issue #640 implementation deliverables."
  },
  {
    "lane": "wuji-shepherd-acceptance",
    "proof_role": "Prove the exact candidate restarts on Wuji, automatically preloads its configured local model, reports consistent readiness, and completes one governed Shepherd inference without manual Ollama loading.",
    "acceptance_ids": [
      "AC-4",
      "AC-6",
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 2400,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/640/validate-model-backed-shepherd.sh",
      "--live-wuji"
    ],
    "parallel_group": "live-wuji",
    "defer_reason": "The issue-owned live mode fails closed until implementation supplies the coordinated exact-candidate Wuji acceptance."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash .csdlc/prepared/issues/640/validate-model-backed-shepherd.sh`
- `bash .csdlc/prepared/issues/640/validate-model-backed-shepherd.sh --live-wuji`

## Failure Semantics

Fail closed on missing #617 ancestry, invalid or secret-bearing configuration, an empty Shepherd set, duplicate configured identity, ungoverned inference, false or inconsistent readiness, whole-Runtime failure coupling, zero-test validation, stale review, or uncoordinated Wuji mutation.

## Handoff

Retain typed evidence before convergence.
