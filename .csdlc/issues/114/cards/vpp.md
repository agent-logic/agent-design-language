# Validation Planning Prompt

Template: 1.0.0

Issue: 114

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Preparation-only validation proves artifact preservation, initialized/unbound parent scope, graph truth, and typed lifecycle/card consistency; it does not prove Runtime, API, Observatory, publication, merge, or terminal behavior.

## Lane Inputs

Design: .csdlc/prepared/issues/114/design.md

Diagram: .csdlc/prepared/issues/114/diagram.mmd

## Selected Lanes

[
  {
    "lane": "114-issue-owned-bound-parent-validator",
    "proof_role": "Validate the bound #114 parent identity, preserved design/diagram digests, issue-owned validator availability, and terminal ancestry for #112, #265, #270, #271, #276, #277, and #278.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/114/validate_preparation_bundle.py"
    ],
    "parallel_group": "114-parent-proof",
    "defer_reason": null
  },
  {
    "lane": "114-parent-terminal-chain-validator",
    "proof_role": "Validate #276, #277, and #278 derived-terminal caches, merged dispositions, canonical generation/digest fields, merge-SHA ancestry, parent-only ownership, and preserved boundary truth at the exact #114 parent proof head.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      "adl/tools/validate_v092_durable_history_parent_integration.py"
    ],
    "parallel_group": "114-parent-proof",
    "defer_reason": null
  },
  {
    "lane": "114-parent-runtime-kernel-integration-test",
    "proof_role": "Prove integrated durable-history parent behavior across history, continuity, journal retention/deletion, restart, duplicate attempt admission, receipts, replay owner state, and Observatory transcript restoration.",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "durable_conversation_history_integration",
      "--",
      "--nocapture"
    ],
    "parallel_group": "114-parent-proof",
    "defer_reason": null
  },
  {
    "lane": "114-parent-hygiene",
    "proof_role": "Check strict relevant Clippy and diff hygiene for the parent-only proof surface before exact-head review.",
    "acceptance_ids": [
      "AC-7",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "durable_conversation_history_integration",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "114-parent-proof",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 21600

Tokens: 100000

## Commands

- `python3 .csdlc/prepared/issues/114/validate_preparation_bundle.py`
- `python3 adl/tools/validate_v092_durable_history_parent_integration.py`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test durable_conversation_history_integration -- --nocapture`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --test durable_conversation_history_integration -- -D warnings`

## Failure Semantics

Fail closed on projection drift, stale digest, corrupt audit, missing preserved design/diagram evidence, accidental parent implementation scope, attempted bind/publication/PR/GitHub mutation, dependency graph mismatch, or issue-local preparation validator failure.

## Handoff

Retain typed evidence before convergence.
