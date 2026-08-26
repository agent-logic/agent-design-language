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
    "proof_role": "Prove deterministic production-admission-backed roster projection, policy filtering, pagination, stable identity, event-owned freshness, relocation, revision ordering, and bounded large-Polis behavior.",
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
    "defer_reason": null
  },
  {
    "lane": "observatory-control-wss-parity",
    "proof_role": "Prove the production roster route, admission and component-state conjunction, server-authoritative policy projection, pagination, and revision behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "control",
      "--no-tests=fail"
    ],
    "parallel_group": "runtime",
    "defer_reason": null
  },
  {
    "lane": "observatory-openapi-parity",
    "proof_role": "Prove the versioned roster route and schema match the production implementation without invented authorities.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "openapi_contract",
      "--no-tests=fail"
    ],
    "parallel_group": "runtime-contract",
    "defer_reason": null
  },
  {
    "lane": "observatory-roster-browser",
    "proof_role": "Drive the exact Runtime-backed browser flow for truthful identity and presence, pagination, search, selection, revision/cursor fencing, reconnect without duplicates, responsive layout, and clean console behavior.",
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
    "defer_reason": null
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
      "AC-9"
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
  },
  {
    "lane": "independent-exact-head-review",
    "proof_role": "Require an independent reviewer to inspect the exact clean candidate and leave no unresolved actionable findings before publication.",
    "acceptance_ids": [
      "AC-10"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-review",
      "guard",
      "--request",
      ".git/csdlc-v2/requests/113-review.json"
    ],
    "parallel_group": "pre-publication-review",
    "defer_reason": "Runs after all deterministic product lanes are current; independent review remains separate from implementation-owned validation."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test agent_roster --no-tests=fail`
- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test control --no-tests=fail`
- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract --no-tests=fail`
- `node adl/tools/validate_v092_html_observatory_roster.mjs`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --lib --bin adl-runtime-kernel -- -D warnings`
- `git diff --check`
- `.adl/bin/csdlc-v2/csdlc-review guard --request .git/csdlc-v2/requests/113-review.json`

## Failure Semantics

Fail closed on unresolved serial gates, path ownership overlap, stale or contradictory Runtime evidence, unauthorized visibility, token/cursor tamper or gaps, unstable identity, stale ownership, unbounded scale, browser authority, zero-test selection, failed exact proof, or unresolved exact-head findings.

## Handoff

Retain typed evidence before convergence.
