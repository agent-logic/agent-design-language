# Validation Planning Prompt

Template: 1.0.0

Issue: 278

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Pre-bind validation proves dependency/scope readiness only; runtime and Observatory product proof remains deferred until the bound #278 implementation creates exact issue-owned test and validator targets.

## Lane Inputs

Design: .csdlc/prepared/issues/278/design.md

Diagram: .csdlc/prepared/issues/278/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-278-preparation-validator",
    "proof_role": "Prove #278 live issue identity, dependency terminal caches, ancestry, scope boundaries, non-goals, and dedicated branch/worktree strings.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/278/validate_preparation_bundle.py"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-kernel-conversation-history-focused",
    "proof_role": "Prove authorized pagination, stale cursor denial, revoked access denial, private-memory denial, restart restoration, search, export, and redaction behavior.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "conversation_history"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "observatory-transcript-restore-validator",
    "proof_role": "Prove Observatory transcript restoration consumes Runtime-owned durable history and rejects stale browser state/redacts unsafe fields.",
    "acceptance_ids": [
      "AC-4",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "node",
      "adl/tools/validate_v092_observatory_transcript_history.mjs"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-kernel-strict-clippy",
    "proof_role": "Prove strict Rust hygiene for the touched Runtime kernel library before review/publication.",
    "acceptance_ids": [
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 .csdlc/prepared/issues/278/validate_preparation_bundle.py`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test conversation_history`
- `node adl/tools/validate_v092_observatory_transcript_history.mjs`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --lib -- -D warnings`

## Failure Semantics

Fail closed on terminal dependency mismatch, non-ancestral merge SHA, missing re-authorization, stale cursor acceptance, revoked access acceptance, private-memory exposure, redaction drift, stale browser restore, invalid bind target, failed proof, or unresolved review finding.

## Handoff

Retain typed evidence before convergence.
