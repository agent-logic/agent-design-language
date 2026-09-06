# Structured Intent Prompt

Template: 1.0.0

Issue: 515

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one bounded local-model shadow-execution path that cannot acquire authority.

## Required Outcome

One bounded local-model shadow-execution and comparison path that cannot acquire authority.

## Scope

- adl/src/provider/**
- docs/milestones/v0.92.1/evidence/provider/prov-b/**
- .csdlc/prepared/issues/515/**
- .csdlc/issues/515/**

## Authority

- Issue authority is agent-logic/agent-design-language#515
- The only hard execution dependency named by #515 is PROV-A/#514
- Shadow execution is non-authoritative and must not mutate authority state, lifecycle state, provider profiles, or production routing
- C-SDLC v2 remains the live lifecycle authority until explicit V3-F/#505 cutover

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Do not write to /private/tmp
- Do not use AWS, paid runners, or live cloud/provider calls without explicit authorization
- Do not claim production cutover or provider benchmark results
