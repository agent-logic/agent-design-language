# Structured Intent Prompt

Template: 1.0.0

Issue: 5348

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Execute the canonical WP-23 release ceremony after WP-22 next-milestone review.

## Required Outcome

Release evidence, tag and publication truth, and reconciled issue, PR, card, milestone, and v0.92 handoff state.

## Scope

- release ceremony truth
- tag and publication evidence
- lifecycle closeout reconciliation

## Authority

- typed C-SDLC v2 only
- no execution before live predecessor merge plus ancestry
- receipts audit-only
- no raw gh, AWS, PR, merge, or closeout during preparation

## Assumptions

- none

## Operator Constraints

- execute only the v0.91.8 documentation and release ceremony
- use adl/tools/release_ceremony.sh for tag and GitHub release actions
- do not write tracked changes on root main
- do not use /private/tmp for request or evidence artifacts
- do not change product code or run broad Rust validation for this docs-only issue
- do not mutate or activate version:v0.92 issues
- obtain one exact-head review before publication
- close #5595 only after the v0.91.8 tag and published release are verified
