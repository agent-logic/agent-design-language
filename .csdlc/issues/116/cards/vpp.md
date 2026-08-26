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
    "lane": "issue-116-preparation-validator",
    "proof_role": "Prove #116 design remains bounded and dependency-aware before bind.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/116/validate_preparation_bundle.py"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-observatory-attention-inbox-tests",
    "proof_role": "After bind, prove attention request lifecycle, source binding, dedup/rate/expiry, restart/reconnect, and governed outcomes in the existing Observatory runtime target.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "observatory",
      "operator_attention"
    ],
    "parallel_group": "implementation",
    "defer_reason": "Deferred until typed bind adds the #116 operator_attention cases to adl-runtime-kernel/tests/observatory.rs."
  },
  {
    "lane": "html-observatory-operator-attention-tests",
    "proof_role": "After bind, prove Observatory inbox state, filters, deep links, unread state, preferences, and explicit outcome actions.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "node",
      "--test",
      "demos/html-observatory/tests/operator_attention_inbox.test.mjs"
    ],
    "parallel_group": "implementation",
    "defer_reason": "Deferred until typed bind creates the issue-owned HTML Observatory test."
  },
  {
    "lane": "operator-attention-strict-clippy",
    "proof_role": "Reject warning regressions in the existing Observatory runtime test target after #116 changes.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "observatory",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "implementation",
    "defer_reason": "Deferred until implementation updates the existing Observatory runtime target."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `python3 .csdlc/prepared/issues/116/validate_preparation_bundle.py`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test observatory operator_attention`
- `node --test demos/html-observatory/tests/operator_attention_inbox.test.mjs`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --test observatory -- -D warnings`

## Failure Semantics

Fail closed on missing dependency truth, source identity spoofing, implicit approval, alert flooding, duplicate restart notifications, scope drift, failed validation, review finding, CI failure, or nonterminal finish.

## Handoff

Retain typed evidence before convergence.
