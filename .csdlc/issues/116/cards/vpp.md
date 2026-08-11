# Validation Planning Prompt

Template: 1.0.0

Issue: 116

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/116/design.md

Diagram: .csdlc/prepared/issues/116/diagram.mmd

## Selected Lanes

[
  {
    "lane": "attention-exact-child-tests",
    "proof_role": "Exact nonzero issue-owned target proves lifecycle, schema, authorization, queue bounds, deduplication, expiry, overload, spoofing, restart, reconnect, recovery, projection, and operator outcome invariants.",
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
    "budget_seconds": 1200,
    "budget_tokens": 12000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "operator_attention",
      "--no-tests=fail"
    ],
    "parallel_group": "attention-runtime",
    "defer_reason": "The issue-owned temporary #[path = \"../src/operator_attention.rs\"] harness in adl-runtime/tests/operator_attention.rs will route adl-runtime/src/operator_attention.rs until integration registration; --no-tests=fail preserves the nonzero requirement."
  },
  {
    "lane": "html-observatory-existing-contract",
    "proof_role": "Preserve the checked-in HTML Observatory Runtime v3 feed, event, signed-control, endpoint-selection, and browser rejection baseline before the dedicated #116 browser validator is added through typed VPP replan after bind.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "adl/tools/test_html_observatory.sh"
    ],
    "parallel_group": "attention-browser-contract",
    "defer_reason": null
  },
  {
    "lane": "attention-diff-hygiene",
    "proof_role": "Reject malformed whitespace and patch artifacts before exact-head review.",
    "acceptance_ids": [
      "AC-7"
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
    "parallel_group": "attention-static",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo nextest run --manifest-path adl-runtime/Cargo.toml --test operator_attention --no-tests=fail`
- `adl/tools/test_html_observatory.sh`
- `git diff --check`

## Failure Semantics

Fail closed on identity, authority, topology, schema, queue, rate, expiry, deduplication, durability, recovery, or projection ambiguity. Do not bind while #111/#112/#114 are non-terminal; do not treat deferred, skipped, fixture-only, or browser-only evidence as product proof.

## Handoff

Retain typed evidence before convergence.
