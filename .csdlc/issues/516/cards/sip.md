# Structured Intent Prompt

Template: 1.0.0

Issue: 516

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one immutable release-tail admission decision that includes a complete expected-versus-observed gap analysis for v0.92.1.

## Required Outcome

Every planned issue and retained dependency is reconciled against merged ancestry, production implementation, validation, review, documentation, integration, and closeout evidence; unresolved P0/P1 or unowned material gaps block admission.

## Scope

- docs/milestones/v0.92.1/evidence/integration/**
- docs/milestones/v0.92.1/DEMO_MATRIX_v0.92.1.md
- .csdlc/prepared/issues/516/**
- .csdlc/issues/516/**

## Authority

- #516 decides release-tail admission but does not approve the release
- Canonical issue acceptance and execution specifications define the expected denominator
- Merged code and retained evidence define observed truth
- Missing evidence is a gap or ambiguity, never implicit success
- Child remediation remains owned by the applicable issue

## Assumptions

- none

## Operator Constraints

- Do not implement child fixes
- Do not create duplicate remediation issues when an owner exists
- Do not admit test-only, placeholder, unused, or do-nothing implementations
- Do not mutate cloud or runtime infrastructure
- Do not approve, merge, finish, or close the milestone
