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
    "lane": "runtime-governed-room-model",
    "proof_role": "Prove explicit governed-room recipients, no implicit broadcast, membership/policy refusal, ordering/replay rejection, authority-scope reuse, and accepted-vs-delivered distinction in the Runtime room model.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "conversation_rooms",
      "--",
      "--nocapture"
    ],
    "parallel_group": "local-runtime",
    "defer_reason": null
  },
  {
    "lane": "runtime-governed-room-served-route",
    "proof_role": "Prove the served Runtime governed-room WebSocket route accepts explicit recipients, rejects implicit broadcast without consuming sequence, and returns accepted rather than fabricated delivered state.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "governed_room",
      "--",
      "--nocapture"
    ],
    "parallel_group": "local-runtime",
    "defer_reason": null
  },
  {
    "lane": "observatory-governed-room-ui",
    "proof_role": "Prove the HTML Observatory room composer uses explicit bounded recipients, stable per-room turn sequences, accepted route rows that do not claim delivery, partial/refused/unavailable/revoked delivery rendering, static DOM anchors, and served frame dispatch.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1200,
    "argv": [
      "node",
      "adl/tools/validate_v092_governed_room_observatory.mjs"
    ],
    "parallel_group": "local-ui",
    "defer_reason": null
  },
  {
    "lane": "html-observatory-smoke",
    "proof_role": "Prove the existing HTML Observatory Runtime v3, signed-command, roster projection, and control-frame smoke still passes after the governed-room UI changes.",
    "acceptance_ids": [
      "AC-4",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/test_html_observatory.sh"
    ],
    "parallel_group": "local-ui",
    "defer_reason": null
  },
  {
    "lane": "format-clippy-diff-hygiene",
    "proof_role": "Prove formatting, warning-free Runtime library code, and diff hygiene for the #115 touched surfaces.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "parallel_group": "local-runtime",
    "defer_reason": "Strict clippy and git diff --check are recorded in SOR validation because this lane has a single argv vector."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib conversation_rooms -- --nocapture`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib governed_room -- --nocapture`
- `node adl/tools/validate_v092_governed_room_observatory.mjs`
- `bash adl/tools/test_html_observatory.sh`
- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check`

## Failure Semantics

Fail closed on graph mismatch, missing #270 marker, unexpected lifecycle state, missing or nonterminal dependency cache, non-ancestral dependency merge SHA, or design/readiness review findings.

## Handoff

Retain typed evidence before convergence.
