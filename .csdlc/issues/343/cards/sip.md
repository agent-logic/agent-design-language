# Structured Intent Prompt

Template: 1.0.0

Issue: 343

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Close the v0.92 demonstration sprint by reconciling terminal #256 and #341, validated historical WP-17/WP-19 evidence, real demo paths, release-truth boundaries, and one exact sprint review before handing off to #307/#308.

## Required Outcome

A reviewable, digest-bound sprint packet proves the canonical child outcomes and historical authorities without absorbing child implementation, optional streams, deferred Unity work, or the following release-tail sprint.

## Scope

- Current child terminal reconciliation for #256 and #341
- Read-only validation of historical WP-17 and WP-19 terminal evidence
- Real demo-path and retained-artifact reconciliation
- Release-truth, redaction, publication, and non-claim reconciliation
- One exact-head sprint review and handoff to #307/#308

## Authority

- #343 coordinates and reviews; it does not implement or repair child work
- #342, #340, and deferred Unity #84/#251 are outside the sprint denominator
- #307/#308 are handoff targets and are not executed by #343
- Child terminal caches, canonical validation, merge ancestry, and exact reviews remain their own authority

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 and keep preparation unbound until #256 and #341 are terminal
- Do not run AWS, provider, demo, release, or publication work from #343
- Do not treat #342 or deferred Unity #84/#251 as gates
- Fail closed on stale, noncanonical, nonancestral, fixture-only, or ambiguous evidence
