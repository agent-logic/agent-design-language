# Structured Review Prompt

Template: 1.0.0

Issue: 114

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

Exact #114 changed paths, merged #111/#112 contract boundaries, storage/migration/recovery invariants, all PVF evidence, public projections, and no #83 or sibling mutation.

## Prompts

- Verify every history read and lifecycle action is reauthorized through current #112 truth and no cache, cursor, search index, or export bypass exists.
- Verify storage transactions, idempotency, ordering, outcome monotonicity, receipt chains, migration publication, and restart recovery cannot claim false continuity.
- Verify retention, tombstone deletion, compaction, export, search, and browser cache handling expose no forbidden or unauthorized data.
- Verify exact path ownership, serial gates, forty-two-case denominator, nonzero PVF selection, and no #83 mutation.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
