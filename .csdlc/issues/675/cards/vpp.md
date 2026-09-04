# Validation Planning Prompt

Template: 1.0.0

Issue: 675

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/675/design.md

Diagram: .csdlc/prepared/issues/675/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-a2a-live-style",
    "proof_role": "Prove a live-style Beacon-to-Ember initiation path through the model/shepherd action bridge.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "agent_to_agent"
    ],
    "parallel_group": "runtime-a2a",
    "defer_reason": "Exact test names are implementation deliverables."
  },
  {
    "lane": "observatory-ui-contract",
    "proof_role": "Prove the Observatory path emits/sends/renders first-class A2A frames distinctly from operator chat.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 1000,
    "argv": [
      "node",
      "--check",
      "demos/html-observatory/app.js"
    ],
    "parallel_group": "ui",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Prove exact branch diff hygiene before review.",
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
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib agent_to_agent`
- `node --check demos/html-observatory/app.js`
- `git diff --check`

## Failure Semantics

Fail closed if first-class A2A remains prompt-only, if authority checks are weakened, if recipient/provider results are misattributed, or if live/paid execution becomes required without authorization.

## Handoff

Retain typed evidence before convergence.
