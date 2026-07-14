# Validation Planning Prompt

Template: 1.0.0

Issue: 5335

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/designs/5335/design.md

Diagram: .csdlc/designs/5335/design.mmd

## Selected Lanes

[
  {
    "lane": "planning-docs",
    "proof_role": "Validate milestone and feature planning structure",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/validate_planning_templates.sh",
      "docs/milestones/v0.91.8"
    ],
    "parallel_group": "focused-docs",
    "defer_reason": null
  },
  {
    "lane": "issue-wave-yaml",
    "proof_role": "Parse and validate machine-useful issue-wave YAML",
    "acceptance_ids": [
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
      "require 'yaml'; YAML.load_file('docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml')"
    ],
    "parallel_group": "focused-docs",
    "defer_reason": null
  },
  {
    "lane": "review",
    "proof_role": "Bounded exact-revision review of planning truth and issue topology",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 8000,
    "argv": [
      "codex",
      "review",
      "--bounded",
      "docs/milestones/v0.91.8"
    ],
    "parallel_group": "review",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash adl/tools/validate_planning_templates.sh docs/milestones/v0.91.8`
- `ruby -e require 'yaml'; YAML.load_file('docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml')`
- `codex review --bounded docs/milestones/v0.91.8`

## Failure Semantics

Fail closed on lifecycle, collision, structure, routing, or review blockers; preserve partial artifacts and do not publish unsupported claims.

## Handoff

Retain typed evidence before convergence.
