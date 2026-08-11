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
    "proof_role": "Deterministic issue-specific proof of schemas, ordering, idempotency, failure outcomes, reconnect, and restart boundaries",
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
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "conversation_sessions"
    ],
    "parallel_group": "runtime_core",
    "defer_reason": "Issue-owned integration target is intentionally authored during execution after terminal #83 topology is available."
  },
  {
    "lane": "observatory_wss_integration",
    "proof_role": "Deterministic authenticated WSS ingress, delivery/response correlation, reconnect, and negative-path proof",
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
    "proof_role": "Deterministic public schema and checked-in OpenAPI parity proof",
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
    "lane": "observatory_javascript_syntax",
    "proof_role": "Fast deterministic syntax guard for the non-authoritative browser client integration",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "node",
      "--check",
      "demos/html-observatory/app.js"
    ],
    "parallel_group": "client_static",
    "defer_reason": null
  },
  {
    "lane": "exact_diff_hygiene",
    "proof_role": "Fast exact-worktree whitespace and conflict-marker hygiene before independent review",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
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
    "parallel_group": "client_static",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test conversation_sessions`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test observatory`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract`
- `node --check demos/html-observatory/app.js`
- `git diff --check`

## Failure Semantics

Fail closed: no lane passes on skip, pending, timeout, missing target after execution begins, unavailable credentials, nondeterministic retry, or partial output. Stop implementation on any failed lane, stale #83 ancestry, authority widening, unbounded/provider-private projection, or unresolved actionable review finding.

## Handoff

Retain typed evidence before convergence.
