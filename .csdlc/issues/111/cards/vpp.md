# Validation Planning Prompt

Template: 1.0.0

Issue: 111

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: design/issue-111.md

Diagram: design/issue-111.mmd

## Selected Lanes

[
  {
    "lane": "runtime_conversation_contract",
    "proof_role": "Deterministic issue-specific proof of schemas, ordering, idempotency, failure outcomes, reconnect, and restart boundaries with a required nonzero test count.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "conversation_sessions",
      "--no-tests=fail"
    ],
    "parallel_group": "runtime_core",
    "defer_reason": "The sentinel target is intentionally nonzero and failing until WP-18C.01 implements the complete conversation-session denominator."
  },
  {
    "lane": "observatory_wss_integration",
    "proof_role": "Deterministic authenticated WSS ingress, delivery/response correlation, reconnect, and negative-path proof.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "observatory"
    ],
    "parallel_group": "runtime_integration",
    "defer_reason": null
  },
  {
    "lane": "observatory_openapi_contract",
    "proof_role": "Deterministic public schema and checked-in OpenAPI parity proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5"
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
      "openapi_contract"
    ],
    "parallel_group": "runtime_integration",
    "defer_reason": null
  },
  {
    "lane": "observatory_browser_conversation_behavior",
    "proof_role": "Run the exact issue-owned browser validator and exit nonzero unless agent ACK/hash metadata is never rendered as an agent reply and reconnect does not duplicate transcript entries.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "node",
      "demos/html-observatory/tests/conversation_sessions.test.mjs"
    ],
    "parallel_group": "client_behavior",
    "defer_reason": "The sentinel exits nonzero until the exact behavioral assertions are implemented."
  },
  {
    "lane": "conversation_exact_head_review_receipts",
    "proof_role": "Validate that all required product and hygiene receipts cover the exact candidate revision and that independent SRP review resolved every actionable finding without widening scope.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "node",
      "demos/html-observatory/tests/conversation_sessions.test.mjs",
      "--review-only"
    ],
    "parallel_group": "review",
    "defer_reason": "The issue-owned validator must fail closed until exact-head receipts and independent SRP review evidence exist."
  },
  {
    "lane": "exact_diff_hygiene",
    "proof_role": "Fast exact-worktree whitespace and conflict-marker hygiene; this lane does not substitute for independent review.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "client_behavior",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test conversation_sessions --no-tests=fail`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test observatory`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract`
- `node demos/html-observatory/tests/conversation_sessions.test.mjs`
- `node demos/html-observatory/tests/conversation_sessions.test.mjs --review-only`
- `git diff --check`

## Failure Semantics

Fail closed: no lane passes on skip, pending, timeout, missing target after execution begins, unavailable credentials, nondeterministic retry, or partial output. Stop implementation on any failed lane, stale #83 ancestry, authority widening, unbounded/provider-private projection, or unresolved actionable review finding.

## Handoff

Retain typed evidence before convergence.
