# Structured Intent Prompt

Template: 1.0.0

Issue: 525

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one independent findings-first review of the complete v0.92.2 planning package at an immutable revision.

## Required Outcome

One exact-revision planning review report with a complete review denominator and evidence-backed disposition for every finding.

## Scope

- Merged #523 and #524 planning outputs
- docs/milestones/v0.92.2/**
- docs/planning/ADL_FEATURE_LIST.md
- docs/milestones/v0.92.1/evidence/release/tail-09/**

## Authority

- The review is read-only against one immutable planning revision
- Findings do not authorize remediation inside this issue
- The review does not approve the current release or create successor issues

## Assumptions

- none

## Operator Constraints

- Do not edit reviewed planning files
- Do not begin until #524 has a reviewed merge
- Report findings before summary
- Do not convert green validators into semantic approval
