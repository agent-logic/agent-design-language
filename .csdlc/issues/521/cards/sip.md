# Structured Intent Prompt

Template: 1.0.0

Issue: 521

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one retained independent external review report for the unchanged final v0.92.1 candidate.

## Required Outcome

Reviewer independence, complete declared scope, exact revision, findings, and limitations validate without candidate drift.

## Scope

- docs/milestones/v0.92.1/evidence/release/tail-05/**
- .csdlc/issues/521/**
- .csdlc/prepared/issues/521/**

## Authority

- Issue #521 owns only the independent review report
- #520 supplies the internal register and unchanged candidate
- #522 owns finding remediation
- External review does not approve release

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Do not execute before #520 reviewed merge
- Use a reviewer independent of implementation and internal synthesis
- Do not mutate candidate or product code
