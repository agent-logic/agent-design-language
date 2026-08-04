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

- preparation only; do not execute the release ceremony
- do not publish, open a PR, tag, merge, or perform closeout
- do not touch tracked root main
- do not use /private/tmp for request or evidence artifacts
- do not touch #5357 remediation
- do not mutate any version:v0.92 issue
- write only issue-local #5348 C-SDLC paths plus their typed request/evidence files
- future execution stays blocked until #5359 is live-merged and its merge SHA is ancestral to the exact #5348 execution base
