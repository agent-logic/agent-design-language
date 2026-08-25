# Validation Planning Prompt

Template: 1.0.0

Issue: 312

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/312/design.md

Diagram: .csdlc/prepared/issues/312/diagram.mmd

## Selected Lanes

[
  {
    "lane": "docs-release-truth",
    "proof_role": "Regenerate and validate the complete declared documentation inventory, merged producer bindings, ownership, claim status, release boundaries, and exact output packet.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/312/validate-doc-release-truth.rb",
      "packet"
    ],
    "parallel_group": "312-doc-shared-state-serial",
    "defer_reason": "Runs first in the serialized shared-state documentation lane."
  },
  {
    "lane": "docs-negative-suite",
    "proof_role": "Reject missing/duplicate inventory rows, stale or cross-repository authority, retired commands, broken links, unsupported claims, redaction failures, .adl dependencies, and out-of-scope paths through the production validator.",
    "acceptance_ids": [
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/312/test-validate-doc-release-truth.rb"
    ],
    "parallel_group": "312-doc-shared-state-serial",
    "defer_reason": "Runs after packet validation because it temporarily mutates issue-local fixtures and the README manifest."
  },
  {
    "lane": "docs-structure-links-handoff",
    "proof_role": "Parse every canonical Markdown, JSON, and YAML surface; validate relative links, exact changed paths, tracked no-.adl dependencies, the external-review manifest and handoff, redaction, and publication-time producer rescan.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/312/validate-doc-release-truth.rb",
      "structure-handoff"
    ],
    "parallel_group": "312-doc-shared-state-serial",
    "defer_reason": "Runs after the negative suite has restored shared issue-local state."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Validate whitespace and patch hygiene for the exact candidate diff.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "312-diff",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/312/validate-doc-release-truth.rb packet`
- `ruby .csdlc/prepared/issues/312/test-validate-doc-release-truth.rb`
- `ruby .csdlc/prepared/issues/312/validate-doc-release-truth.rb structure-handoff`
- `git diff --check`

## Failure Semantics

Fail closed when #311 is not merged into the execution base, its blocked gate truth is lost or misstated, merged producer overlap is not incorporated, a documentation surface is missing or duplicate, authority is stale or fabricated, a release claim is unsupported, a command is retired, a link is broken, redaction or .adl dependency fails, scope drifts, review is stale, or the candidate changes. Administrative terminal reconciliation, closeout, and cleanup are explicitly non-blocking.

## Handoff

Retain typed evidence before convergence.
