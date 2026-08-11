# Validation Planning Prompt

Template: 1.0.0

Issue: 237

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/237/design.md

Diagram: .csdlc/prepared/issues/237/diagram.mmd

## Selected Lanes

[
  {
    "lane": "continuity-composition-focused",
    "proof_role": "Prove the updated public capability API, continuity substitutions, and exact reviewed source in the one required CI job.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 1200,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--locked",
      "--test",
      "capability_envelope"
    ],
    "parallel_group": "required",
    "defer_reason": null
  },
  {
    "lane": "continuity-cognitive-authority",
    "proof_role": "In the same required CI job, prove real signed composition plus retained cognitive authority and privacy negatives.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 1200,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--locked",
      "--lib"
    ],
    "parallel_group": "required",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --locked --test capability_envelope`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --locked --lib`

## Failure Semantics

Fail closed on any unverified continuity record, identity mismatch, digest substitution, authority drift, privacy regression, missing exact review, or non-green required CI.

## Handoff

Retain typed evidence before convergence.
