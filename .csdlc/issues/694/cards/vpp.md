# Validation Planning Prompt

Template: 1.0.0

Issue: 694

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/694/design.md

Diagram: .csdlc/prepared/issues/694/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-694-complete-history-reload",
    "proof_role": "Prove complete ordered authorized history and fresh Observatory restoration through the issue-owned production-path validator.",
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
    "budget_seconds": 900,
    "budget_tokens": 3000,
    "argv": [
      "adl/tools/test_issue694_conversation_history_reload.sh"
    ],
    "parallel_group": "694-e2e",
    "defer_reason": null
  },
  {
    "lane": "runtime-conversation-history",
    "proof_role": "Prove complete ordered bounded authorized history records for operator and agent turns.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "durable_conversation_history_integration"
    ],
    "parallel_group": "694-focused",
    "defer_reason": null
  },
  {
    "lane": "observatory-history-restore",
    "proof_role": "Prove fresh UI restoration exact-once replay and privacy behavior through the governed transcript-history validator.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 2000,
    "argv": [
      "node",
      "adl/tools/validate_v092_observatory_transcript_history.mjs"
    ],
    "parallel_group": "694-focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `adl/tools/test_issue694_conversation_history_reload.sh`
- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test durable_conversation_history_integration`
- `node adl/tools/validate_v092_observatory_transcript_history.mjs`

## Failure Semantics

Fail closed on absent authorization malformed bounds unknown roles or ambiguous replay identity; never reconstruct operator content from reply text.

## Handoff

Retain typed evidence before convergence.
