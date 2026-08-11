# Validation Planning Prompt

Template: 1.0.0

Issue: 114

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/114/design.md

Diagram: .csdlc/prepared/issues/114/diagram.mmd

## Selected Lanes

[
  {
    "lane": "conversation-history-store",
    "proof_role": "Prove ordered atomic persistence, restart, idempotency, outcomes, cursors, retention, deletion, migration, corruption quarantine, redaction, and the exact forty-two-case denominator with nonzero selection.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 24000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "conversation_history",
      "--no-tests=fail"
    ],
    "parallel_group": "114-runtime",
    "defer_reason": "Deferred until #111 and #112 are terminal and ancestral and the exact owned test target exists; fail closed on zero tests or denominator drift."
  },
  {
    "lane": "conversation-history-runtime-api",
    "proof_role": "Prove fresh authorization before every read or lifecycle action, bounded projections, stable cursor behavior, denial, export, deletion, and no private-field exposure.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 16000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--test",
      "conversation_history_runtime_api",
      "--no-tests=fail"
    ],
    "parallel_group": "114-api",
    "defer_reason": "Deferred until the merged dependency APIs and exact owned integration target exist; fail closed on unauthorized projection or zero tests."
  },
  {
    "lane": "conversation-history-clippy",
    "proof_role": "Reject warnings and API misuse in the exact history store test target.",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "conversation_history",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "114-runtime",
    "defer_reason": "Deferred until the exact owned source and target exist; fail closed on any warning."
  },
  {
    "lane": "conversation-history-browser",
    "proof_role": "Prove real Runtime-backed paging, search, receipts, reconnect, revocation cache clearing, deletion, migration unavailable, and corruption quarantine states in the Observatory.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 18000,
    "argv": [
      "node",
      "adl/tools/validate_v092_html_observatory_history.mjs"
    ],
    "parallel_group": "114-browser",
    "defer_reason": "Deferred until the exact browser validator and merged Runtime history API exist; fixture-only or simulated responses do not pass."
  },
  {
    "lane": "conversation-history-diff-hygiene",
    "proof_role": "Reject malformed patches and out-of-scope whitespace damage before review.",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "114-hygiene",
    "defer_reason": "Run against the eventual implementation candidate after dependency gates clear; preparation readiness is validated separately by typed C-SDLC validation."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 21600

Tokens: 100000

## Commands

- `cargo nextest run --locked --manifest-path adl-runtime/Cargo.toml --test conversation_history --no-tests=fail`
- `cargo nextest run --locked --manifest-path adl/Cargo.toml --test conversation_history_runtime_api --no-tests=fail`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test conversation_history -- -D warnings`
- `node adl/tools/validate_v092_html_observatory_history.mjs`
- `git diff --check`

## Failure Semantics

Fail closed on nonterminal or nonancestral dependencies, stale authority, unauthorized read, cursor or sequence drift, duplicate conflict, terminal rewrite, forbidden-field exposure, unbounded search/export, retention or deletion ambiguity, partial write, disk-full, reply loss, unknown or lossy migration, unsafe rollback, corruption, receipt-chain break, residue, zero-test selection, denominator drift, or unresolved review finding.

## Handoff

Retain typed evidence before convergence.
