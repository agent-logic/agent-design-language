# Validation Planning Prompt

Template: 1.0.0

Issue: 115

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/115/design.md

Diagram: .csdlc/prepared/issues/115/diagram.mmd

## Selected Lanes

[
  {
    "lane": "rooms-runtime-contract",
    "proof_role": "Use the existing exact nonzero control integration target as the ready-phase denominator for authenticated room routes, exact participants and recipients, deterministic delivery events, refusal, reconnect, replay, and OpenAPI behavior added during #115 execution; execution must add the dedicated conversation_rooms target through typed VPP replan.",
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
    "lane": "acip-routing-input-contract",
    "proof_role": "Re-run the exact existing protocol_adapters target to prove bounded authenticated ACIP-compatible routing, replay rejection, timeout, cancellation, retry, response bounds, and shutdown inputs remain compatible with the #115 fan-out adapter.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7"
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
      "protocol_adapters",
      "--no-tests=fail"
    ],
    "parallel_group": "runtime-input",
    "defer_reason": null
  },
  {
    "lane": "observatory-room-openapi",
    "proof_role": "Use the existing exact OpenAPI contract target for checked-in room, participant, recipient, delivery, response, replay, and refusal schema parity after #115 integration.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
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
    "parallel_group": "runtime",
    "defer_reason": null
  },
  {
    "lane": "html-observatory-contract",
    "proof_role": "Run the checked-in executable HTML Observatory Runtime v3 contract; execution must add the dedicated live room browser validator through typed VPP replan once #111-#113 outputs are available.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-7",
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
    "lane": "rooms-focused-clippy",
    "proof_role": "Reject warning-bearing Rust across the bounded Runtime kernel library and production binary after room integration.",
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
- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test protocol_adapters --no-tests=fail`
- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract --no-tests=fail`
- `adl/tools/test_html_observatory.sh`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --lib --bin adl-runtime-kernel -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed on unresolved serial gates, ownership overlap, stale revisions, implicit or widened recipients, authorization ambiguity, unattributed responses, nondeterministic partial delivery, replay conflict, event gaps, restart uncertainty, unbounded resources, forbidden data, failed exact proof, or unresolved exact-head findings.

## Handoff

Retain typed evidence before convergence.
