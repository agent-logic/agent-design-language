# Structured Intent Prompt

Template: 1.0.0

Issue: 519

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one exact-revision publication-candidate packet without merge or release mutation.

## Required Outcome

One redacted publication-candidate packet whose closing relationships and artifacts are bound to the exact revision reviewed by #518.

## Scope

- docs/milestones/v0.92.1/evidence/release/tail-03/**
- .csdlc/issues/519/**
- .csdlc/prepared/issues/519/**

## Authority

- Issue authority is agent-logic/agent-design-language#519
- Execution starts only after #518 has a reviewed merge
- The packet does not authorize merge, tag, release, or issue closure
- Typed finish and cleanup of #518 are non-gating

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Bind publication linkage to the exact reviewed candidate
- Redact credentials, private payloads, and machine-local paths
- Do not merge, tag, release, or close issues
