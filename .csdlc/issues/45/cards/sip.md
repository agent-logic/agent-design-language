# Structured Intent Prompt

Template: 1.0.0

Issue: 45

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make csdlc-doctor validate issue-tracker and code-publication repository authority separately without weakening drift detection.

## Required Outcome

Doctor accepts same-repository topology and the explicit supported split-authority topology, rejects accidental repository drift, and active operator guidance describes the contract accurately.

## Scope

- Typed C-SDLC v2 repository identity model used by doctor
- Doctor readiness and repository drift diagnostics
- Focused same-repository, valid split-repository, and invalid-drift tests
- Active C-SDLC v2 skills and runbooks that describe repository identity

## Authority

- Issue #45 changes only C-SDLC v2 doctor identity validation and directly coupled typed contracts
- The source issue repository and code/PR repository remain independently authoritative
- Remote names are evidence labels, not authority
- No repository migration or legacy issue mutation is part of this issue

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Use only typed C-SDLC v2 Rust lifecycle tools
- Do not infer split authority from an incidental remote mismatch
- Update all active skills and runbooks affected by the contract
- Treat time, token, and line-count estimates as reviewable rather than hard limits
