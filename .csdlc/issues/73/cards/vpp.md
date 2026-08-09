# Validation Planning Prompt

Template: 1.0.0

Issue: 73

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md

Diagram: .adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.mmd

## Selected Lanes

[
  {
    "lane": "planning-diff-hygiene",
    "proof_role": "Reject whitespace and patch hygiene errors in the planning packet.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "planning-structure",
    "proof_role": "Verify required architecture and issue-plan sections, links, and repository-relative source references.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "rg",
      "--line-number",
      "Implementation Issue Plan",
      ".adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "independent-model-review",
    "proof_role": "Verify the retained record contains findings-first Claude and Gemini reviews over the exact plan revision and their dispositions.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 1000,
    "argv": [
      "rg",
      "--line-number",
      "Claude Review|Gemini Review|Reviewed revision|Disposition",
      ".adl/docs/TBD/CSDLC_V3_RUST_PLAN_REVIEW.md"
    ],
    "parallel_group": "external-review",
    "defer_reason": "The retained review record is produced after both live provider reviews complete."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `git diff --check`
- `rg --line-number Implementation Issue Plan .adl/docs/TBD/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE.md`
- `rg --line-number Claude Review|Gemini Review|Reviewed revision|Disposition .adl/docs/TBD/CSDLC_V3_RUST_PLAN_REVIEW.md`

## Failure Semantics

Fail closed on missing issue-plan detail, stale or mismatched review revisions, undispositioned actionable findings, invalid source references, or any expansion into implementation.

## Handoff

Retain typed evidence before convergence.
