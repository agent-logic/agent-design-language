# Validation Planning Prompt

Template: 1.0.0

Issue: 300

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the production-backed focused integration matrix, exact prerequisite regression lanes, strict all-target Clippy, and later hosted required checks.

## Lane Inputs

Design: .csdlc/prepared/issues/300/design.md

Diagram: .csdlc/prepared/issues/300/diagram.mmd

## Selected Lanes

[
  {
    "lane": "projection-recovery-integration",
    "proof_role": "Prove AC-1 through AC-5 and retain observed command/result truth for AC-7.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 7000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "projection_recovery_integration"
    ],
    "parallel_group": "local",
    "defer_reason": "Blocked until terminal and ancestral #299."
  },
  {
    "lane": "projection-recovery-regression",
    "proof_role": "Keep terminal #298/#299 and existing #291 recovery behavior green for AC-6.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5"
    ],
    "parallel_group": "local",
    "defer_reason": "Blocked until terminal and ancestral #299; gate5 is read-only for #300."
  },
  {
    "lane": "csdlc-v2-strict-clippy",
    "proof_role": "Reject integration-target warning regressions for AC-8.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--workspace",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "local",
    "defer_reason": "Blocked until terminal and ancestral #299."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test projection_recovery_integration`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --workspace --all-targets -- -D warnings`

## Failure Semantics

Fail closed before bind on missing/stale terminal ancestry and during proof on production API insufficiency, collision, ambiguity, fabricated authority, nondeterministic ordering, evidence loss, or unresolved findings.

## Handoff

Retain typed evidence before convergence.
