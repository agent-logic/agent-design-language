# Validation Planning Prompt

Template: 1.0.0

Issue: 3

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/3/design.md

Diagram: .csdlc/prepared/issues/3/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-publication-focused",
    "proof_role": "Focused Rust tests prove split identities, effective remote verification, PR reconciliation, compatibility, and schemas.",
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
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate6",
      "--test",
      "gate_finish",
      "--bin",
      "csdlc-publish"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "split-authority-canary",
    "proof_role": "A deterministic local validator checks retained live GitHub identity, linkage, terminal, merge, and ancestry evidence for canonical PR #5 and preserved issue #5901.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/3/validate-split-authority-canary.rb"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate6 --test gate_finish --bin csdlc-publish`
- `ruby .csdlc/prepared/issues/3/validate-split-authority-canary.rb`

## Failure Semantics

Fail closed on repository ambiguity, effective push substitution, stale review, unqualified linkage, canary drift, or any attempted legacy code mutation.

## Handoff

Retain typed evidence before convergence.
