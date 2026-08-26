# Validation Planning Prompt

Template: 1.0.0

Issue: 560

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/560/design.md

Diagram: .csdlc/prepared/issues/560/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-runtime-v2-unified-kernel-coverage",
    "proof_role": "Run the issue-owned focused coverage script for the three exact runtime_v2 unified-kernel tests under ci-coverage.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 7080,
    "budget_tokens": 1200,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/560/validate-focused-proof.sh"
    ],
    "parallel_group": "560-serial-01",
    "defer_reason": null
  },
  {
    "lane": "lifecycle-evidence-hygiene",
    "proof_role": "Run the issue-owned lifecycle evidence hygiene script before exact-head review and publication.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 400,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/560/validate-lifecycle-evidence.sh"
    ],
    "parallel_group": "560-serial-02",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash .csdlc/prepared/issues/560/validate-focused-proof.sh`
- `bash .csdlc/prepared/issues/560/validate-lifecycle-evidence.sh`

## Failure Semantics

Fail closed on product semantic changes, broad timeout masking, missing focused proof, missing exact-head API review, stale head, red required checks, or lifecycle topology drift.

## Handoff

Retain typed evidence before convergence.
