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
    "proof_role": "Measure the current canonical Runtime v3 source posture through its owner report",
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
    "lane": "baseline-json",
    "proof_role": "Parse the exact baseline and Runtime v3 parity manifests",
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
    "budget_tokens": 1000,
    "argv": [
      "jq",
      "empty",
      "docs/milestones/v0.91.8/baseline_and_ownership_v0.91.8.json",
      "docs/milestones/v0.91.8/runtime_v3_functional_parity_plan_v0.91.8.json"
    ],
    "parallel_group": "local-docs",
    "defer_reason": null
  },
  {
    "lane": "issue-wave-yaml",
    "proof_role": "Parse the canonical v0.91.8 issue graph used by the architecture plan",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      "-e",
      "require 'yaml'; YAML.safe_load(File.read('docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml'), aliases: true)"
    ],
    "parallel_group": "local-docs",
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
- `jq empty docs/milestones/v0.91.8/baseline_and_ownership_v0.91.8.json docs/milestones/v0.91.8/runtime_v3_functional_parity_plan_v0.91.8.json`
- `ruby -e require 'yaml'; YAML.safe_load(File.read('docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml'), aliases: true)`
- `git diff --check origin/main...HEAD`

## Failure Semantics

Fail closed on missing feature disposition, fixture-as-live overclaim, dependency cycles, budget ambiguity, invalid planning artifacts, or AWS reliance.

## Handoff

Retain typed evidence before convergence.
