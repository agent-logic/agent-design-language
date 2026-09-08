# Structured Intent Prompt

Template: 1.0.0

Issue: 517

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one fail-closed quality-gate decision for the exact converged v0.92.1 candidate.

## Required Outcome

One exact-candidate quality-gate decision with every required proving lane passing and zero unowned exceptions.

## Scope

- docs/milestones/v0.92.1/evidence/release/tail-01/**
- docs/milestones/v0.92.1/QUALITY_GATE_v0.92.1.md
- .csdlc/issues/517/**
- .csdlc/prepared/issues/517/**

## Authority

- Issue authority is agent-logic/agent-design-language#517
- Execution starts only after #516 has a reviewed merge
- The issue decides the gate but does not remediate failed lanes
- Typed finish and cleanup of #516 are non-gating

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Reject absent, skipped, zero-test, stale, or non-proving evidence
- Do not publish, merge, tag, or release
