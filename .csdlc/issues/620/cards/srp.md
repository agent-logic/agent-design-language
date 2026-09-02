# Structured Review Prompt

Template: 1.0.0

Issue: 620

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

docs/milestones/v0.92.2/**
.csdlc/prepared/issues/620/design.md
.csdlc/prepared/issues/620/validate-v0922-first-pass-planning.sh

## Prompts

- Does the package include every canonical document and first-class feature surface?
- Does each planned issue describe one bounded unit with a concrete result?
- Is every relevant TBD source explicitly scheduled, deferred, preserved as reference or provenance, or flagged for operator decision?
- Does the plan avoid duplicating existing and completed issues?
- Does the package remain number-free and refrain from opening the milestone?

## Findings

[
  {
    "id": "620-R5",
    "severity": "p1",
    "summary": "The #484/OPS-AWS reconciliation guard uses a literal pipe and makes scheduling validation fail without diagnostics.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Live PR mergeability and CI remain publication-time checks.

## Review Result

Revision: Some("git-blake3:f51c25c52df05a98209ef65a3b732a01ae501f99:47db3bf536685aa0f264a1f9bb4f194e3a91b034dc52407313759fa8e1b25d63")

Reviewer: Some("fresh-session:620-docs-f51c25c52")

Result: changes_required
