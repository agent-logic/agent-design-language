# Structured Intent Prompt

Template: 1.0.0

Issue: 522

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one complete disposition ledger for every accepted internal and external review finding.

## Required Outcome

Every accepted #520/#521 finding appears exactly once with evidence, owner, disposition, review identity, and release consequence.

## Scope

- docs/milestones/v0.92.1/evidence/release/tail-06/**
- .csdlc/issues/522/**
- .csdlc/prepared/issues/522/**

## Authority

- Issue #522 owns the finding census and disposition ledger
- Substantive fixes remain separately reviewable changes
- Deferrals require explicit owner, rationale, target milestone, and release consequence
- Release-blocking findings cannot be deferred silently

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Do not execute before #521 reviewed merge
- Do not collapse distinct findings or fixes
- Do not claim a fix from issue closure or green CI alone
