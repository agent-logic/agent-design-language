# Validation Planning Prompt

Template: 1.0.0

Issue: 307

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/307/design.md

Diagram: .csdlc/prepared/issues/307/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Validate exact coordination scope, current child inventory, #343 entry gate, exact #308-through-#319 sequence, and authored bundle.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/307/validate_preparation_bundle.py"
    ],
    "parallel_group": "307-serial-01-preparation",
    "defer_reason": null
  },
  {
    "lane": "child-sequence",
    "proof_role": "Fail closed unless the exact #308-through-#319 graph is retained and every predecessor satisfies the dependent issue's merge/readiness contract.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/307/validate_child_sequence.py",
      "--terminal"
    ],
    "parallel_group": "307-serial-02-sequence",
    "defer_reason": "Deferred until #343 terminal."
  },
  {
    "lane": "release-tail-reconciliation",
    "proof_role": "Validate included-child review, merge, async terminal/cleanup, finding, release, handoff, #268 successful closure, and #471 WP-27 child-remediation evidence at exact revisions.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/307/validate_child_sequence.py",
      "--terminal"
    ],
    "parallel_group": "307-serial-03-release",
    "defer_reason": "Deferred until approved child sequence completes."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed final coordination packet changes.",
    "acceptance_ids": [
      "AC-7"
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
    "parallel_group": "307-serial-04-diff",
    "defer_reason": "Deferred until final sprint packet exists."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 .csdlc/prepared/issues/307/validate_preparation_bundle.py`
- `python3 .csdlc/prepared/issues/307/validate_child_sequence.py --terminal`
- `python3 .csdlc/prepared/issues/307/validate_child_sequence.py --terminal`
- `git diff --check`

## Failure Semantics

Fail closed on child-graph drift, dependency, authority, review, check, ancestry, release, evidence, handoff, final closeout reconciliation, or #471 routing mismatch; never repair child work from #307.

## Handoff

Retain typed evidence before convergence.
