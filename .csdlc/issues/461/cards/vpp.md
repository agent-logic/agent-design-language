# Validation Planning Prompt

Template: 1.0.0

Issue: 461

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/461/design.md

Diagram: .csdlc/prepared/issues/461/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-tls-config-only-focused",
    "proof_role": "prove config-owned TLS through the executable Guardian lifecycle validator",
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
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/validate_v092_runtime_guardian_lifecycle.sh"
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

- `bash adl/tools/validate_v092_runtime_guardian_lifecycle.sh`

## Failure Semantics

Fail closed on missing, invalid, ambiguous, or command-supplied TLS authority and retain the exact causal test result.

## Handoff

Retain typed evidence before convergence.
