# Validation Planning Prompt

Template: 1.0.0

Issue: 426

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/426/design.md

Diagram: .csdlc/prepared/issues/426/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csmctl-linux-lifecycle",
    "proof_role": "Prove Darwin routing, native Linux lifecycle and process ownership, stop-timeout refusal, continuity preservation after refusal, unsupported-platform refusal, and documentation coverage.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_csmctl_linux_backend.sh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "gemini-exact-head-receipt",
    "proof_role": "Deterministically validate the redacted Gemini 3.1 Pro exact-head approval receipt without treating provider output as lifecycle authority.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/426/validate_gemini_review.py"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_csmctl_linux_backend.sh`
- `python3 .csdlc/prepared/issues/426/validate_gemini_review.py`

## Failure Semantics

Fail closed on unsupported OS, ambiguous PID ownership, readiness failure, or review findings.

## Handoff

Retain typed evidence before convergence.
