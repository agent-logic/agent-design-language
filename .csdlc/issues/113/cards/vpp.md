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
    "lane": "observatory-roster-runtime-contract",
    "proof_role": "Use the existing exact nonzero control integration target as the ready-phase issue denominator for authenticated roster/detail routes, policy-safe serialization, WSS revision updates, pagination, presence/freshness, denial, reconnect, OpenAPI parity, and large-population bounds added during #113 execution.",
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
    "budget_tokens": 12000,
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
    "lane": "distributed-projection-input-contract",
    "proof_role": "Re-run the exact existing distributed_projection target to prove stable identity, deterministic topology ordering, redaction, stale-state handling, placement, and migration inputs remain compatible with the #113 projection adapter.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
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
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_projection",
      "--no-tests=fail"
    ],
    "parallel_group": "runtime-input",
    "defer_reason": null
  },
  {
    "lane": "html-observatory-contract",
    "proof_role": "Run the checked-in HTML Observatory Runtime v3 contract test for feed consumption, event handling, signed-control boundary, endpoint selection, and browser-side rejection behavior; execution must add the dedicated live roster browser validator through typed VPP replan once #83/#142 outputs are available.",
    "acceptance_ids": [
      "AC-4",
      "AC-6",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "adl/tools/test_html_observatory.sh"
    ],
    "parallel_group": "browser-contract",
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

- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test control --no-tests=fail`
- `cargo nextest run --manifest-path adl-runtime/Cargo.toml --test distributed_projection --no-tests=fail`
- `adl/tools/test_html_observatory.sh`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --lib --bin adl-runtime-kernel -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed on unresolved serial gates, path ownership overlap, stale or contradictory Runtime evidence, unauthorized visibility, token/cursor tamper or gaps, unstable identity, stale ownership, unbounded scale, browser authority, zero-test selection, failed exact proof, or unresolved exact-head findings.

## Handoff

Retain typed evidence before convergence.
