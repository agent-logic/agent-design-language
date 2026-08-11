# Validation Planning Prompt

Template: 1.0.0

Issue: 234

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/234/design.md

Diagram: .csdlc/prepared/issues/234/diagram.mmd

## Selected Lanes

[
  {
    "lane": "ci-runtime-contracts",
    "proof_role": "Verify central CI job gates, required heavy-runner selection, coverage aggregation gating, slow-proof isolation, and post-merge suppression without hosted dispatch.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "parallel_group": "local-policy",
    "defer_reason": null
  },
  {
    "lane": "whole-workflow-policy",
    "proof_role": "Scan every workflow and reject unauthorized automatic triggers, schedules, missing explicit dispatch, SHA-concurrency regressions, and optional runner allocation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      "adl/tools/validate_ci_workflow_policy.rb"
    ],
    "parallel_group": "local-policy",
    "defer_reason": null
  },
  {
    "lane": "csdlc-required-check-classifier",
    "proof_role": "Prove unstable mergeability ignores canceled optional checks only when every declared required check and required review passes, while conflicts, draft state, missing or failed required checks, and exact-head drift fail closed in GitHub observation and finish preflight.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib"
    ],
    "parallel_group": "local-policy",
    "defer_reason": null
  },
  {
    "lane": "issue-diff-hygiene",
    "proof_role": "Reject malformed whitespace and patch artifacts before exact-head review.",
    "acceptance_ids": [
      "AC-9"
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
    "parallel_group": "local-policy",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash adl/tools/test_ci_runtime_contracts.sh`
- `ruby adl/tools/validate_ci_workflow_policy.rb`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --lib`
- `git diff --check`

## Failure Semantics

Fail closed on unauthorized automatic workflow triggers, required-check loss, heavy-runner bypass, duplicate-head execution, optional fanout, soak leakage, malformed workflow YAML, or unresolved exact-head findings.

## Handoff

Retain typed evidence before convergence.
