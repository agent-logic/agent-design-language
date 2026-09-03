# Validation Planning Prompt

Template: 1.0.0

Issue: 515

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/515/design.md

Diagram: .csdlc/prepared/issues/515/diagram.mmd

## Selected Lanes

[
  {
    "lane": "shadow-isolation",
    "proof_role": "Prove authority and shadow paths are distinguishable and shadow output cannot mutate authoritative state.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "-p",
      "adl",
      "--test",
      "provider_shadow_isolation"
    ],
    "parallel_group": "provider-shadow",
    "defer_reason": "The named integration test is a #515 implementation deliverable."
  },
  {
    "lane": "deterministic-comparison",
    "proof_role": "Prove exact deterministic comparison inputs and rules for authority-versus-shadow observations.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "-p",
      "adl",
      "--test",
      "provider_shadow_comparison"
    ],
    "parallel_group": "provider-shadow",
    "defer_reason": "The named integration test is a #515 implementation deliverable."
  },
  {
    "lane": "fallback",
    "proof_role": "Prove shadow failures preserve the authoritative result and do not alter authority state.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "-p",
      "adl",
      "--test",
      "provider_shadow_fallback"
    ],
    "parallel_group": "provider-shadow",
    "defer_reason": "The named integration test is a #515 implementation deliverable."
  },
  {
    "lane": "redaction",
    "proof_role": "Prove shadow comparison evidence is redacted and does not expose credentials, private payloads, prompts, or host-local paths.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/515/validate-provider-shadow-redaction.sh"
    ],
    "parallel_group": "evidence",
    "defer_reason": "The issue-owned redaction wrapper is a #515 implementation deliverable."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Prove exact branch diff hygiene before review.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
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

- `cargo test --manifest-path adl/Cargo.toml -p adl --test provider_shadow_isolation`
- `cargo test --manifest-path adl/Cargo.toml -p adl --test provider_shadow_comparison`
- `cargo test --manifest-path adl/Cargo.toml -p adl --test provider_shadow_fallback`
- `bash .csdlc/prepared/issues/515/validate-provider-shadow-redaction.sh`
- `git diff --check`

## Failure Semantics

Fail closed on authority mutation, drifted comparison inputs, shadow-output promotion, unredacted evidence, production cutover, or unauthorized paid/live-provider use.

## Handoff

Retain typed evidence before convergence.
