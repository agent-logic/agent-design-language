# Validation Planning Prompt

Template: 1.0.0

Issue: 22

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/22/design.md

Diagram: .csdlc/prepared/issues/22/diagram.mmd

## Selected Lanes

[
  {
    "lane": "builder-image-contract",
    "proof_role": "Verify pinned Ruby source, digest, provenance, and existing immutable builder tools",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/test_adl_builder_image.sh"
    ],
    "parallel_group": "focused-shell",
    "defer_reason": null
  },
  {
    "lane": "spot-builder-preflight-contract",
    "proof_role": "Verify Ruby smoke ordering and missing-Ruby fail-closed behavior",
    "acceptance_ids": [
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
      "bash",
      "adl/tools/test_run_aws_spot_builder_image_validation.sh"
    ],
    "parallel_group": "focused-shell",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace and unrelated changes",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 200,
    "argv": [
      "git",
      "diff",
      "--check"
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

- `bash adl/tools/test_adl_builder_image.sh`
- `bash adl/tools/test_run_aws_spot_builder_image_validation.sh`
- `git diff --check`

## Failure Semantics

Fail closed before the requested validation command when Ruby or validator smoke fails; do not substitute host installation.

## Handoff

Retain typed evidence before convergence.
