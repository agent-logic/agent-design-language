# Validation Planning Prompt

Template: 1.0.0

Issue: 270

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/270/design.md

Diagram: .csdlc/prepared/issues/270/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-270-preparation-validator",
    "proof_role": "Prove #270 remains open, ready/unbound before bind, dependency-terminal on #112/#265, and scoped to trusted recipient acknowledgement Runtime API only.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/270/validate_preparation_bundle.py"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-doctor-270",
    "proof_role": "Prove #270 lifecycle/card packet is mechanically coherent before and after bind.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "270"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-validate-270",
    "proof_role": "Prove rendered #270 card structure is valid.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      ".",
      "issue",
      "--issue",
      "270"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-ack-api-focused",
    "proof_role": "Focused Runtime tests proving production served acknowledgement route, verify-before-side-effects, credential-generation binding, refusal/delivery distinction, and correlation redaction.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "recipient_ack",
      "--",
      "--nocapture"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-ack-api-strict-clippy",
    "proof_role": "Strict warning-free proof for touched Runtime acknowledgement API surfaces.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
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

- `python3 .csdlc/prepared/issues/270/validate_preparation_bundle.py`
- `.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 270`
- `.adl/bin/csdlc-v2/csdlc-validate --root . issue --issue 270`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib recipient_ack -- --nocapture`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed: #270 may bind and implement only while #112/#265 terminal caches validate and their merge SHAs remain ancestral to current main; any failed preparation validator, doctor, focused Runtime proof, strict lint, fresh exact-head review, publication check, or required CI gate blocks publication/finish until repaired through typed v2.

## Handoff

Retain typed evidence before convergence.
