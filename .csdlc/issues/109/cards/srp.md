# Structured Review Prompt

Template: 1.0.0

Issue: 109

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope



## Prompts

- Review only the named immutable commit SHA in the named worktree; do not inherit or rely on the implementation conversation.
- Operate read-only: do not edit files, lifecycle state, PR state, or GitHub state.
- Report findings first, ordered P0 through P3, with repository-relative file and line evidence; include explicit limitations and state PASS only when no actionable findings remain.
- Check every acceptance criterion and identify any actionable finding that the implementation session must resolve.
- Apply authority-critical precedence: changes to authentication, authorization, security boundaries, lifecycle authority, or proof production require code, security, and evidence review even when the changed files are documentation.
- Verify the standard SRP remains the sole review-result authority and that any substantive fix requires a refreshed SRP and fresh-session review at the new exact head.
- Verify no daemon, scheduler, registry, claim, parallel review record, provider abstraction, lifecycle phase, or redundant broad validation was added.

## Findings

[
  {
    "id": "R109-P1-REVIEW-PROOF",
    "severity": "p1",
    "summary": "Focused validator could pass before any fresh-session review assignment, evidence, exact reviewed revision, or terminal finding dispositions existed.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "R109-P1-REUSABLE-SCOPE",
    "severity": "p1",
    "summary": "Reusable review skill omitted the mandatory runbook, authority-first classification, code/security/evidence coverage, acceptance review, explicit limitations, and P0-P3 ordering.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "R109-P1-ASSIGNMENT-ORDER",
    "severity": "p1",
    "summary": "The documented route did not require typed reviewer assignment before review activity began, permitting backfilled independence evidence.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "R109-P1-SUBSTANTIVE-BINDING",
    "severity": "p1",
    "summary": "The validator accepted an ancestor revision and did not verify the BLAKE3 portion of the exact substantive review revision.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "R109-P1-SRP-PARITY",
    "severity": "p1",
    "summary": "The validator did not compare structured SRP reviewer, revision, scope, findings, dispositions, and residual risks with retained review evidence.",
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

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
