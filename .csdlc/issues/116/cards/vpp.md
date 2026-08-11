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
    "lane": "attention-runtime-contract",
    "proof_role": "Run the exact nonzero issue-owned Runtime target for lifecycle, schema, queue ordering, deduplication, expiry, authorization, overload, restart, and recovery.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 7000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "operator_attention"
    ],
    "parallel_group": "attention-runtime",
    "defer_reason": "The issue-owned adl-runtime/tests/operator_attention.rs target is created only after #111, #112, and #114 are terminal and #116 is bound; execution must fail closed if absent or zero-test."
  },
  {
    "lane": "attention-observatory-browser",
    "proof_role": "Run the issue-owned live Runtime-backed browser proof for inbox accessibility, unread/filter/deep-link behavior, authorization, overload, degradation, reconnect, and duplicate-notification prevention.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 7000,
    "argv": [
      "adl/tools/test_v092_operator_attention_inbox.sh"
    ],
    "parallel_group": "attention-browser",
    "defer_reason": "The issue-owned browser validator is created with #116 implementation after serial gates pass; execution must fail closed if absent or non-executable."
  },
  {
    "lane": "html-observatory-existing-contract",
    "proof_role": "Preserve the checked-in HTML Observatory Runtime v3 feed, event, signed-control, endpoint-selection, and browser rejection contract during later integration.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6"
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

- `cargo test --manifest-path adl-runtime/Cargo.toml --test operator_attention`
- `adl/tools/test_v092_operator_attention_inbox.sh`
- `adl/tools/test_html_observatory.sh`
- `git diff --check`

## Failure Semantics

Fail closed on identity, authority, topology, schema, queue, rate, expiry, deduplication, durability, recovery, or projection ambiguity. Do not bind while #111/#112/#114 are non-terminal; do not treat deferred, skipped, fixture-only, or browser-only evidence as product proof.

## Handoff

Retain typed evidence before convergence.
