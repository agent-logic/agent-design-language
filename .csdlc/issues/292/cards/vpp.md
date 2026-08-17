# Validation Planning Prompt

Template: 1.0.0

Issue: 292

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/292/design.md

Diagram: .csdlc/prepared/issues/292/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-edit-card-identity-target",
    "proof_role": "focused csdlc-edit card identity regression target",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "card_identity"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-validate-292",
    "proof_role": "lifecycle/card consistency after bind",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-292-implemented-card-identity-repair",
      "issue",
      "--issue",
      "292"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "required-hosted-pr-checks",
    "proof_role": "hosted required checks prove the published ready PR before merge is withheld",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 1500,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-shepherd",
      "observe",
      "--issue",
      "292"
    ],
    "parallel_group": "hosted",
    "defer_reason": "Runs after csdlc-publish creates the PR and before merge authority."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test card_identity`
- `.adl/bin/csdlc-v2/csdlc-validate --root /Volumes/FastWork/adl-worktrees/adl-issue-292-implemented-card-identity-repair issue --issue 292`
- `.adl/bin/csdlc-v2/csdlc-shepherd observe --issue 292`

## Failure Semantics

Fail closed on stale CAS, missing/mismatched live issue evidence, forbidden phase or existing review/publication/readiness/terminal truth, incompatible latest review-related audit state, sibling-scope title, malformed/colliding slug, validation failure, or missing fresh-session review.

## Handoff

Retain typed evidence before convergence.
