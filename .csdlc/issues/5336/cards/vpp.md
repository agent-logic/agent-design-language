# Validation Planning Prompt

Template: 1.0.0

Issue: 5336

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5336/design.md

Diagram: .csdlc/prepared/issues/5336/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-owner-budget",
    "proof_role": "Measure and enforce the canonical Runtime v3 source and test budget posture",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/report_runtime_v3_loc.sh"
    ],
    "parallel_group": "local-docs",
    "defer_reason": null
  },
  {
    "lane": "runtime-architecture-semantics",
    "proof_role": "Prove JSON and YAML schemas, canonical ownership, exact budgets, ten parity groups, four lanes, feature dispositions, non-authorization, dependency ordering, and graph acyclicity",
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
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5336/validate_architecture_plan.rb"
    ],
    "parallel_group": "local-docs",
    "defer_reason": null
  },
  {
    "lane": "milestone-local-links",
    "proof_role": "Prove every local Markdown link in the v0.91.8 milestone planning package resolves",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5336/validate_links.rb"
    ],
    "parallel_group": "local-docs",
    "defer_reason": null
  },
  {
    "lane": "coverage-orchestration-contract",
    "proof_role": "Prove one bounded instrumented prebuild per profile, fail-closed profile cleanup, preserved causal status, and unchanged two-partition single-report execution",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/test_run_authoritative_coverage_lane.sh"
    ],
    "parallel_group": "local-control",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Check the complete issue branch for patch hygiene",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "parallel_group": "local-control",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash adl/tools/report_runtime_v3_loc.sh`
- `ruby .csdlc/prepared/issues/5336/validate_architecture_plan.rb`
- `ruby .csdlc/prepared/issues/5336/validate_links.rb`
- `bash adl/tools/test_run_authoritative_coverage_lane.sh`
- `git diff --check origin/main...HEAD`

## Failure Semantics

Fail closed on missing feature disposition, fixture-as-live overclaim, dependency cycles, budget ambiguity, invalid planning artifacts, or AWS reliance.

## Handoff

Retain typed evidence before convergence.
