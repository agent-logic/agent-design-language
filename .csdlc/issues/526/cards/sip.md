# Structured Intent Prompt

Template: 1.0.0

Issue: 526

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Perform one operator-authorized v0.92.1 release ceremony against an exact approved candidate and retain its readback receipt.

## Required Outcome

One release-ceremony receipt binds the exact ancestral candidate, release notes, tag, published release identity, and explicit operator authorization.

## Scope

- docs/milestones/v0.92.1/evidence/release/tail-10/**
- docs/milestones/v0.92.1/RELEASE_NOTES_v0.92.1.md
- Exact tag and release identity authorized by the operator
- Issue-local lifecycle and validation evidence for #526

## Authority

- The operator names and authorizes the exact release candidate and mutation
- All prior tail issues must have reviewed-green ancestral merges
- The ceremony receipt records release truth; later finish and cleanup are asynchronous bookkeeping

## Assumptions

- none

## Operator Constraints

- Do not tag or publish without explicit operator authorization
- Do not merge unreviewed work
- Do not begin until #525 has a reviewed merge and all tail merges are ancestral
- Do not make administrative closeout a release dependency
