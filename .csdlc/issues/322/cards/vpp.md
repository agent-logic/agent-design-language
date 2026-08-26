# Validation Planning Prompt

Template: 1.0.0

Issue: 322

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5913/design.md

Diagram: .csdlc/prepared/issues/5913/diagram.mmd

## Selected Lanes

[
  {
    "lane": "adl-review-compatibility",
    "proof_role": "Focused compatibility regression for adl-review help/dispatch and CodeFriend deterministic smoke routing",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "bash",
      "adl/tools/test_adl_review_compatibility.sh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "strict-clippy-adl-review",
    "proof_role": "Strict relevant Rust lint for adl-review dispatch changes",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl-review",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "fresh-exact-head-review",
    "proof_role": "Typed fresh-session exact-head review gate before publication; not a Rust lint proof",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 5000,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-review",
      "--root",
      ".",
      "assign",
      "--request",
      ".git/csdlc-v2/requests/5913-review-assign.json"
    ],
    "parallel_group": "review",
    "defer_reason": "Runs only after #5913 reaches implemented phase with a clean substantive exact head."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_adl_review_compatibility.sh`
- `cargo clippy --manifest-path adl/Cargo.toml --bin adl-review -- -D warnings`
- `.adl/bin/csdlc-v2/csdlc-review --root . assign --request .git/csdlc-v2/requests/5913-review-assign.json`

## Failure Semantics

Fail closed if a command would route through removed v1 lifecycle logic, mutate active lifecycle state, require provider credentials, or overclaim CodeFriend product completion.

## Handoff

Retain typed evidence before convergence.
