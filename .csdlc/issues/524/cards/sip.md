# Structured Intent Prompt

Template: 1.0.0

Issue: 524

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Define one executable, reviewable v0.92.2 closeout and release-tail contract.

## Required Outcome

The v0.92.2 release plan and milestone checklist agree on denominator, exact tail order, operator gates, and non-gating asynchronous bookkeeping.

## Scope

- docs/milestones/v0.92.2/RELEASE_PLAN_v0.92.2.md
- docs/milestones/v0.92.2/MILESTONE_CHECKLIST_v0.92.2.md
- Issue-local lifecycle and validation evidence for #524

## Authority

- The reviewed #523 planning merge supplies the successor denominator
- Release gates depend on reviewed merges, not asynchronous finish or cleanup receipts
- Operator-only actions remain explicitly operator-gated

## Assumptions

- none

## Operator Constraints

- Do not execute the v0.92.2 tail
- Do not perform current release actions
- Do not make closeout bookkeeping a dependency
- Do not begin until #523 has a reviewed merge
