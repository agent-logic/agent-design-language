# Validation Planning Prompt

Template: 1.0.0

Issue: 280

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/280/design.md

Diagram: .csdlc/prepared/issues/280/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-280-preparation-validator",
    "proof_role": "Validate #280 issue identity, dependency ancestry, performance/recovery proof scope, and forbidden sibling/parent ownership.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/280/validate_preparation_bundle.py"
    ],
    "parallel_group": "280-local-proof",
    "defer_reason": null
  },
  {
    "lane": "observatory-large-polis-performance-recovery",
    "proof_role": "Run deterministic large-roster, long-transcript, stream pressure, reconnect/restart/offline/version-mismatch recovery assertions and emit public-safe metrics.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 180,
    "budget_tokens": 3000,
    "argv": [
      "node",
      "demos/html-observatory/tests/large_polis_performance_recovery.test.mjs"
    ],
    "parallel_group": "280-local-proof",
    "defer_reason": null
  },
  {
    "lane": "observatory-conversation-regression",
    "proof_role": "Prove existing Runtime conversation acceptance/delivery/reconnect behavior still passes after #280 projection-bounding changes.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "node",
      "demos/html-observatory/tests/conversation_sessions.test.mjs"
    ],
    "parallel_group": "280-local-regression",
    "defer_reason": null
  },
  {
    "lane": "observatory-operator-attention-regression",
    "proof_role": "Prove operator-attention Runtime-authority boundaries still pass after #280 projection-bounding changes.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "node",
      "demos/html-observatory/tests/operator_attention_inbox.test.mjs"
    ],
    "parallel_group": "280-local-regression",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed whitespace or patch artifacts before review.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "280-local-hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 .csdlc/prepared/issues/280/validate_preparation_bundle.py`
- `node demos/html-observatory/tests/large_polis_performance_recovery.test.mjs`
- `node demos/html-observatory/tests/conversation_sessions.test.mjs`
- `node demos/html-observatory/tests/operator_attention_inbox.test.mjs`
- `git diff --check`

## Failure Semantics

Fail closed on missing dependency terminal truth, performance/recovery proof failure, overbroad Runtime or sibling scope, stale exact-head review, PR linkage failure, CI failure, or terminal-cache mismatch.

## Handoff

Retain typed evidence before convergence.
