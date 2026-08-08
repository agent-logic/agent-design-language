# Validation Planning Prompt

Template: 1.0.0

Issue: 55

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/55/design.md

Diagram: .csdlc/prepared/issues/55/diagram.mmd

## Selected Lanes

[
  {
    "lane": "workflow-yaml",
    "proof_role": "Parse the changed GitHub Actions workflow as valid YAML.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "ruby",
      "-e",
      "require 'yaml'; YAML.load_file('.github/workflows/ci.yaml')"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "ci-runner-contract",
    "proof_role": "Focused CI runtime and path-policy contracts prove the heavy aggregator route and preserved surrounding semantics.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "ci-path-policy-contract",
    "proof_role": "Focused path-policy proof preserves producer, Spot, stable status, and coverage routing boundaries.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/test_ci_path_policy.sh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "larger-runner-preflight-contract",
    "proof_role": "The typed larger-runner preflight contract verifies expected 16-core runner eligibility semantics.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_runner_preflight"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby -e require 'yaml'; YAML.load_file('.github/workflows/ci.yaml')`
- `bash adl/tools/test_ci_runtime_contracts.sh`
- `bash adl/tools/test_ci_path_policy.sh`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate_runner_preflight`

## Failure Semantics

Fail closed on wrong-job routing, malformed workflow syntax, regression to ubuntu-latest, altered stable-status/producer/Spot/artifact/Codecov semantics, stale typed state, or missing exact-head review.

## Handoff

Retain typed evidence before convergence.
