# Validation Planning Prompt

Template: 1.0.0

Issue: 5352

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving preparation validation DAG now; reserve execution-time lanes for the future handoff ledger.

## Lane Inputs

Design: .csdlc/prepared/issues/5352/design.md

Diagram: .csdlc/prepared/issues/5352/diagram.mmd

## Selected Lanes

[
  {
    "lane": "wp21-handoff-prep",
    "proof_role": "Validate issue-local preparation packet shape and dependency-gate language without implementation",
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
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5352/validate_preparation.rb"
    ],
    "parallel_group": "wp21-preparation",
    "defer_reason": null
  },
  {
    "lane": "dependency-ancestry",
    "proof_role": "Execution-time proof that #5384, #5358, and #5361 are live-closed and their accepted merges are ancestral to current origin/main",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "merge-base",
      "--is-ancestor",
      "<accepted-merge>",
      "origin/main"
    ],
    "parallel_group": "future-execution",
    "defer_reason": "Deferred because this session is preparation only; execution-time origin/main must be re-read."
  },
  {
    "lane": "handoff-ledger-docs",
    "proof_role": "Execution-time docs/link/schema validation for the future exact-revision handoff ledger",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 1800,
    "budget_tokens": 12000,
    "argv": [
      "future",
      "focused-docs-validation"
    ],
    "parallel_group": "future-execution",
    "defer_reason": "Deferred until the handoff ledger exists during execution."
  },
  {
    "lane": "pre-pr-review",
    "proof_role": "Bounded gpt-5.5/external review of the future handoff artifact before PR publication",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "review",
    "budget_seconds": 1800,
    "budget_tokens": 12000,
    "argv": [
      "future",
      "bounded-gpt-5.5-review"
    ],
    "parallel_group": "future-review",
    "defer_reason": "Deferred until execution produces the handoff ledger; preparation review is recorded separately."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1800

Tokens: 12000

## Commands

- `ruby .csdlc/prepared/issues/5352/validate_preparation.rb`
- future: `git merge-base --is-ancestor` for #5384/#5358/#5361 accepted merges against execution-time `origin/main`
- future: focused docs/link/schema validation for `docs/milestones/v0.91.8/handoff/issue-5352-v092-consumption-handoff.md`
- future: bounded gpt-5.5/external review before PR publication

## Failure Semantics

Fail closed on missing six-card packet, missing design/diagram/review artifacts, implementation-state advancement, active-claim requirement during preparation, live dependency ambiguity, receipt-gated execution language, missing exact revisions, missing rollback truth, or any v0.92 birthday/Adaptive Learning implementation claim.

## Handoff

Retain typed evidence before convergence.
