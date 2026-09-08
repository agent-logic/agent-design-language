# Validation Planning Prompt

Template: 1.0.0

Issue: 745

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/745/design.md

Diagram: .csdlc/prepared/issues/745/diagram.mmd

## Selected Lanes

[
  {
    "lane": "adr-docs",
    "proof_role": "Prove documentation and review/publication contract readiness locally; actual independent review, CI and terminal delivery remain separate gates.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/745/validate-adrs.py"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `python3 .csdlc/prepared/issues/745/validate-adrs.py`

## Failure Semantics

Fail closed on missing topics, broken links, unsupported authority claims, stale proof, review findings, or red CI.

## Handoff

Retain typed evidence before convergence.
