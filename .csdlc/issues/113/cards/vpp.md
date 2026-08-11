# Validation Planning Prompt

Template: 1.0.0

Issue: 113

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/113/design.md

Diagram: .csdlc/prepared/issues/113/diagram.mmd

## Selected Lanes

[
  {
    "lane": "agent-roster-contract",
    "proof_role": "Run the exact nonzero issue-owned agent_roster integration target for deterministic paginated roster and detail projection, stable identity, every declared presence state, freshness deadlines, provenance, policy filtering and redaction, relocation fencing, capability and communication eligibility, token and cursor integrity, reconnect and restart behavior, and large-Polis resource bounds.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "agent_roster",
      "--no-tests=fail"
    ],
    "parallel_group": "runtime",
    "defer_reason": "Deferred until #113 implementation creates adl-runtime-kernel/tests/agent_roster.rs after the #83 and #142 serial gates pass; validation must fail closed if the target is absent, selects zero tests, or does not prove the exact candidate revision."
  },
  {
    "lane": "observatory-roster-browser",
    "proof_role": "Run the exact issue-owned live Runtime-backed roster browser validator for truthful count and pagination, presence and freshness display, server-authoritative visibility and redaction, bounded capability and communication eligibility, revision-bound updates, search, filter, sort, selection, detail, status changes, keyboard operation, denial, disconnect, bounded reconnect without duplicates, bounded DOM rows, responsive layout, and a clean console.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "node",
      "adl/tools/validate_v092_html_observatory_roster.mjs"
    ],
    "parallel_group": "browser",
    "defer_reason": "Deferred until #113 implementation creates the dedicated validator after #83 hands off the Observatory paths and open #142 lands the required Runtime identity and topology contract; validation must fail closed if the validator is absent or does not exercise the exact live candidate."
  },
  {
    "lane": "roster-focused-clippy",
    "proof_role": "Reject Rust type, ownership, serialization, route, and dead-code regressions across the bounded Runtime kernel library and production binary.",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "--bin",
      "adl-runtime-kernel",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "static",
    "defer_reason": null
  },
  {
    "lane": "issue-diff-hygiene",
    "proof_role": "Reject malformed whitespace and patch artifacts before exact-head review.",
    "acceptance_ids": [
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "static",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test agent_roster --no-tests=fail`
- `node adl/tools/validate_v092_html_observatory_roster.mjs`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --lib --bin adl-runtime-kernel -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed on unresolved serial gates, path ownership overlap, stale or contradictory Runtime evidence, unauthorized visibility, token/cursor tamper or gaps, unstable identity, stale ownership, unbounded scale, browser authority, zero-test selection, failed exact proof, or unresolved exact-head findings.

## Handoff

Retain typed evidence before convergence.
