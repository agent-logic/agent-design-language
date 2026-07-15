# Validation Planning Prompt

Template: 1.0.0

Issue: 5403

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: docs/reviews/v0.91.7/remaining-sprints-5403/DESIGN.md

Diagram: docs/reviews/v0.91.7/remaining-sprints-5403/DIAGRAM.mmd

## Selected Lanes

[
  {
    "lane": "review-doc-integrity",
    "proof_role": "Prove the review and register patch has no whitespace or conflict-marker defects",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
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
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "sprint-inventory-retention",
    "proof_role": "Confirm the retained sprint scope and evidence inventory is present and non-empty",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 100,
    "argv": [
      "/bin/test",
      "-s",
      "docs/reviews/v0.91.7/remaining-sprints-5403/SCOPE_EVIDENCE_INDEX.md"
    ],
    "parallel_group": "records",
    "defer_reason": null
  },
  {
    "lane": "canonical-register-retention",
    "proof_role": "Confirm the canonical v0.91.7 sprint review register is present and non-empty",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 100,
    "argv": [
      "/bin/test",
      "-s",
      "docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md"
    ],
    "parallel_group": "records",
    "defer_reason": null
  },
  {
    "lane": "independent-review-pass",
    "proof_role": "Require the refreshed independent review-quality evaluation to record an explicit passing decision",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 100,
    "argv": [
      "rg",
      "-q",
      "^Status: pass$",
      "docs/reviews/v0.91.7/remaining-sprints-5403/REFRESHED_REVIEW_QUALITY_EVALUATION.md"
    ],
    "parallel_group": "review",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `git diff --check`
- `/bin/test -s docs/reviews/v0.91.7/remaining-sprints-5403/SCOPE_EVIDENCE_INDEX.md`
- `/bin/test -s docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md`
- `rg -q ^Status: pass$ docs/reviews/v0.91.7/remaining-sprints-5403/REFRESHED_REVIEW_QUALITY_EVALUATION.md`

## Failure Semantics

Fail closed on missing source evidence, contradictory lifecycle truth, incomplete child coverage, or unreviewed actionable findings.

## Handoff

Retain typed evidence before convergence.
