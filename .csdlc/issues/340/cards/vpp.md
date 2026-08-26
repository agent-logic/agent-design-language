# Validation Planning Prompt

Template: 1.0.0

Issue: 340

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/340/design.md

Diagram: .csdlc/prepared/issues/340/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-340-html-observatory-contract",
    "proof_role": "Prove the static HTML Observatory contract, Runtime v3 config normalization, health endpoint requirement, and route-coverage assertions without launching local services.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1200,
    "argv": [
      "bash",
      "adl/tools/validate_v092_observatory_restart_reconnect.sh",
      "--contract"
    ],
    "parallel_group": "340-local-contract",
    "defer_reason": null
  },
  {
    "lane": "issue-340-live-start-stop-restart",
    "proof_role": "Launch CSMctl against configured local TLS material, require /v1/ready, /v1/observatory, and /v1/health to return HTTP 200 before success, serve the HTML Observatory root as index.html, gracefully stop with PID/lease cleanup, restart the Runtime, and re-probe exposed read/static, OPTIONS, invalid-body POST, and WebSocket handshake routes from the documented OpenAPI surfaces.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/validate_v092_observatory_restart_reconnect.sh",
      "--live"
    ],
    "parallel_group": "340-live-serial",
    "defer_reason": null
  },
  {
    "lane": "runtime-api-wss-restart-tests",
    "proof_role": "Run focused Runtime API/WSS contract tests proving the OpenAPI route split, validator route coverage, strict 200 live-feed gating, and CSMctl root-to-index/runtime-base persistence strings.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "runtime_api_wss"
    ],
    "parallel_group": "340-rust-contract",
    "defer_reason": null
  },
  {
    "lane": "format-and-diff-hygiene",
    "proof_role": "Prove Rust formatting for the added integration test and reject whitespace/conflict artifacts across the #340 diff.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 600,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--check"
    ],
    "parallel_group": "340-hygiene",
    "defer_reason": "git diff --check is recorded separately in SOR validation because this lane has one typed argv vector."
  },
  {
    "lane": "issue-340-exact-review",
    "proof_role": "Record a fresh exact-head review with no unresolved actionable findings before publication.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-review",
      "--root",
      ".",
      "--issue",
      "340"
    ],
    "parallel_group": "340-review",
    "defer_reason": "Runs after implementation, proof, and immutable commit are current."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash adl/tools/validate_v092_observatory_restart_reconnect.sh --contract`
- `bash adl/tools/validate_v092_observatory_restart_reconnect.sh --live`
- `cargo test --manifest-path adl-runtime/Cargo.toml --test runtime_api_wss`
- `cargo fmt --manifest-path adl-runtime/Cargo.toml --check`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-review --root . --issue 340`

## Failure Semantics

Fail closed on stale dependency truth, missing live endpoint proof, fixture-only proof, restart duplicate events, stale correlation, authorization drift, redaction leak, failed validation, review finding, publication drift, or CI failure.

## Handoff

Retain typed evidence before convergence.
