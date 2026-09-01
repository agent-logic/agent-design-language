# Validation Planning Prompt

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/607/design.md

Diagram: .csdlc/prepared/issues/607/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue607-preparation-contract",
    "proof_role": "Validate the exact #607 design packet, all twelve acceptance criteria, plan coverage, separate state ownership, complete artifact closure, exact platform identity, timing denominators, performance budgets, three authorizations, and seven-day USD 20 envelope before binding.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11",
      "AC-12"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/607/validate-preparation.sh"
    ],
    "parallel_group": "readiness",
    "defer_reason": null
  },
  {
    "lane": "issue607-diff-hygiene",
    "proof_role": "Reject preparation and later implementation diff hygiene defects.",
    "acceptance_ids": [
      "AC-12"
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
    "parallel_group": "hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash .csdlc/prepared/issues/607/validate-preparation.sh`
- `git diff --check`

## Failure Semantics

Fail closed before AWS mutation on identity review artifact volume AZ cost timing or cleanup ambiguity; after apply always destroy disposable compute while preserving only exact authorized warm volumes; never convert a missed startup target into PASS.

## Handoff

Retain typed evidence before convergence.
