# Validation Planning Prompt

Template: 1.0.0

Issue: 713

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/713/design.md

Diagram: .csdlc/prepared/issues/713/diagram.mmd

## Selected Lanes

[
  {
    "lane": "a2a-history-focused",
    "proof_role": "Prove verbatim causal A2A transcript persistence, replay handling, redaction, API projection, Observatory restore, restart, checkpoint, and rehydration in deterministic local proof.",
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
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_issue713_a2a_history.sh"
    ],
    "parallel_group": "a2a-history-local",
    "defer_reason": null
  },
  {
    "lane": "live-wuji-a2a",
    "proof_role": "Prove raw redacted bidirectional non-Shepherd Wuji A2A exchange and recovered verbatim transcript after reconnect/restart when explicitly operator-authorized.",
    "acceptance_ids": [
      "AC-4",
      "AC-6",
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 1200,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/713/validate-live-a2a-history.sh"
    ],
    "parallel_group": "a2a-live-serial",
    "defer_reason": "Live Wuji Runtime/ACIP proof requires explicit operator authorization; the wrapper fails closed unless ADL_LIVE_WUJI_A2A_HISTORY=1 is set."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/test_issue713_a2a_history.sh`
- `bash .csdlc/prepared/issues/713/validate-live-a2a-history.sh`

## Failure Semantics

Fail closed on identity, causal-chain, replay, authorization, or redaction mismatch; never manufacture missing transcript content.

## Handoff

Retain typed evidence before convergence.
