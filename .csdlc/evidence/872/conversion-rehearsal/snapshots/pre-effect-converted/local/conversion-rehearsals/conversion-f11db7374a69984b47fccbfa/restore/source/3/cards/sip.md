# Structured Intent Prompt

Template: 1.0.0

Issue: 3

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Harden and prove canonical code-repository PR publication against a separately identified preserved issue repository.

## Required Outcome

Typed publication and finish preserve exact code and issue repository identities, reject substitution, remain backward compatible, and retain a live split-authority canary.

## Scope

- C-SDLC v2 publication remote and PR reconciliation
- Split-authority finish and linkage proof
- Public schemas and typed operator documentation
- Existing canonical PR #5 to legacy issue #5901 canary evidence

## Authority

- Issue #3 owns only the declared C-SDLC v2 publication hardening and proof surfaces
- The preserved legacy repository is read-only except for GitHub's closing relation caused by the canonical PR
- Already-merged #5901 implementation is baseline evidence, not work to duplicate

## Assumptions

- none

## Operator Constraints

- Never write tracked issue changes on main
- Use only typed C-SDLC v2 lifecycle tools
- Do not use AWS or remote builders
- Do not merge without explicit operator authorization
