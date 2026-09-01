# Validation Planning Prompt

Template: 1.0.0

Issue: 589

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/589/authored/design.md

Diagram: .csdlc/issues/589/authored/diagram.mmd

## Selected Lanes

[
  {
    "lane": "kernel-focused",
    "proof_role": "Writer recovery, Shepherd readiness, configuration, and health-export regression proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "parallel_group": "local-runtime",
    "defer_reason": null
  },
  {
    "lane": "guardian-csm-focused",
    "proof_role": "Guardian supervision and ordered CSM lifecycle regression proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--lib",
      "cli::csm_runtime_v3_cmd::tests"
    ],
    "parallel_group": "local-runtime",
    "defer_reason": null
  },
  {
    "lane": "terraform-static",
    "proof_role": "Bounded AWS health-recovery infrastructure syntax and plan proof",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1500,
    "argv": [
      "terraform",
      "validate"
    ],
    "parallel_group": "infra",
    "defer_reason": null
  },
  {
    "lane": "wuji-live",
    "proof_role": "Identity-bound HTTPS, Shepherd admission, CloudWatch alarm, and SSM recovery proof",
    "acceptance_ids": [
      "AC-1",
      "AC-6",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 2000,
    "argv": [
      "csm",
      "runtime-v3",
      "status",
      "--init",
      ".adl/runtime-v3/live/runtime-init.toml"
    ],
    "parallel_group": "live",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml`
- `cargo test --locked --manifest-path adl/Cargo.toml --lib cli::csm_runtime_v3_cmd::tests`
- `terraform validate`
- `csm runtime-v3 status --init .adl/runtime-v3/live/runtime-init.toml`

## Failure Semantics

Fail closed on live-writer ambiguity, retained-state incompatibility, failed candidate configuration validation, or unstable readiness.

## Handoff

Retain typed evidence before convergence.
