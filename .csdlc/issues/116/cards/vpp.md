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
    "lane": "attention-api-browser-validator",
    "proof_role": "Run the dedicated issue-owned API/browser validator against live Runtime truth and require nonzero attention-inbox assertions for authorized listing, unread projection, filters, deep links, acknowledge, reply, defer, resolve, refuse, notification preferences, refusal and degradation, restart, reconnect, stale cache, and duplicate suppression.",
    "acceptance_ids": [
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
      "bash",
      "adl/tools/test_v092_operator_attention_inbox.sh"
    ],
    "parallel_group": "attention-api-browser",
    "defer_reason": "Deferred and fail closed during preparation: #111, #112, and #114 must be terminal, merged, ancestral, and handed off before issue #116 implements this exact validator target; a missing validator, unavailable live Runtime API/browser surface, skipped proof, or zero attention-inbox assertions must fail."
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
- `bash adl/tools/test_v092_operator_attention_inbox.sh`
- `git diff --check`

## Failure Semantics

Fail closed on identity, authority, topology, schema, queue, rate, expiry, deduplication, durability, recovery, or projection ambiguity. Do not bind while #111/#112/#114 are non-terminal; do not treat deferred, skipped, fixture-only, or browser-only evidence as product proof.

## Handoff

Retain typed evidence before convergence.
