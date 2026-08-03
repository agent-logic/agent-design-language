# Validation Planning Prompt

Template: 1.0.0

Issue: 5357

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5357/design.md

Diagram: .csdlc/prepared/issues/5357/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove six typed cards, reviewed design, exact review-document scope, current WP-17/WP-18 predecessor truth, schemas, budgets, no product changes, and typed doctor truth",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5357/validate-preparation.rb"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "wp18-terminal-gate",
    "proof_role": "Prove #5356 terminal receipt, reviewed head, merged PR commit, claim release, and ancestry without requiring a stale tracked projection",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5357/check-dependencies.rb"
    ],
    "parallel_group": "execution-gate",
    "defer_reason": null
  },
  {
    "lane": "corpus-dispatch-preflight",
    "proof_role": "Build and verify immutable exact-revision corpus and dispatch receipt while enforcing identity, independence, redaction, and non-self-inclusion",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5357/run-validation-lane.rb",
      "corpus-dispatch-preflight"
    ],
    "parallel_group": "review",
    "defer_reason": "Mandatory after #5356 terminal proof and exact corpus freeze; forbidden during preparation"
  },
  {
    "lane": "review-output-contract",
    "proof_role": "Validate findings-first severity order, exact evidence, evidence/inference/author-decision separation, residual risk, and typed synthesis mapping",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5357/run-validation-lane.rb",
      "review-output-contract"
    ],
    "parallel_group": "review",
    "defer_reason": "Mandatory after reviewer output exists; forbidden during preparation"
  },
  {
    "lane": "complete",
    "proof_role": "Run dependency, identity, corpus, dispatch, output, redaction, budget, no-deferral, exact-review, and publication preflight",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 9000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5357/run-validation-lane.rb",
      "complete"
    ],
    "parallel_group": "pre-publication",
    "defer_reason": "Mandatory at the exact reviewed result revision before publication"
  },
  {
    "lane": "post-merge-exact",
    "proof_role": "Re-run target ancestry, corpus/receipt/output digests, redaction, typed synthesis, CI and WP-20 release predicate after authorized merge",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 9000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5357/run-validation-lane.rb",
      "post-merge-exact"
    ],
    "parallel_group": "post-merge",
    "defer_reason": "Mandatory after authorized serialized merge and before typed closeout"
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/5357/validate-preparation.rb`
- `ruby .csdlc/prepared/issues/5357/check-dependencies.rb`
- `ruby .csdlc/prepared/issues/5357/run-validation-lane.rb corpus-dispatch-preflight`
- `ruby .csdlc/prepared/issues/5357/run-validation-lane.rb review-output-contract`
- `ruby .csdlc/prepared/issues/5357/run-validation-lane.rb complete`
- `ruby .csdlc/prepared/issues/5357/run-validation-lane.rb post-merge-exact`

## Failure Semantics

Fail closed without dispatch, finding acceptance, publication, merge, WP-20 release, or closeout on incomplete #5356 terminal truth, handoff mutation, claim collision, stale/non-ancestral target, mutable or incomplete corpus/receipt identity, undisclosed reviewer control, malformed output, evidence/inference confusion, secret or host-bound data, new dependency, budget breach, deferred gate, stale review, red CI, or absent post-merge proof.

## Handoff

Retain typed evidence before convergence.
