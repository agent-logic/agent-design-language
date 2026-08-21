# Validation Planning Prompt

Template: 1.0.0

Issue: 308

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/308/design.md

Diagram: .csdlc/prepared/issues/308/diagram.mmd

## Selected Lanes

[
  {
    "lane": "wp20-demo-proof-validator",
    "proof_role": "Prove the reconciled demo and AEE coverage surfaces agree at one exact revision",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "python3",
      "adl/tools/validate_v092_demo_proof_coverage.py"
    ],
    "parallel_group": "wp20-focused",
    "defer_reason": "The issue-owned validator is created during execution and must exist before the ready issue may advance beyond implementation."
  },
  {
    "lane": "wp20-demo-proof-negative-suite",
    "proof_role": "Prove missing, duplicate, synthetic, planned, unsupported-platform, and revision-drift rows fail closed",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_v092_demo_proof_coverage.sh"
    ],
    "parallel_group": "wp20-focused",
    "defer_reason": "The issue-owned focused harness is created during execution and must prove all declared rejection classes."
  },
  {
    "lane": "patch-structure",
    "proof_role": "Prove the bounded patch has no whitespace or conflict-marker defects",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "wp20-focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `python3 adl/tools/validate_v092_demo_proof_coverage.py`
- `bash adl/tools/test_v092_demo_proof_coverage.sh`
- `git diff --check`

## Failure Semantics

Fail closed on unmet predecessor gates, missing or contradictory evidence, duplicate ownership, synthetic proof, planned-as-passed status, unsupported platform claims, revision drift, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
