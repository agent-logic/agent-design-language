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
    "lane": "kernel-assembly-focused",
    "proof_role": "Production assembly, writer recovery, and Shepherd lease proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "assembly"
    ],
    "parallel_group": "local-runtime",
    "defer_reason": null
  },
  {
    "lane": "kernel-configuration-focused",
    "proof_role": "Configuration identity and compatible hot-reload proof",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "configuration"
    ],
    "parallel_group": "local-runtime",
    "defer_reason": null
  },
  {
    "lane": "kernel-control-focused",
    "proof_role": "Authenticated ownership and Shepherd-gated readiness proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "control"
    ],
    "parallel_group": "local-runtime",
    "defer_reason": null
  },
  {
    "lane": "kernel-observability-focused",
    "proof_role": "Bounded health heartbeat and observability recovery proof",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "observability"
    ],
    "parallel_group": "local-runtime",
    "defer_reason": null
  },
  {
    "lane": "guardian-lifecycle-focused",
    "proof_role": "Single Guardian child and lease identity lifecycle proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "runtime_guardian_lifecycle"
    ],
    "parallel_group": "local-runtime",
    "defer_reason": null
  },
  {
    "lane": "guardian-csm-focused",
    "proof_role": "Ordered CSM lifecycle, interrupted-reload, process ownership, and transaction proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl",
      "csm_runtime_v3"
    ],
    "parallel_group": "local-runtime",
    "defer_reason": null
  },
  {
    "lane": "coverage-impact-contract-focused",
    "proof_role": "Focused coverage-impact routing contract proof",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "bash",
      "adl/tools/test_check_coverage_impact.sh"
    ],
    "parallel_group": "local-tooling",
    "defer_reason": null
  },
  {
    "lane": "terraform-static",
    "proof_role": "Bounded AWS health-recovery infrastructure syntax proof",
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
      "-chdir=infra/aws/csm-runtime-health",
      "validate"
    ],
    "parallel_group": "infra",
    "defer_reason": null
  },
  {
    "lane": "wuji-runtime-ready-live",
    "proof_role": "Live identity-bound Runtime HTTPS readiness proof",
    "acceptance_ids": [
      "AC-1",
      "AC-6",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "curl",
      "-ksS",
      "https://127.0.0.1:20997/v1/ready"
    ],
    "parallel_group": "live",
    "defer_reason": null
  },
  {
    "lane": "wuji-shepherd-live",
    "proof_role": "Live Shepherd admission freshness and reviewed source identity proof",
    "acceptance_ids": [
      "AC-1",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "curl",
      "-ksS",
      "https://127.0.0.1:20997/v1/agents"
    ],
    "parallel_group": "live",
    "defer_reason": null
  },
  {
    "lane": "aws-health-alarm-live",
    "proof_role": "Live CloudWatch missing-heartbeat alarm state proof",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "aws",
      "cloudwatch",
      "describe-alarms",
      "--alarm-names",
      "adl-axioma-wuji-dev-runtime-health-missing",
      "--profile",
      "agent-logic-admin"
    ],
    "parallel_group": "live",
    "defer_reason": null
  },
  {
    "lane": "aws-governed-reload-receipt-live",
    "proof_role": "Live governed SSM reload receipt proof",
    "acceptance_ids": [
      "AC-3",
      "AC-6",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "aws",
      "ssm",
      "get-command-invocation",
      "--command-id",
      "60136b40-8e73-4a7c-bb45-46a60780641f",
      "--instance-id",
      "mi-0dd41a2b1cad222a0",
      "--profile",
      "agent-logic-admin"
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

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test assembly`
- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test configuration`
- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test control`
- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test observability`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test runtime_guardian_lifecycle`
- `cargo test --locked --manifest-path adl/Cargo.toml --bin adl csm_runtime_v3`
- `bash adl/tools/test_check_coverage_impact.sh`
- `terraform -chdir=infra/aws/csm-runtime-health validate`
- `curl -ksS https://127.0.0.1:20997/v1/ready`
- `curl -ksS https://127.0.0.1:20997/v1/agents`
- `aws cloudwatch describe-alarms --alarm-names adl-axioma-wuji-dev-runtime-health-missing --profile agent-logic-admin`
- `aws ssm get-command-invocation --command-id 60136b40-8e73-4a7c-bb45-46a60780641f --instance-id mi-0dd41a2b1cad222a0 --profile agent-logic-admin`

## Failure Semantics

Fail closed on live-writer ambiguity, retained-state incompatibility, failed candidate configuration validation, or unstable readiness.

## Handoff

Retain typed evidence before convergence.
