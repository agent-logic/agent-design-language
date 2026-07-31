# Structured Review Prompt

Template: 1.0.0

Issue: 5352

Repository: danielbaustin/agent-design-language

Card: srp

Status: ready

## Scope

Preparation review covers only the #5352 issue-local preparation packet, including cards, design, diagram, validator, dependency gates, and non-claim boundaries. Future pre-PR review must cover the exact-revision handoff artifact after execution writes it. Both scopes exclude implementation of birthday, Adaptive Learning, sibling WP-14 children, GitHub mutation, AWS, and closeout authority.

## Prompts

- Does the handoff gate require live merge plus ancestry for #5384, #5358, and #5361?
- Could any wording treat receipts as execution authority?
- Does the packet avoid birthday, Adaptive Learning, and v0.92 implementation claims?
- Is the later handoff ledger exact-revision and rollback-boundary oriented?
- Are intended paths, COTS/tool boundaries, LoC/time budgets, PVF lanes, rollback criteria, and no-deferral rules explicit?
- Is claim reacquisition deferred rather than required by preparation validation?

## Findings

[
  {
    "id": "PR-1",
    "severity": "P1",
    "summary": "Preparation validator required an active claim even though execution-time claim acquisition is deferred.",
    "disposition": "fixed in .csdlc/prepared/issues/5352/validate_preparation.rb"
  },
  {
    "id": "PR-2",
    "severity": "P2",
    "summary": "Existing cards and design retained stale WP-14/open-dependency wording and omitted exact source revision, path, COTS, budget, PVF, rollback, and no-deferral detail.",
    "disposition": "fixed across six cards, design.md, and diagram.mmd"
  }
]

## Dispositions

Every actionable finding requires a terminal disposition before the preparation branch is pushed.

## Residual Risk

- This environment does not expose a separate callable gpt-5.5 reviewer endpoint; the preparation review artifact records that limitation and does not claim an external model call.
- Future execution still needs a fresh bounded review of the actual handoff ledger before PR publication.

## Review Result

Revision: preparation-review-2026-07-31

Reviewer: codex-preparation-review; requested gpt-5.5 lane unavailable in this tool context

Result: changes_required_then_fixed
