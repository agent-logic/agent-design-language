# Validation Planning Prompt

Template: 1.0.0

Issue: 708

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/708/design.md

Diagram: .csdlc/prepared/issues/708/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-orientation-contract",
    "proof_role": "Prove resource validation, delivered-byte digesting, admission ordering, per-agent retention, and reload semantics.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "orientation"
    ],
    "parallel_group": "runtime",
    "defer_reason": "The issue-owned focused Runtime test selector will be added during implementation before PVF execution."
  },
  {
    "lane": "runtime-projection-observatory",
    "proof_role": "Prove Runtime projection and Observatory rendering of exact per-agent orientation provenance.",
    "acceptance_ids": [
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "node",
      "demos/html-observatory/tests/agent_orientation.test.mjs"
    ],
    "parallel_group": "observatory",
    "defer_reason": "The issue-owned Observatory test target will be added during implementation before PVF execution."
  },
  {
    "lane": "planning-contract",
    "proof_role": "Prove the issue-owned design, diagram, source input, and typed lifecycle records are present and structurally clean.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 200,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/708/validate-orientation-plan.sh"
    ],
    "parallel_group": "hygiene",
    "defer_reason": null
  },
  {
    "lane": "source-immutability",
    "proof_role": "Prove the canonical welcome-package source document is unchanged by the implementation.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 200,
    "argv": [
      "git",
      "diff",
      "--exit-code",
      "origin/main...HEAD",
      "--",
      "docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md"
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

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --lib orientation`
- `node demos/html-observatory/tests/agent_orientation.test.mjs`
- `bash .csdlc/prepared/issues/708/validate-orientation-plan.sh`
- `git diff --exit-code origin/main...HEAD -- docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md`

## Failure Semantics

Fail closed if orientation cannot be validated, injected before the first turn, or bound to exact per-agent delivery provenance; preserve the last valid active resource on reload failure.

## Handoff

Retain typed evidence before convergence.
