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
    "lane": "conversation-rooms-runtime-contract",
    "proof_role": "Run the exact issue-owned conversation_rooms Rust integration target with a nonzero-test requirement to prove bounded room membership revisions, exact frozen recipient sets, whole-set authorization before dispatch, deterministic fan-out and aggregate outcomes, attributed responses, replay and reorder rejection, revocation, timeout, cancellation, reconnect, and restart behavior.",
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
      "conversation_rooms",
      "--no-tests=fail"
    ],
    "parallel_group": "runtime",
    "defer_reason": "Deferred and fail closed during preparation: the checked-in target selects one failing sentinel test until #111, #112, and #113 are terminal, merged, ancestral, and handed off; issue #115 execution must replace the sentinel with nonzero proving tests."
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
    "lane": "rooms-browser-validator",
    "proof_role": "Run the dedicated issue-owned rooms browser validator against live Runtime truth and require nonzero room assertions for authenticated room transport, exact room and participant lists, frozen recipients, transcript and attributed responses, composer behavior, delivery and partial-delivery states, refusal and failure states, disconnect, reconnect, and accessible UI projection.",
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
      "node",
      "adl/tools/validate_v092_html_observatory_rooms.mjs"
    ],
    "parallel_group": "browser-contract",
    "defer_reason": "Deferred and fail closed during preparation: the checked-in validator selects one failing sentinel test until #111, #112, and #113 are terminal, merged, ancestral, and handed off; issue #115 execution must replace the sentinel with nonzero live room assertions."
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
    "proof_role": "Reject malformed whitespace and patch artifacts before exact-head review without claiming review acceptance.",
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
    "lane": "exact-head-review-receipt",
    "proof_role": "Record the independent security and correctness review for the exact candidate revision through typed csdlc-review; only a current passing SRP receipt with no unresolved actionable findings satisfies AC-10.",
    "acceptance_ids": [
      "AC-10"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "run",
      "--quiet",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--bin",
      "csdlc-review",
      "--",
      "--root",
      ".",
      "record",
      "--request",
      ".git/csdlc-v2/requests/115.json"
    ],
    "parallel_group": "pre-publication-review",
    "defer_reason": "Runs only after implementation and all product validation lanes pass at one candidate revision; the typed review record and SRP remain the sole acceptance authority."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test conversation_rooms --no-tests=fail`
- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test protocol_adapters --no-tests=fail`
- `cargo nextest run --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract --no-tests=fail`
- `node adl/tools/validate_v092_html_observatory_rooms.mjs`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --lib --bin adl-runtime-kernel -- -D warnings`
- `git diff --check`
- `cargo run --quiet --manifest-path csdlc-v2/Cargo.toml --bin csdlc-review -- --root . record --request .git/csdlc-v2/requests/115.json`

## Failure Semantics

Fail closed on unresolved serial gates, ownership overlap, stale revisions, implicit or widened recipients, authorization ambiguity, unattributed responses, nondeterministic partial delivery, replay conflict, event gaps, restart uncertainty, unbounded resources, forbidden data, failed exact proof, or unresolved exact-head findings. Run only the declared focused issue proofs locally. Permit at most one required big-runner job for issue #115 after publication; do not launch optional, soak, slow, duplicate, hosted coverage, or unrelated workflows.

## Handoff

Retain typed evidence before convergence.
