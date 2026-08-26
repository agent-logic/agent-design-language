# Validation Planning Prompt

Template: 1.0.0

Issue: 415

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/415/design.md

Diagram: .csdlc/prepared/issues/415/diagram.mmd

## Selected Lanes

[
  {
    "lane": "builder-diagnostics-focused",
    "proof_role": "Prove labeled preflight success, missing-tool retained diagnostics, runner emission, cleanup compatibility, syntax, exact scope, and lifecycle non-mutation without AWS.",
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
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/test_run_aws_spot_builder_image_validation.sh"
    ],
    "parallel_group": "415-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/test_run_aws_spot_builder_image_validation.sh`

## Failure Semantics

Fail closed on any preflight failure, retain exact redacted diagnostics before exit, and preserve existing cleanup ownership without retrying or launching AWS.

## Handoff

Retain typed evidence before convergence.
