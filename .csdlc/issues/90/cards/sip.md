# Structured Intent Prompt

Template: 1.0.0

Issue: 90

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Add one typed, audited recovery operation that assigns the exact canonical code repository to legacy bound records that predate code_repository.

## Required Outcome

A clean, topology-verified bound, implemented, or reviewed legacy issue record with no code_repository can adopt its exact effective GitHub origin identity and then use the unchanged split-repository publication checks without direct state edits.

## Scope

- Typed code_repository migration request, report, and csdlc-issue command
- Exact origin, branch, worktree, phase, CAS, and cleanliness authorization
- Atomic canonical record and audit update
- Focused positive, negative, idempotency, and reviewed-publication regression proof
- Operator documentation for the supported recovery route

## Authority

- Issue #90 owns only migration of an absent code_repository field on an already bound legacy record
- The effective origin, registered branch/worktree topology, clean worktree, exact CAS, and existing review state remain authoritative
- csdlc-publish retains all repository identity, exact-head review, linkage, and freshness checks
- No arbitrary remote, issue repository, branch, worktree, phase, review, publication, or terminal mutation is authorized

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle routes and generated cards
- Never hand-edit canonical issue records or rendered cards
- Keep tracked work off main and use FastWork for builds
- Do not use AWS or raw gh
- Do not merge without explicit operator authorization
